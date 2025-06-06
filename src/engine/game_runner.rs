use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use bevy_asset_loader::prelude::*;
use bevy_seedling::prelude::*;

use crate::{
    bar::crafting::CraftingPlugin,
    customers::CustomerPlugin,
    engine::{
        GameState,
        asset_loader::{AudioAssets, ImageAssets},
    },
    ui::GameUiPlugin,
};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SeedlingPlugin::default(),
            GameUiPlugin,
            CustomerPlugin,
            CraftingPlugin,
        ))
        // .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .load_collection::<AudioAssets>()
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::Crafting),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Crafting), spawn_background);
    }
}

#[derive(Component)]
pub struct MainGameCamera;

fn setup_camera(mut commands: Commands) {
    let main_camera = Camera2d::default();
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: 1080.0,
        },
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((main_camera, MainGameCamera, projection));
}

fn spawn_background(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite {
            image: image_assets.background_image.clone(),
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}

// cgekc