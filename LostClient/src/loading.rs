use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;

use crate::{net::{create_renet_client, client_start_connection, self}, GameStates};

pub struct Loading;

impl Plugin for Loading {
    fn build(&self, app: &mut bevy::prelude::App) {

        app.add_plugin(RenetClientPlugin);
        app.insert_resource(create_renet_client());

        app.insert_resource(net::util::Me{ username: "".to_string(), id:"".to_string(), server_id: 0 });

        app

        .add_system(client_start_connection.in_schedule(OnEnter(GameStates::Loading)))

        .add_system(change_mode.in_set(OnUpdate(GameStates::Loading)))

        .add_system(on_exit.in_schedule(OnExit(GameStates::Loading)));


        // app.add_system_set(
        //     SystemSet::on_enter(GameStates::Loading)
        //         .with_system(client_start_connection)
        // );

        // app.add_system_set(
        //     SystemSet::on_update(GameStates::Loading)
        //         .with_system(change_mode)
        // );

        // app.add_system_set(
        //     SystemSet::on_exit(GameStates::Loading)
        //         .with_system(on_exit)
        // );
        

        
        
    }
}

impl Loading {

}

fn on_exit() {
    info!("Loading game now");
}

fn change_mode(mut game_state: ResMut<State<GameStates>>,) {
    game_state.0 = GameStates::InGame;

    info!("{:?}",game_state.0);
}