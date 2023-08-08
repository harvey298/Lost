use bevy::prelude::*;

use crate::{GameStates, AllEntites};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub struct MainMenu {
    pub ids: Vec<i32>
}

#[derive(Component)]
pub struct UICamera;

#[derive(Component)]
pub struct UIObject;

#[derive(Component)]
pub struct ConnectButton;


impl Plugin for MainMenu {

    fn build(&self, app: &mut App) {
        
        app        // systems to run only in the main menu
        // .add_system_set(
        //     SystemSet::on_update(GameStates::MainMenu)
        //         .with_system(button_system)
        // )

        // // // setup when entering the state
        // .add_system_set(
        //     SystemSet::on_enter(GameStates::MainMenu)
        //         .with_system(MainMenu::render_ui)
        // )

        .add_system(MainMenu::render_ui.in_schedule(OnEnter(GameStates::MainMenu)))

        .add_system(button_system.in_set(OnUpdate(GameStates::MainMenu)))

        .add_system(MainMenu::remove_ui.in_schedule(OnExit(GameStates::MainMenu)))

        // .add_system_set(
        //     SystemSet::on_exit(GameStates::MainMenu)
        //         .with_system(MainMenu::remove_ui)
        // )

        ;

    }

}


impl MainMenu {
    pub fn new() -> Self {
        Self{ ids: Vec::new() }
    }

    pub fn render_ui(mut commands: Commands, assets: Res<AssetServer>, mut ent_ids: ResMut<AllEntites>) {
        println!("Rendering Main Menu");
        let id = commands.spawn( Camera2dBundle::default() ).insert(UICamera).insert(UIObject).id();

        ent_ids.ids.push(id);

        let mut button = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        });

        ent_ids.ids.push(button.id());

        let id = button.with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Connect",
                TextStyle {
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            ));
        }).id();

        ent_ids.ids.push(id);
    }
    
    pub fn remove_ui(mut commands: Commands, assets: Res<AssetServer>, mut ent_ids: ResMut<AllEntites>) {
        println!("Removing Main menu");
        
        for ent in &ent_ids.ids {
            commands.entity(*ent).despawn_recursive();
        }

        ent_ids.ids.clear();        
    }
}


pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>)
    >,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    mut game_state: ResMut<State<GameStates>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Connecting".to_string();
                
                // game_state.set(GameStates::Loading).unwrap();

                game_state.0 = GameStates::Loading;



            }
            Interaction::Hovered => {
                text.sections[0].value = "Ready".to_string();
                // *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                // *color = NORMAL_BUTTON.into();
            }
        }
    }
}