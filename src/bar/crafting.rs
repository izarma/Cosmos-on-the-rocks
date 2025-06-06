use bevy::prelude::*;

use crate::{
    animation::{AnimationEvent, sprite_animation::animate_spite},
    bar::{glass::spawn_glass, ingredient::spawn_ingredients},
    customers::spawn_bartender,
    engine::{GameState, audio_controller::play_crafting_bg},
};

#[derive(Component)]
pub struct OnCraftingScreen;

#[derive(Component)]
pub enum CraftButtons {
    Craft,
    Reset,
}

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Crafting),
            (
                spawn_ingredients,
                spawn_glass,
                spawn_bartender,
                play_crafting_bg,
                //spawn_craft_buttons,
            ),
        )
        .add_systems(Update, animate_spite.run_if(in_state(GameState::Crafting)))
        .add_systems(OnExit(GameState::Crafting), cleanup_crafting)
        .add_event::<AnimationEvent>();
    }
}

pub fn cleanup_crafting(mut commands: Commands, query: Query<Entity, With<OnCraftingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
