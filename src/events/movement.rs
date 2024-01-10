// / An event sent for a movement input action
#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
    Jump,
}
