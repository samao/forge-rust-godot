use crate::{
    managers::visual_effect::VisualEffectType,
    message::Message,
    states::{PlayerState, attack::AttackState, event::StateEvent, idle::IdelState},
};
use godot::{classes::Input, obj::WithBaseField, prelude::*};

pub struct FallState {
    duration: f32,
    time: f32,
}

impl FallState {
    pub fn new() -> Self {
        Self {
            duration: 0.08,
            time: 0.0,
        }
    }
}

impl PlayerState for FallState {
    fn enter(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[状态] 进入 跌落");
        self.time = self.duration;
        _player.play_anim("fall");
    }

    fn exit(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[退出] 状态 跌落");
    }

    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::Physics { delta } => {
                let input = Input::singleton();
                let h = input.get_axis("left", "right");
                if h != 0.0 {
                    player.set_horizontal_speed(h * player.speed * 0.8);
                }

                self.time -= delta;
                if self.time <= 0.0 {
                    if player.base().is_on_floor() {
                        let pos = player.base().get_global_position();
                        Message::singleton()
                            .signals()
                            .play_effect()
                            .emit(VisualEffectType::Land, pos);
                        return Some(Box::new(IdelState::new()));
                    }
                }
            }
            StateEvent::InputJustPressed { action } if action == "attack" => {
                return Some(Box::new(AttackState::new(0)));
            }
            _ => {}
        }
        None
    }
}
