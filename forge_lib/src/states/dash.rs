use godot::{global::godot_print, obj::WithBaseField};

use crate::states::{
    PlayerState, event::StateEvent, fall::FallState, idle::IdelState, slam::SlamState,
};

pub struct DashState {
    time: f64,
    duration: f64,
    effect_time: f64,
    effect_time_gap: f64,
}

impl DashState {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            duration: 0.3,
            effect_time: 0.0,
            effect_time_gap: 0.05,
        }
    }
}

impl PlayerState for DashState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        self.time = self.duration;
        self.effect_time = self.effect_time_gap;
        let speed = player.speed * 2.0;
        let dir = player.get_face_direction();
        godot_print!("Dash: {speed}, Dir: {dir}");
        let color = player.base().get_modulate();
        player.base_mut().set_modulate(color.with_alpha(0.2));
        player.set_horizontal_speed(speed * dir);
        player.set_player_disable(true);
        player.play_anim("dash");
        player.set_ver_speed(0.0);
        player.play_sound_effect("dash");
        player.set_gravity_disable(true);
    }

    fn exit(&mut self, player: &mut crate::player::Player) {
        player.set_player_disable(false);
        let color = player.base().get_modulate();
        player.base_mut().set_modulate(color.with_alpha(1.0));
        player.set_gravity_disable(false);
        player.set_horizontal_speed(0.0);
    }

    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        _event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match _event {
            StateEvent::Physics { delta } => {
                let dt = delta as f64;
                self.time -= dt;
                player.set_ver_speed(0.0);

                self.effect_time -= dt;
                if self.effect_time <= 0.0 {
                    player.create_tail_shadow();
                    self.effect_time = self.effect_time_gap;
                }

                if self.time <= 0.0 {
                    if player.base().is_on_floor() {
                        return Some(Box::new(IdelState::new()));
                    } else {
                        return Some(Box::new(FallState::new()));
                    }
                }
            }
            StateEvent::InputJustRelease { action } if action == "down" => {
                return Some(Box::new(SlamState::new()));
            }
            _ => {}
        }
        None
    }
}
