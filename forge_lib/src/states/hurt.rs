use godot::{classes::Input, obj::WithBaseField, prelude::*};

use crate::states::{
    PlayerState, event::StateEvent, fall::FallState, idle::IdelState, run::RunState,
};

pub struct HurtState {
    timer: f32,
    duration: f32,
}

impl HurtState {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: 0.0,
            duration,
        }
    }
}

impl PlayerState for HurtState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        godot_print!("[状态] 进入受伤");
        player.play_anim("hurt");
        // player.set_horizontal_speed(0.0);
        player.start_timer("hurt_timer", self.duration);
    }

    fn exit(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[退出] 状态 受伤");
    }

    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::TimerTimeout { timer_name } if timer_name == "hurt_timer" => {
                let input = Input::singleton();
                let h = input.get_axis("left", "right");
                if player.base().is_on_floor() {
                    if h != 0.0 {
                        return Some(Box::new(RunState::new(h)));
                    } else {
                        return Some(Box::new(IdelState::new()));
                    }
                } else {
                    return Some(Box::new(FallState::new()));
                }
            }
            StateEvent::TakeDamage { damage, knockback } => {
                player.apply_damage(damage, knockback);
                self.timer = 0.0;
                return None;
            }
            _ => {}
        }
        None
    }
}
