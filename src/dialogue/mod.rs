use std::fs;

static DIALOGUE_FILE:PathBuf = PathBuf::from("assets/texts/npc_dialogue.json");

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
  fn build(&self, app: &mut App) {
        //app.add_plugins(());
        app.add_systems(OnEnter(GameState::Game), dialogue_setup_sys);
    }
}

#[derive(Component)]
struct Dialogue;

fn dialogue_setup_sys(mut cmd: Commands, asset_server:Res<AssetServer>) {
  // do dialogue
}

fn get_dialogue_from_file() {
  let dialogue_text = fs::read_to_string(DIALOGUE_FILE);
}