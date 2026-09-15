use godot::{classes::Input, obj::WithBaseField, prelude::*};

use crate::{
    managers::visual_effect::VisualEffectType,
    message::Message,
    states::{
        PlayerState, attack::AttackState, dash::DashState, event::StateEvent, fall::FallState,
        hurt::HurtState, idle::IdelState, run::RunState, slam::SlamState,
    },
};

pub struct JumpState {
    velocity: f32,
    jump_count: u32,
    // coyote_timer: f32,
}

impl JumpState {
    pub fn new(v: f32, count: u32) -> Self {
        godot_print!("new Jump state: v = {}, c = {}", v, count);
        Self {
            velocity: v,
            jump_count: count,
            // coyote_timer: 0.08,
        }
    }
}

impl PlayerState for JumpState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        godot_print!("[状态] 进入跳跃 (第{}段)", self.jump_count + 1);
        let pos = player.base().get_global_position();
        let mut v = player.base().get_velocity();
        v.y = self.velocity;
        player.base_mut().set_velocity(v);
        player.play_sound("jump");
        player.play_anim("jump");
        Message::singleton()
            .signals()
            .play_effect()
            .emit(VisualEffectType::Jump, pos);
    }
    fn exit(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[退出] 状态 跳跃");
    }
    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::InputJustPressed { action } if action == "jump" && self.jump_count < 1 => {
                godot_print!("执行二段跳");
                return Some(Box::new(JumpState::new(
                    player.jump_velocity * 0.9,
                    self.jump_count + 1,
                )));
            }
            StateEvent::InputJustPressed { action } if action == "dash" => {
                if player.base().get_velocity().y.abs() < player.speed * 0.8 {
                    return Some(Box::new(DashState::new()));
                }
            }
            StateEvent::InputJustPressed { action } if action == "attack" => {
                return Some(Box::new(AttackState::new(0)));
            }
            StateEvent::InputPressed { action } if action == "down" => {
                if player.base().get_velocity().y.abs() < player.speed * 0.8 {
                    return Some(Box::new(SlamState::new()));
                }
            }
            StateEvent::InputJustRelease { action } if action == "jump" => {
                let v = player.base().get_velocity();
                if v.y < -50.0 {
                    player.base_mut().set_velocity(Vector2::new(v.x, v.y * 0.5));
                }
            }
            StateEvent::Physics { delta: _ } => {
                if player.base().is_on_floor() && player.base().get_velocity().y >= 0.0 {
                    let input = Input::singleton();
                    let h = input.get_axis("left", "right");
                    if h != 0.0 {
                        return Some(Box::new(RunState::new(h)));
                    } else {
                        godot_print!("在这里退出了jump");
                        return Some(Box::new(IdelState::new()));
                    }
                }

                if !player.base().is_on_floor() && player.base().get_velocity().y > 0.0 {
                    return Some(Box::new(FallState::new()));
                }

                let input = Input::singleton();
                let h = input.get_axis("left", "right");
                if h != 0.0 {
                    player.set_horizontal_speed(h * player.speed * 0.8);
                }
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
