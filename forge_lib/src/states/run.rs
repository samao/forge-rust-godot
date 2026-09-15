use godot::classes::Input;
use godot::obj::Singleton;
use godot::prelude::*;

use crate::states::dash::DashState;
use crate::states::fall::FallState;
use crate::states::hurt::HurtState;
use crate::states::idle::IdelState;
use crate::states::morph::MorphState;
use crate::states::{PlayerState, attack::AttackState, event::StateEvent, jump::JumpState};

pub struct RunState {
    direction: f32,
}

impl PlayerState for RunState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        godot_print!("[状态] 进入 奔跑");
        player.play_anim("run");
    }
    fn exit(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[退出] 奔跑 状态");
    }
    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::InputJustPressed { action }
                if action == "jump" && player.coyote_timer > 0.0 =>
            {
                return Some(Box::new(JumpState::new(player.jump_velocity, 0)));
            }
            StateEvent::InputJustPressed { action } if action == "attack" => {
                return Some(Box::new(AttackState::new(0)));
            }
            StateEvent::InputJustPressed { action } if action == "dash" => {
                return Some(Box::new(DashState::new()));
            }
            StateEvent::InputJustPressed { action } if action == "roll" => {
                return Some(Box::new(MorphState::new()));
            }
            StateEvent::Physics { delta: _ } => {
                let input = Input::singleton();
                let h = input.get_axis("left", "right");
                if player.base().is_on_floor() {
                    if h == 0.0 {
                        return Some(Box::new(IdelState::new()));
                    }
                } else {
                    return Some(Box::new(FallState::new()));
                }
                player.set_horizontal_speed(h * player.speed);
                self.direction = h;
            }
            StateEvent::TakeDamage {
                damage: _,
                knockback: __,
            } => {
                return Some(Box::new(HurtState::new(0.5)));
            }
            _ => {}
        }
        None
    }
}

impl RunState {
    pub fn new(dir: f32) -> Self {
        Self { direction: dir }
    }
}
