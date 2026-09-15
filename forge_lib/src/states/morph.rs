use godot::obj::WithBaseField;

use crate::states::{PlayerState, event::StateEvent, fall::FallState, idle::IdelState};

pub struct MorphState {
    duration: f64,
    time: f64,
}

impl MorphState {
    pub fn new() -> Self {
        Self {
            duration: 0.8,
            time: 0.0,
        }
    }
}

impl PlayerState for MorphState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        self.time = self.duration;
        player.play_anim("ball");
        player.set_morph_roll(true);
        player.play_sound_effect("morph");
    }

    fn exit(&mut self, player: &mut crate::player::Player) {
        player.set_morph_roll(false);
    }

    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::Physics { delta } => {
                let dt = delta as f64;

                self.time -= dt;
                let dir = player.get_face_direction();

                player.set_horizontal_speed(dir * player.speed * 1.5);

                if self.time <= 0.0 {
                    if player.base().is_on_floor() {
                        return Some(Box::new(IdelState::new()));
                    } else {
                        return Some(Box::new(FallState::new()));
                    }
                }
            }
            _ => {}
        }
        None
    }
}
