pub mod dialogue_texts;

use super::{CurrentCustomer, GameState, PlayState};
use bevy::prelude::*;
use dialogue_texts::*;
use std::path::PathBuf;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugins(());
        app.add_systems(OnEnter(GameState::Game), dialogue_setup_sys);
        app.add_systems(OnEnter(PlayState::CustomerRequest), request_setup_sys);
        // if we quit to main menu, we should get rid of the previous game shit
        // app.add_systems(OnEnter(GameState::Menu), dialogue_cleanup_sys);
    }
}

#[derive(Component)]
struct OnDialogueScreen;

fn dialogue_setup_sys(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // do dialogue

    // place the appropriate background box, maybe set up some resource or smth idk
    // get current customer probably?

    //if in PlayState::Disabled OR CustomerThankyou, or CustomerBonus then
    // we should move to CustomerRequest
}

fn request_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    current_customer: Res<CurrentCustomer>,
) {
    // do request things
    // check who is the current customer
    // create a filepath based on that
    let filepath_string: String = format!("texts/{}.json", current_customer.value);
    let filepath: PathBuf = PathBuf::from(filepath_string);

    // read in the info
    match read_customer_from_json(filepath) {
        Ok(customer) => info!("Loaded customer: {:?}", customer),
        Err(e) => error!("Error reading JSON: {}", e),
    }
}
