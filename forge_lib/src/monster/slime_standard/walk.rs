use godot::{classes::InputEvent, prelude::*};

use crate::monster::enemy_state::IStateTrait;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct SlimeWalkState {
    base: Base<Node>,

    #[export]
    #[init(val = 50.0)]
    walk_speed: f32,
}

impl IStateTrait for SlimeWalkState {
    fn base_node(&mut self) -> Gd<Node> {
        self.base_mut().clone().upcast()
    }
}

#[godot_api]
impl SlimeWalkState {
    #[func]
    fn enter(&mut self) {
        self.play_animation("walk");
    }

    #[func]
    fn re_enter(&mut self) {}

    #[func]
    fn exit(&mut self) {}
    #[func]
    fn update(&mut self, _delta: f32) {
        if let Some(mut enemy) = self.get_owner_node() {
            if let Some(board) = self.get_blackboard() {
                let dir = board.bind().get_dir();
                if enemy.is_on_wall() {
                    enemy.bind_mut().change_dir(-dir);
                    return;
                }
                let mut velocity = enemy.get_velocity();
                velocity.x = self.walk_speed * dir.x;
                enemy.set_velocity(velocity);
            }
        }
    }
    #[func]
    fn handle_input(&mut self, _event: Gd<InputEvent>) {}
}
