use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


#[derive(Resource)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
  Move
}

impl Actionlike for PlayerAction {
  fn input_control_kind(&self) -> InputControlKind {
    match self {
      PlayerAction::Move => InputControlKind::DualAxis,
    }
  }
}

impl PlayerAction {
  pub fn default_input_map() -> InputMap<Self> {

    // Apply controls to the input
    let mut input_map = InputMap::default();
    input_map.insert_dual_axis(PlayerAction::Move, KeyboardVirtualDPad::WASD);

    input_map
  } 
}

// impl PlayerAction {
//   pub fn default_input_map() -> InputMap<Self> {
//     let mut input_map = InputMap::default();
//     input_map.with_dual_axis(PlayerAction::Move, KeyboardVirtualDPad::WASD)
//     // input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::WASD);

    
//   }

// }