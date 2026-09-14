use godot::{global::godot_print, obj::WithBaseField};

use crate::states::{PlayerState, event::StateEvent, idle::IdelState};

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
        player.base_mut().set_modulate(color.with_alpha(80.0));
        player.set_horizontal_speed(speed * dir);
        player.set_player_disable(true);
        player.play_anim("dash");
        player.play_sound_effect("dash");
    }

    fn exit(&mut self, player: &mut crate::player::Player) {
        player.set_player_disable(false);
        let color = player.base().get_modulate();
        player.base_mut().set_modulate(color.with_alpha(255.0));
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
                    return Some(Box::new(IdelState::new()));
                }
            }
            _ => {}
        }
        None
    }
}
