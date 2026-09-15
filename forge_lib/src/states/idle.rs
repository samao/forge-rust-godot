use godot::{classes::Input, global::godot_print, obj::Singleton};

use crate::states::{
    PlayerState, attack::AttackState, crouch::CrouchState, dash::DashState, event::StateEvent,
    hurt::HurtState, jump::JumpState, morph::MorphState, run::RunState,
};

pub struct IdelState {
    breath_timer: f32,
}

impl IdelState {
    pub fn new() -> Self {
        Self { breath_timer: 0.0 }
    }
}

impl PlayerState for IdelState {
    fn enter(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[状态] 进入空闲");
        _player.set_horizontal_speed(0.0);
        _player.play_anim("idle");
    }
    fn exit(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[退出] 空闲");
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
            StateEvent::InputPressed { action } if action == "down" => {
                return Some(Box::new(CrouchState::new()));
            }
            StateEvent::InputJustPressed { action } if action == "dash" => {
                return Some(Box::new(DashState::new()));
            }
            StateEvent::InputJustPressed { action } if action == "roll" => {
                return Some(Box::new(MorphState::new()));
            }
            StateEvent::Physics { delta } => {
                let input = Input::singleton();
                let h = input.get_axis("left", "right");
                if h != 0.0 {
                    return Some(Box::new(RunState::new(h)));
                }
                self.breath_timer += delta;
                if self.breath_timer > 0.5 {
                    self.breath_timer = 0.0;
                }
                None
            }
            StateEvent::TakeDamage {
                damage: _,
                knockback: __,
            } => {
                return Some(Box::new(HurtState::new(0.5)));
            }
            _ => None,
        }
    }
}
