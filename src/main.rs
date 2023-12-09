use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, TiledCamera, ToWorld};
use bevy_prng::WyRand;
use bevy_rand::prelude::*;
use consts::MAP_SIZE;
use events::EventsPlugin;
use plugins::ai::AiPlugin;
use plugins::energy::EnergyPlugin;
use plugins::input::InputPlugin;
use plugins::map::MapPlugin;
use plugins::render::RenderPlugin;
use plugins::spawner::SpawnerPlugin;

mod behaviors;
mod bundles;
mod components;
mod consts;
mod events;
mod plugins;
mod util;

struct SetupPlugin;

impl SetupPlugin {
    fn setup_terminal(mut commands: Commands) {
        let mut terminal = Terminal::new(MAP_SIZE);
        terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));
        commands.spawn((
            TerminalBundle::from(terminal),
            AutoCamera,
            ToWorld::default(),
        ));
    }

    fn setup_resolution(q_camera: Query<&TiledCamera>, mut q_window: Query<&mut Window>) {
        let camera = q_camera.single();
        let mut window = q_window.single_mut();
        let resolution = camera.target_resolution();
        window.resolution =
            bevy::window::WindowResolution::new(resolution.x as f32, resolution.y as f32);
        window.resizable = false;
    }
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            TerminalPlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .add_systems(Startup, Self::setup_terminal)
        // Running setup_resolution on each update is horrible, but 1. `AutoCamera` runs stuff in `First` which also runs on each update, and 2. I ran out of fucks
        .add_systems(PreUpdate, Self::setup_resolution);
    }
}

fn main() {
    App::new()
        .add_plugins((
            EventsPlugin,
            SetupPlugin,
            MapPlugin { size: MAP_SIZE },
            SpawnerPlugin,
            InputPlugin,
            AiPlugin,
            EnergyPlugin,
            RenderPlugin,
        ))
        .run();
}
