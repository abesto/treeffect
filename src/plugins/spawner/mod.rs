mod plugin;
mod systems;

pub use plugin::SpawnerPlugin;

pub mod events {
    pub use super::systems::generic::Spawn;
    pub use super::systems::player::SpawnPlayer;
}
