use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use std::collections::VecDeque;

use crate::plugins::map::resources::{Map, TileType};
use crate::util::ivec2_ext::*;
use crate::util::urect_ext::URectExt;
use crate::util::uvec2_ext::UVec2Ext;

pub fn apply_room_to_map(room: &URect, map: &mut Map) {
    fill(room, TileType::Floor, map);
}

pub fn fill(room: &URect, tile: TileType, map: &mut Map) {
    for p in room.point_set() {
        map[&p] = tile;
    }
}

pub fn apply_horizontal_tunnel(map: &mut Map, x1: u32, x2: u32, y: u32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let position = [x, y].into();
        if map.contains(&position) {
            map[&position] = TileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(map: &mut Map, y1: u32, y2: u32, x: u32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let position = [x, y].into();
        if map.contains(&position) {
            map[&position] = TileType::Floor;
        }
    }
}

pub fn connect_positions<RNG: Rng>(a: &UVec2, b: &UVec2, map: &mut Map, rng: &mut RNG) {
    if rng.gen_range(0..=1) == 1 {
        apply_horizontal_tunnel(map, a.x, b.x, a.y);
        apply_vertical_tunnel(map, a.y, b.y, b.x);
    } else {
        apply_vertical_tunnel(map, a.y, b.y, a.x);
        apply_horizontal_tunnel(map, a.x, b.x, b.y);
    }
}

/*
pub fn connect_rooms<RNG: Rng>(a: &URect, b: &URect, map: &mut Map, rng: &mut RNG) {
    connect_positions(&a.center(), &b.center(), map, rng);
}
*/

fn position_if_in_map(position: IVec2, map: &Map) -> Option<IVec2> {
    if position.x < 0 || position.y < 0 {
        return None;
    }
    if map.contains(&position.as_uvec2()) {
        Some(position)
    } else {
        None
    }
}

pub fn walk(from: UVec2, step: IVec2, map: &Map) -> impl Iterator<Item = UVec2> + '_ {
    itertools::unfold(from.as_ivec2(), move |pos| {
        *pos += step;
        position_if_in_map(*pos, map)
    })
    .map(|pos| pos.as_uvec2())
}

/// Cast a ray `from` a position in a `direction`, returning the position of the first `look_for`
/// tile that is found, if any
pub fn raycast(from: UVec2, step: IVec2, look_for: TileType, map: &Map) -> Option<UVec2> {
    walk(from, step, map).find(|pos| map[pos] == look_for)
}

/// Cast `n` rays in `ray_direction`, starting at `from`, walking in `walk_direction`,
/// returning all positions where a `look_for` tile was found.
pub fn raycast_walk(
    from: UVec2,
    ray_step: IVec2,
    walk_step: IVec2,
    n: usize,
    look_for: TileType,
    map: &Map,
) -> Vec<UVec2> {
    walk(from, walk_step, map)
        .take(n)
        .flat_map(|ray_from| raycast(ray_from, ray_step, look_for, map))
        .collect()
}

/// Return a mapping from "which side is this" to "positions along the hull"
pub fn raycast_hull(bbox: URect, look_for: TileType, map: &Map) -> HashMap<IVec2, Vec<UVec2>> {
    let mut retval = HashMap::new();

    let n = |step: IVec2| match step {
        NORTH | SOUTH => bbox.tile_height() as usize,
        EAST | WEST => bbox.tile_width() as usize,
        _ => unimplemented!(),
    };

    let mut record_raycast_walk = |from: UVec2, walk_step: IVec2| {
        retval.insert(
            walk_step.perp(),
            raycast_walk(
                from,
                walk_step.perp_cw(),
                walk_step,
                n(walk_step),
                look_for,
                map,
            ),
        );
    };

    // Raycast inward, walking along each edge of `bbox`
    record_raycast_walk(bbox.north_west(), EAST);
    record_raycast_walk(bbox.north_east(), SOUTH);
    record_raycast_walk(bbox.south_east(), WEST);
    record_raycast_walk(bbox.south_west(), NORTH);

    // Restrict matches to bbox
    for values in retval.values_mut() {
        values.retain(|position| bbox.contains(*position))
    }

    retval
}

/// Add exactly one corridor to connect the TileSet::Floor subsets of `a` and `b`.
/// Guarantees a fully connected area IFF both `a` and `b` are themselves internally
/// fully connected.
pub fn connect_regions<RNG: Rng>(a: URect, b: URect, map: &mut Map, rng: &mut RNG) {
    let a_hull = raycast_hull(a, TileType::Floor, map);
    let b_hull = raycast_hull(b, TileType::Floor, map);

    // Direct vertical?
    {
        let (north_hull, south_hull) = if a.max.y <= b.min.y {
            (&a_hull, &b_hull)
        } else {
            (&b_hull, &a_hull)
        };
        let north_x = north_hull[&SOUTH]
            .iter()
            .map(|p| p.x)
            .collect::<HashSet<_>>();
        let south_x = south_hull[&NORTH]
            .iter()
            .map(|p| p.x)
            .collect::<HashSet<_>>();
        let facing_x = north_x.intersection(&south_x).collect_vec();
        if !facing_x.is_empty() {
            let &&x = facing_x.choose(rng).unwrap();
            let north_position = north_hull[&SOUTH].iter().find(|p| p.x == x).unwrap();
            let south_position = south_hull[&NORTH].iter().find(|p| p.x == x).unwrap();
            connect_positions(north_position, south_position, map, rng);
            return;
        }
    }

    // Direct horizontal?
    {
        let (west_hull, east_hull) = if a.max.x <= b.min.x {
            (&a_hull, &b_hull)
        } else {
            (&b_hull, &a_hull)
        };
        let west_y = west_hull[&EAST].iter().map(|p| p.y).collect::<HashSet<_>>();
        let east_y = east_hull[&WEST].iter().map(|p| p.y).collect::<HashSet<_>>();
        let facing_y = west_y.intersection(&east_y).collect_vec();
        if !facing_y.is_empty() {
            let &&y = facing_y.choose(rng).unwrap();
            let west_position = west_hull[&EAST].iter().find(|p| p.y == y).unwrap();
            let east_position = east_hull[&WEST].iter().find(|p| p.y == y).unwrap();
            connect_positions(west_position, east_position, map, rng);
            return;
        }
    }

    // No direct vertical / horizontal corridor is possible, do a corner
    {
        let mut a_faces = vec![];
        let mut b_faces = vec![];

        // Figure out valid candidates for which sides we'll connect
        if a.max.x <= b.min.x {
            a_faces.push(EAST);
            b_faces.push(WEST);
        } else {
            a_faces.push(WEST);
            b_faces.push(EAST);
        }
        if a.max.y <= b.min.y {
            a_faces.push(SOUTH);
            b_faces.push(NORTH);
        } else {
            a_faces.push(NORTH);
            b_faces.push(SOUTH);
        }

        // Randomly determine which way the corner goes
        let (a_face, b_face) = {
            if rng.gen_range(1..=2) == 1 {
                (a_faces[0], b_faces[1])
            } else {
                (a_faces[1], b_faces[0])
            }
        };

        // Pick two random points along those faces
        let a_position = a_hull[&a_face].choose(rng).unwrap();
        let b_position = b_hull[&b_face].choose(rng).unwrap();

        // And finally, do the thing!
        connect_positions(a_position, b_position, map, rng);
    }
}

pub fn walls_around(rect: &URect, map: &mut Map) {
    for x in rect.min.x..=rect.max.x {
        map[(x, rect.min.y)] = TileType::Wall;
        map[(x, rect.max.y)] = TileType::Wall;
    }
    for y in rect.min.y..=rect.max.y {
        map[(rect.min.x, y)] = TileType::Wall;
        map[(rect.max.x, y)] = TileType::Wall;
    }
}

#[allow(dead_code)]
pub fn connected_region(seed: &UVec2, map: &Map) -> HashSet<UVec2> {
    let target = map[seed];
    let mut retval = HashSet::new();
    retval.insert(*seed);

    let mut queue = VecDeque::new();
    queue.push_back(*seed);

    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let p_next = p.offset(&[dx, dy].into());
                if map.get(&p_next) == Some(target) && !retval.contains(&p_next) {
                    retval.insert(p_next);
                    queue.push_back(p_next);
                }
            }
        }
    }

    retval
}

/*
pub fn random_position_with_tile<RNG: Rng>(target: TileType, map: &Map, rng: &mut RNG) -> UVec2 {
    loop {
        let p = (rng.gen_range(0..map.size.x), rng.gen_range(0..map.size.y));
        if map[p] == target {
            break p.into();
        }
    }
}
*/

/*
pub fn remove_unreachable_areas(start: &UVec2, map: &mut Map) -> HashSet<UVec2> {
    let connected_region = connected_region(start, map);
    for position in map.rect().point_set() {
        if map[&position] != TileType::Wall && !connected_region.contains(&position) {
            map[&position] = TileType::Wall;
        }
    }
    connected_region
}
*/

/*
pub fn generate_voronoi_spawn_regions(
    map: &Map,
    rng: &mut EntropyComponent<WyRand>,
) -> HashMap<u32, Vec<UVec2>> {
    let mut areas: HashMap<u32, Vec<UVec2>> = HashMap::new();
    let mut noise = bracket_noise::prelude::FastNoise::seeded(rng.next_u64());
    noise.set_noise_type(bracket_noise::prelude::NoiseType::Cellular);
    noise.set_frequency(0.08);
    noise.set_cellular_distance_function(
        bracket_noise::prelude::CellularDistanceFunction::Manhattan,
    );

    for position in map.rect().point_set() {
        if map[&position] == TileType::Floor {
            let cell_value_f = noise.get_noise(position.x as f32, position.y as f32) * 10240.0;
            let cell_value = cell_value_f as u32;
            areas
                .entry(cell_value)
                .and_modify(|ps| ps.push(position))
                .or_insert_with(|| vec![position]);
        }
    }

    areas
}
*/

/*
pub fn find_furthest_reachable_tiles(
    map: &Map,
    dijsktra_map: &bracket_pathfinding::prelude::DijkstraMap,
    count: usize,
) -> Vec<UVec2> {
    dijsktra_map
        .map
        .iter()
        .enumerate()
        .sorted_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal))
        .map(|(idx, _distance)| map.idx_pos(idx))
        .take(count)
        .collect()
}
*/

#[cfg(test)]
mod tests {
    use crate::util::urect_ext::urect_with_size;

    use super::*;

    fn make_map() -> Map {
        /*
         01234567890123456789
        9#...................
        8#...................
        7#...................
        6####################
        5####################
        4####################
        3####################
        2####################
        1#####.......###....#
        0#####.......###....#
        9#####.......###....#
        8#####.......###....#
        7#####.......###....#
        6#####..............#
        5#####.......###....#
        4###############....#
        3###############....#
        2####################
        1####################
        0####################
        */
        let mut map = Map::new([20, 20]);
        apply_room_to_map(&URect::new(5, 5, 11, 11), &mut map);
        apply_room_to_map(&URect::new(15, 3, 18, 11), &mut map);
        apply_room_to_map(&URect::new(2, 17, 19, 19), &mut map);
        apply_horizontal_tunnel(&mut map, 10, 15, 6);
        map
    }

    #[test]
    fn test_make_map() {
        let map = make_map();
        assert_eq!(map[(10, 5)], TileType::Floor);
        assert_eq!(map[(10, 4)], TileType::Wall);
        assert_eq!(map[(12, 5)], TileType::Wall);
    }

    #[test]
    fn test_raycast() {
        let map = make_map();
        assert_eq!(
            raycast(UVec2::new(6, 0), NORTH, TileType::Floor, &map),
            Some(UVec2::new(6, 5))
        );
        assert_eq!(
            raycast(UVec2::new(19, 7), WEST, TileType::Floor, &map),
            Some(UVec2::new(18, 7))
        );
        assert_eq!(
            raycast(UVec2::new(0, 0), NORTH, TileType::Floor, &map),
            None
        );
    }

    #[test]
    fn test_raycast_hull() {
        let map = make_map();
        let hull = raycast_hull(URect::new(0, 0, 19, 12), TileType::Floor, &map);

        assert_eq!(
            hull[&NORTH],
            vec!(
                // Room 1
                UVec2::new(5, 11),
                UVec2::new(6, 11),
                UVec2::new(7, 11),
                UVec2::new(8, 11),
                UVec2::new(9, 11),
                UVec2::new(10, 11),
                UVec2::new(11, 11),
                // Corridor
                UVec2::new(12, 6),
                UVec2::new(13, 6),
                UVec2::new(14, 6),
                // Room 2
                UVec2::new(15, 11),
                UVec2::new(16, 11),
                UVec2::new(17, 11),
                UVec2::new(18, 11),
            )
        );
        assert_eq!(
            hull[&EAST],
            vec!(
                UVec2::new(18, 11),
                UVec2::new(18, 10),
                UVec2::new(18, 9),
                UVec2::new(18, 8),
                UVec2::new(18, 7),
                UVec2::new(18, 6),
                UVec2::new(18, 5),
                UVec2::new(18, 4),
                UVec2::new(18, 3)
            )
        );
        assert_eq!(
            hull[&SOUTH],
            vec!(
                // Room 1
                UVec2::new(18, 3),
                UVec2::new(17, 3),
                UVec2::new(16, 3),
                UVec2::new(15, 3),
                // Corridor
                UVec2::new(14, 6),
                UVec2::new(13, 6),
                UVec2::new(12, 6),
                UVec2::new(11, 5),
                UVec2::new(10, 5),
                // Room 2
                UVec2::new(9, 5),
                UVec2::new(8, 5),
                UVec2::new(7, 5),
                UVec2::new(6, 5),
                UVec2::new(5, 5)
            )
        );
        assert_eq!(
            hull[&WEST],
            vec!(
                UVec2::new(15, 3),
                UVec2::new(15, 4),
                UVec2::new(5, 5),
                UVec2::new(5, 6),
                UVec2::new(5, 7),
                UVec2::new(5, 8),
                UVec2::new(5, 9),
                UVec2::new(5, 10),
                UVec2::new(5, 11)
            )
        );
    }

    #[test]
    fn test_connect_regions_direct_vertical() {
        let mut map = make_map();
        connect_regions(
            URect::new(0, 17, 19, 19),
            URect::new(0, 0, 19, 13),
            &mut map,
            &mut wyrand::WyRand::new(0),
        );

        let new_hull = raycast_hull(urect_with_size(0, 0, 20, 20), TileType::Floor, &map);
        assert!(new_hull[&EAST]
            .iter()
            .filter(|p| p.y >= 12 && p.y <= 16)
            .all(|p| p.x >= 5 && p.x <= 18));
    }

    #[test]
    fn test_connected_regions() {
        let map = make_map();
        assert_eq!(connected_region(&UVec2::new(7, 7), &map).len(), 88);
    }
}
