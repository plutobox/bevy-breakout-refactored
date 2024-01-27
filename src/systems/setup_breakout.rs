use bevy::{
    app::{App, Plugin, Startup}, asset::{AssetServer, Assets}, core_pipeline::{clear_color::ClearColor, core_2d::Camera2dBundle}, ecs::{schedule::{common_conditions::in_state, IntoSystemConfigs, ScheduleLabel}, system::{Commands, Res, ResMut}}, math::{Vec2, Vec3}, render::mesh::{shape, Mesh}, sprite::ColorMaterial, utils::intern::Interned
};

use crate::{
    bundles, constants, events, resources, states
};

pub struct SetupBreakoutPlugin {
    pub schedule: Interned<dyn ScheduleLabel>,
    pub state: Option<states::AppState>,
}

impl Plugin for SetupBreakoutPlugin {
    fn build(&self, app: &mut App) {
        // TODO - how do I properly handle this accounting for state?
        app
            .insert_resource(resources::Scoreboard { score: 0 })
            .insert_resource(ClearColor(constants::background::BACKGROUND_COLOR))
            .add_event::<events::CollisionEvent>();

        match self.state {
            Some(s) => app.add_systems(self.schedule, setup_breakout.run_if(in_state(s))),
            None => app.add_systems(self.schedule, setup_breakout),
        };
    }
}

fn setup_breakout(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(resources::CollisionSound(ball_collision_sound));

    // Paddle
    let paddle_y = constants::wall::BOTTOM_WALL + constants::paddle::GAP_BETWEEN_PADDLE_AND_FLOOR;
    let paddle_translation = Vec3::new(0.0, paddle_y, 0.0);

    commands.spawn(bundles::PaddleBundle::new(paddle_translation));

    // Ball
    commands.spawn(bundles::BallBundle::new(
        meshes.add(shape::Circle::default().into()),
        materials.add(ColorMaterial::from(constants::ball::BALL_COLOR)),
        None
    ));

    // Ball number 2
    commands.spawn(bundles::BallBundle::new(
        meshes.add(shape::Circle::default().into()),
        materials.add(ColorMaterial::from(constants::ball::BALL_COLOR)),
        Some(Vec3::new(0.0, -25.0, 1.0))
    ));

    // Scoreboard
    commands.spawn(bundles::ScoreboardBundle::new("Score: "));

    // Walls
    commands.spawn(bundles::WallBundle::new(bundles::WallLocation::Left));
    commands.spawn(bundles::WallBundle::new(bundles::WallLocation::Right));
    commands.spawn(bundles::WallBundle::new(bundles::WallLocation::Bottom));
    commands.spawn(bundles::WallBundle::new(bundles::WallLocation::Top));

    // Bricks
    let total_width_of_bricks = (constants::wall::RIGHT_WALL - constants::wall::LEFT_WALL) - 2. * constants::brick::GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = paddle_y + constants::paddle::GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks = constants::wall::TOP_WALL - bottom_edge_of_bricks - constants::brick::GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_bricks / (constants::brick::BRICK_SIZE.x + constants::brick::GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (constants::brick::BRICK_SIZE.y + constants::brick::GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    let center_of_bricks = (constants::wall::LEFT_WALL + constants::wall::RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        // Space taken up by the bricks
        - (n_columns as f32 / 2.0 * constants::brick::BRICK_SIZE.x)
        // Space taken up by the gaps
        - n_vertical_gaps as f32 / 2.0 * constants::brick::GAP_BETWEEN_BRICKS;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_bricks + constants::brick::BRICK_SIZE.x / 2.;
    let offset_y = bottom_edge_of_bricks + constants::brick::BRICK_SIZE.y / 2.;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x + column as f32 * (constants::brick::BRICK_SIZE.x + constants::brick::GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (constants::brick::BRICK_SIZE.y + constants::brick::GAP_BETWEEN_BRICKS),
            );

            commands.spawn(bundles::BrickBundle::new(brick_position));
        }
    }
}
