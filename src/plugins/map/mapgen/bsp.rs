use bevy::ecs::event::EventWriter;
use bevy::math::{URect, UVec2};
use petgraph::Graph;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::cmp::{max, min};
use std::collections::VecDeque;

use crate::plugins::map::resources::{Map, TileType};
use crate::plugins::spawner::events::Spawn;
use crate::util::urect_ext::{urect_with_size, URectExt};

use super::{common::*, MapBuilder, SnapshotManager};

pub struct BspConfig {
    /// 0..=1; higher values lead to more varied room aspect ratios
    pub subdivision_variance: f64,
    pub depth: u32,

    pub min_room_width: u32,
    pub max_room_width: u32,

    pub min_room_height: u32,
    pub max_room_height: u32,

    /// Minimum number of wall tiles between the edge of the region and the start of the room.
    /// Value is *per side*.
    pub min_padding: u32,
    /// Maximum number of wall tiles between the edge of the region and the start of the room.
    /// Value is *per side*.
    pub max_padding: u32,
}

impl BspConfig {
    pub fn dungeon() -> Self {
        BspConfig {
            subdivision_variance: 0.2,
            depth: 6,
            min_room_width: 6,
            max_room_width: 12,
            min_room_height: 6,
            max_room_height: 12,
            max_padding: 9000, // Arbitrarily large number. Not maxint because that leads to overflow.
            min_padding: 2,
        }
    }

    pub fn interior() -> Self {
        BspConfig {
            subdivision_variance: 0.2,
            depth: 5,
            min_room_width: 6,
            max_room_width: 9000,
            min_room_height: 6,
            max_room_height: 9000,
            max_padding: 0,
            min_padding: 0,
        }
    }

    fn min_region_width(&self) -> u32 {
        self.min_room_width + self.min_padding * 2
    }

    fn min_region_height(&self) -> u32 {
        self.min_room_height + self.min_padding * 2
    }

    fn subdivision_min(&self) -> f64 {
        0.5 - self.subdivision_variance / 2.0
    }

    fn subdivision_max(&self) -> f64 {
        0.5 + self.subdivision_variance / 2.0
    }
}

pub struct BspMapBuilder {
    config: BspConfig,
    rooms: Vec<URect>,
    map: Map,
    snapshot_manager: SnapshotManager,
}

impl<RNG> MapBuilder<RNG> for BspMapBuilder
where
    RNG: Rng,
{
    fn build_map(&mut self, rng: &mut RNG) {
        let mut graph = Graph::<URect, ()>::new();
        let root = graph.add_node(self.map.rect());
        let mut leaves = vec![root];

        // Generate space partition
        let mut max_depth = 0;
        for depth in 1..self.config.depth + 1 {
            leaves = leaves
                .iter()
                .flat_map(|&leaf| {
                    let leaf_rect = graph.node_weight(leaf).unwrap();
                    let mut a_rect;
                    let mut b_rect;

                    a_rect = *leaf_rect;
                    b_rect = *leaf_rect;
                    let position = rng
                        .gen_range(self.config.subdivision_min()..=self.config.subdivision_max());
                    if rng.gen_range(1..=2) == 1 {
                        a_rect.max.x -= (a_rect.tile_width() as f64 * position).round() as u32;
                        b_rect.min.x = a_rect.max.x;
                    } else {
                        a_rect.max.y -= (a_rect.tile_height() as f64 * position).round() as u32;
                        b_rect.min.y = a_rect.max.y;
                    }

                    if a_rect.tile_width() < self.config.min_region_width()
                        || b_rect.tile_width() < self.config.min_region_width()
                        || a_rect.tile_height() < self.config.min_region_height()
                        || b_rect.tile_height() < self.config.min_region_height()
                    {
                        vec![leaf]
                    } else {
                        let a = graph.add_node(a_rect);
                        let b = graph.add_node(b_rect);
                        graph.add_edge(leaf, a, ());
                        graph.add_edge(leaf, b, ());
                        max_depth = max(max_depth, depth);
                        vec![a, b]
                    }
                })
                .collect();

            if cfg!(feature = "visualize-mapgen") {
                self.take_subdivision_snapshot(
                    leaves
                        .iter()
                        .map(|index| *graph.node_weight(*index).unwrap())
                        .collect(),
                );
            }
        }

        // Create room in each partition
        for leaf in leaves {
            let partition = graph.node_weight(leaf).unwrap();

            // Generate random room width based on config
            let min_width = max(
                self.config.min_room_width,
                partition
                    .tile_width()
                    .saturating_sub(self.config.max_padding * 2),
            );
            let max_width = min(
                partition
                    .tile_width()
                    .saturating_sub(self.config.min_padding * 2),
                self.config.max_room_width,
            );
            let width = rng.gen_range(min_width..=max_width);

            // Generate random room left-edge, based on config and width
            let min_x1 = partition.min.x + self.config.min_padding;
            let max_x1 = partition
                .max
                .x
                .saturating_sub(width - 1 + self.config.min_padding);
            let x1 = rng.gen_range(min_x1..=max_x1);

            // Generate random room height based on config
            let min_height = max(
                self.config.min_room_height,
                partition
                    .tile_height()
                    .saturating_sub(self.config.max_padding * 2),
            );
            let max_height = min(
                partition
                    .tile_height()
                    .saturating_sub(self.config.min_padding * 2),
                self.config.max_room_height,
            );
            let height = rng.gen_range(min_height..=max_height);

            // Generate random room top-edge, based on config and height
            let min_y1 = partition.min.y + self.config.min_padding;
            let max_y1 = partition
                .max
                .y
                .saturating_sub(height - 1 + self.config.min_padding);
            let y1 = rng.gen_range(min_y1..=max_y1);

            let room = urect_with_size(x1, y1, width, height);
            apply_room_to_map(&room, &mut self.map);
            walls_around(&room, &mut self.map);
            self.rooms.push(room);
        }
        self.take_snapshot();

        // Add corridors
        for depth in (0..max_depth).rev() {
            let mut parents = vec![root];

            // Find nodes at `depth`
            for _ in 0..depth {
                parents = parents
                    .iter()
                    .flat_map(|&index| graph.neighbors(index))
                    .collect();
            }

            // Connect the children of each node at `depth`
            for parent in parents {
                let children = graph.neighbors(parent).collect::<Vec<_>>();
                if children.len() < 2 {
                    continue;
                }
                for i in 0..children.len() - 1 {
                    connect_regions(
                        *graph.node_weight(children[i]).unwrap(),
                        *graph.node_weight(children[i + 1]).unwrap(),
                        &mut self.map,
                        rng,
                    );
                    self.take_snapshot();
                }
            }
        }

        // Shuffle rooms so that starting, stairs positions are interesting
        self.rooms.shuffle(rng);

        // Place stairs down in "last" room
        let last_room = self.rooms.last().unwrap();
        self.map[&last_room.center()] = TileType::DownStairs;
    }

    fn spawn_entities(&self, mut ev_spawn: EventWriter<Spawn>, _rng: &mut RNG) {
        ev_spawn.send(Spawn::Dog(self.rooms[1].center()));
        for room in self.rooms.iter().skip(1) {
            ev_spawn.send(Spawn::Area {
                area: *room,
                depth: 1, //self.map.depth,
            });
        }
    }

    fn get_starting_position(&self) -> UVec2 {
        self.rooms[0].center()
    }

    fn get_snapshots(&self) -> VecDeque<Map> {
        self.snapshot_manager.get_snapshots()
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }
}

impl BspMapBuilder {
    #[must_use]
    pub fn new(width: u32, height: u32, _depth: u32, config: BspConfig) -> Self {
        BspMapBuilder {
            rooms: vec![],
            map: Map::new([width, height]),
            snapshot_manager: SnapshotManager::new(),
            config,
        }
    }

    fn clear(&mut self, tile: TileType) {
        for x in 0..self.map.size.x {
            for y in 0..self.map.size.y {
                self.map[(x, y)] = tile;
            }
        }
    }

    fn take_subdivision_snapshot(&mut self, rects: Vec<URect>) {
        self.clear(TileType::Floor);
        for rect in rects {
            walls_around(&rect, &mut self.map);
        }
        self.take_snapshot();
        self.clear(TileType::Wall);
    }

    fn take_snapshot(&mut self) {
        self.snapshot_manager.record_snapshot(self.map.clone());
    }
}
