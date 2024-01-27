mod bundles;
mod components;
mod constants;
mod events;
mod resources;
mod systems;
mod states;

use bevy::{
    ecs::{schedule::ScheduleLabel, system::Res},
    prelude::{
        default, AlignItems, App, AssetServer, BackgroundColor, BorderColor, BuildChildren, Button,
        ButtonBundle, Camera2dBundle, Changed, Children, ClearColor, Color, Commands,
        DefaultPlugins, FixedUpdate, Interaction, IntoSystemConfigs, JustifyContent, NodeBundle,
        Query, Startup, Style, Text, TextBundle, TextStyle, UiRect, Update, Val, With,
    },
    window,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/no-heart.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .insert_resource(resources::Scoreboard { score: 0 })
//         .insert_resource(ClearColor(constants::background::BACKGROUND_COLOR))
//         .add_event::<events::CollisionEvent>()
//         .add_plugins(systems::SetupBreakoutPlugin)
//         .add_plugins((
//             systems::ApplyVelocityPlugin,
//             systems::MovePaddlePlugin,
//             systems::CheckForCollisionsPlugin,
//             systems::PlayCollisionSoundPlugin,
//         ))
//         .add_plugins(systems::UpdateScoreboardPlugin)
//         .add_systems(Startup, (setup_button))
//         .add_systems(Update, (button_system, window::close_on_esc))
//         .run();
// }



use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<states::AppState>()


        .add_plugins(systems::SetupBreakoutPlugin{
            schedule: OnEnter(states::AppState::Menu).intern(),
            state: None,
        })
        .add_plugins((
            systems::ApplyVelocityPlugin {
                schedule: Update.intern(),
                state: Some(states::AppState::Menu),
            },
            systems::MovePaddlePlugin {
                schedule: Update.intern(),
                state: Some(states::AppState::Menu),
            },
            systems::CheckForCollisionsPlugin{
                schedule: Update.intern(),
                state: Some(states::AppState::Menu),
            },
            systems::PlayCollisionSoundPlugin{
                schedule: Update.intern(),
                state: Some(states::AppState::Menu),
            }
        ))
        .add_plugins(systems::UpdateScoreboardPlugin {
            schedule: Update.intern(),
            state: Some(states::AppState::Menu),
        })
        


        // .add_systems(Startup, setup)
        // This system runs when we enter `AppState::Menu`, during the `StateTransition` schedule.
        // All systems from the exit schedule of the state we're leaving are run first,
        // and then all systems from the enter schedule of the state we're entering are run second.
        // .add_systems(OnEnter(AppState::Menu), setup_menu)
        // .add_plugins(systems::MovePaddlePlugin)
        // By contrast, update systems are stored in the `Update` schedule. They simply
        // check the value of the `State<T>` resource to see if they should run each frame.
        // .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        // .add_systems(OnExit(AppState::Menu), cleanup_menu)
        // .add_systems(OnEnter(AppState::InGame), setup_game)
        // .add_systems(
        //     Update,
        //     (movement, change_color).run_if(in_state(AppState::InGame)),
        // )
        .run();
}

// #[derive(Resource)]
// struct MenuData {
//     button_entity: Entity,
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
// }

// fn setup_menu(mut commands: Commands) {
//     let button_entity = commands
//         .spawn(NodeBundle {
//             style: Style {
//                 // center button
//                 width: Val::Percent(100.),
//                 height: Val::Percent(100.),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             ..default()
//         })
//         .with_children(|parent| {
//             parent
//                 .spawn(ButtonBundle {
//                     style: Style {
//                         width: Val::Px(150.),
//                         height: Val::Px(65.),
//                         // horizontally center child text
//                         justify_content: JustifyContent::Center,
//                         // vertically center child text
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     background_color: NORMAL_BUTTON.into(),
//                     ..default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn(TextBundle::from_section(
//                         "Play",
//                         TextStyle {
//                             font_size: 40.0,
//                             color: Color::rgb(0.9, 0.9, 0.9),
//                             ..default()
//                         },
//                     ));
//                 });
//         })
//         .id();
//     commands.insert_resource(MenuData { button_entity });
// }

// fn menu(
//     mut next_state: ResMut<NextState<AppState>>,
//     mut interaction_query: Query<
//         (&Interaction, &mut BackgroundColor),
//         (Changed<Interaction>, With<Button>),
//     >,
// ) {
//     for (interaction, mut color) in &mut interaction_query {
//         match *interaction {
//             Interaction::Pressed => {
//                 *color = PRESSED_BUTTON.into();
//                 next_state.set(AppState::InGame);
//             }
//             Interaction::Hovered => {
//                 *color = HOVERED_BUTTON.into();
//             }
//             Interaction::None => {
//                 *color = NORMAL_BUTTON.into();
//             }
//         }
//     }
// }

// fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
//     commands.entity(menu_data.button_entity).despawn_recursive();
// }

// fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(SpriteBundle {
//         texture: asset_server.load("branding/icon.png"),
//         ..default()
//     });
// }

// const SPEED: f32 = 100.0;
// fn movement(
//     time: Res<Time>,
//     input: Res<Input<KeyCode>>,
//     mut query: Query<&mut Transform, With<Sprite>>,
// ) {
//     for mut transform in &mut query {
//         let mut direction = Vec3::ZERO;
//         if input.pressed(KeyCode::Left) {
//             direction.x -= 1.0;
//         }
//         if input.pressed(KeyCode::Right) {
//             direction.x += 1.0;
//         }
//         if input.pressed(KeyCode::Up) {
//             direction.y += 1.0;
//         }
//         if input.pressed(KeyCode::Down) {
//             direction.y -= 1.0;
//         }

//         if direction != Vec3::ZERO {
//             transform.translation += direction.normalize() * SPEED * time.delta_seconds();
//         }
//     }
// }

// fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
//     for mut sprite in &mut query {
//         sprite
//             .color
//             .set_b((time.elapsed_seconds() * 0.5).sin() + 2.0);
//     }
// }