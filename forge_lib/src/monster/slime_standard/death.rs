use godot::{classes::InputEvent, prelude::*};

use crate::monster::enemy_state::IStateTrait;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct SlimeDeathState {
    base: Base<Node>,
    #[export]
    #[init(val = 100.0)]
    knockback_strength: f32,

    #[init(val = 0.0)]
    vel_x: f32,
    #[init(val = 0.0)]
    duration: f64,
    #[init(val = 0.0)]
    timer: f64,
}

impl IStateTrait for SlimeDeathState {
    fn base_node(&mut self) -> Gd<Node> {
        self.base_mut().clone().upcast()
    }
}

impl SlimeDeathState {
    fn calc_velocity(&mut self) {
        self.vel_x = 1.0;
        if let Some(mut board) = self.get_blackboard() {
            if let Some(attack_area) = board.bind_mut().get_damage_source()
                && let Some(enemy) = self.get_owner_node()
            {
                let attack_pos = attack_area.get_global_position();
                let enemy_pos = enemy.get_global_position();
                if attack_pos.x > enemy_pos.x {
                    self.vel_x = -1.0;
                }
            }
            self.vel_x *= self.knockback_strength;
            board.bind_mut().set_damage_source(None);
            board.bind_mut().set_can_decide(false);
        }
    }
}
#[godot_api]
impl SlimeDeathState {
    #[func]
    fn enter(&mut self) {
        self.play_animation("death");
        self.duration = self.get_current_animation_length();
        self.timer = 0.0;
        self.calc_velocity();
    }
    #[func]
    fn exit(&mut self) {}
    #[func]
    fn update(&mut self, delta: f32) {
        self.timer += delta as f64;
        if let Some(mut enemy) = self.get_owner_node() {
            let mut velocity = enemy.get_global_position();
            velocity.x = 0.0;
            enemy.set_velocity(velocity);
            if self.timer >= self.duration {
                enemy.queue_free();
            }
        }
    }
    #[func]
    fn handle_input(&mut self, _event: Gd<InputEvent>) {}
}
