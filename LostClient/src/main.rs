use bevy::{prelude::*, ecs::schedule::ScheduleLabel};
use bevy_text_mesh::prelude::*;
use game::{Game, camera::FPSCamera};
use loading::Loading;
use main_menu::MainMenu;
use net::util::EnemySpawnChannels;

mod main_menu;
mod loading;
mod net;
mod game;
mod debug;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameStates {
    Starting,
    MainMenu,
    Loading,
    InGame
}

impl Default for GameStates { fn default() -> Self { Self::Starting } }

#[derive(Resource)]
pub struct AllEntites {
    pub ids: Vec<Entity>
}

fn main() {
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenu::new())
        .insert_resource(AllEntites{ ids: Vec::new() })
        .insert_resource(EnemySpawnChannels::default())

        .add_state::<GameStates>()

        .add_default_schedules()

        .add_plugin(Loading)
        
        .add_plugin(TextMeshPlugin)

        .add_plugin(Game);

        app.add_plugin(FPSCamera);

    #[cfg(debug_assertions)]
    app.add_plugin(debug::Debug);


    app.run();

}
