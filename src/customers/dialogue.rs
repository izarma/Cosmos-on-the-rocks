use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{
    customers::{OnCustomerScreen, cleanup_customer},
    engine::GameState,
};

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::new(),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .add_systems(
            OnEnter(GameState::CustomerInteraction),
            spawn_dialogue_runner,
        )
        .add_systems(OnExit(GameState::CustomerInteraction), cleanup_customer);
    }
}

pub fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    println!("Spawning dialogue runner with project: {:?}", project);
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.start_node("BartenderMonologue");
    commands.spawn((dialogue_runner, OnCustomerScreen));
}
