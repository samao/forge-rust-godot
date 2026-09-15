use godot::{
    global::godot_print,
    obj::{Singleton, WithBaseField},
};

use crate::{
    managers::visual_effect::VisualEffectType,
    message::Message,
    states::{PlayerState, event::StateEvent, idle::IdelState},
};

pub struct SlamState {
    effect_delay: f64,
    effect_time: f64,
}

impl SlamState {
    pub fn new() -> Self {
        Self {
            effect_delay: 0.05,
            effect_time: 0.0,
        }
    }
}

impl PlayerState for SlamState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        godot_print!("进入下砸");
        self.effect_time = self.effect_delay;
        player.play_anim("slam");
        player.set_slam_active(true);
    }

    fn exit(&mut self, player: &mut crate::player::Player) {
        godot_print!("退出下砸");
        player.set_slam_active(false);
        player.play_sound_effect("slam");
        let pos = player.base().get_global_position();
        Message::singleton()
            .signals()
            .play_effect()
            .emit(VisualEffectType::Hit, pos);
        Message::singleton().signals().camera_shake().emit(10.0);
    }

    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::Physics { delta } => {
                let dt = delta as f64;
                self.effect_time -= dt;

                if self.effect_time <= 0.0 {
                    self.effect_time = self.effect_delay;
                    player.create_tail_shadow();
                }

                player.set_horizontal_speed(0.0);
                player.set_ver_speed(player.speed * 2.0);

                if player.base().is_on_floor() {
                    return Some(Box::new(IdelState::new()));
                }
            }
            _ => {}
        }
        None
    }
}
