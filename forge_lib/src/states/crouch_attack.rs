use godot::global::{godot_print, wrapi};

use crate::states::{
    PlayerState, crouch::CrouchState, dash::DashState, event::StateEvent, fall::FallState,
    idle::IdelState, jump::JumpState,
};

pub struct CrouchAttack {
    combo: i64,
    duration: f64,
    time: f64,
    comboing: bool,
}

impl CrouchAttack {
    pub fn new(combo: i64) -> Self {
        Self {
            combo,
            duration: 0.3,
            time: 0.0,
            comboing: false,
        }
    }

    fn get_next_combo(&self) -> i64 {
        let next_combo = wrapi(self.combo + 1, 0, 2);
        next_combo
    }
}

impl PlayerState for CrouchAttack {
    fn enter(&mut self, player: &mut crate::player::Player) {
        godot_print!("蹲伏攻击: {} 段", self.combo);
        player.play_anim(format!("crouch_attack_{}", self.combo).as_str());
        self.time = 0.0;
        self.comboing = false;
        player.set_attack_enabled(true);
    }

    fn exit(&mut self, player: &mut crate::player::Player) {
        player.set_attack_enabled(false);
        godot_print!("退出蹲伏攻击: {} 段", self.combo);
    }

    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::Physics { delta } => {
                let dt = delta as f64;
                self.time += dt;

                if self.time > self.duration {
                    if self.comboing {
                        let next_combo = self.get_next_combo();
                        return Some(Box::new(CrouchAttack::new(next_combo)));
                    } else {
                        return Some(Box::new(CrouchState::new(true)));
                    }
                }
            }

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
                if self.time > self.duration * 0.7 {
                    self.comboing = true;
                }
            }
            _ => {}
        }
        None
    }
}
