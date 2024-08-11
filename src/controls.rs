use bevy::prelude::*;


#[derive(Resource)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}



pub fn user_input(
    input: Res<ButtonInput<KeyCode>>,
    mut last_pressed: ResMut<Direction>,
) {
    if input.pressed(KeyCode::ArrowUp) {
        *last_pressed = Direction::Up;
    } else if input.pressed(KeyCode::ArrowDown) {
        *last_pressed = Direction::Down;
    } else if input.pressed(KeyCode::ArrowLeft) {
        *last_pressed = Direction::Left;
    } else if input.pressed(KeyCode::ArrowRight) {
        *last_pressed = Direction::Right;
    }
}