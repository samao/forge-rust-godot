use godot::prelude::*;

use crate::states::{
    PlayerState, crouch_attack::CrouchAttack, dash::DashState, event::StateEvent, fall::FallState,
    idle::IdelState, jump::JumpState,
};

pub struct CrouchState {
    fix_to_end: bool,
}

impl CrouchState {
    pub fn new(fix: bool) -> Self {
        Self { fix_to_end: fix }
    }
}

impl PlayerState for CrouchState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        godot_print!("[进入] 状态 蹲伏");
        if self.fix_to_end {
            player.anim_seek_to_end("crouch");
        } else {
            player.play_anim("crouch");
        }
    }

    fn exit(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[状态] 退出 蹲伏");
    }

    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::InputJustPressed { action } if action == "jump" => {
                if player.is_one_way_collidering() {
                    player.prepar_fall();
                    return Some(Box::new(FallState::new()));
                }
                return Some(Box::new(JumpState::new(player.jump_velocity, 0)));
            }
            StateEvent::InputJustPressed { action } if action == "dash" => {
                return Some(Box::new(DashState::new()));
            }
            StateEvent::InputJustRelease { action } if action == "down" => {
                return Some(Box::new(IdelState::new()));
            }
            StateEvent::InputJustPressed { action } if action == "attack" => {
                return Some(Box::new(CrouchAttack::new(0)));
            }
            _ => {}
        }
        None
    }
}
