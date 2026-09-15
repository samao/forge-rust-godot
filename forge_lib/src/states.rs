// use std::fmt::Debug;

use crate::player::Player;

pub mod attack;
pub mod crouch;
pub mod dash;
pub mod die;
pub mod event;
pub mod fall;
pub mod hurt;
pub mod idle;
pub mod jump;
pub mod morph;
pub mod run;
pub mod slam;
use event::StateEvent;

pub trait PlayerState: Send + Sync {
    fn enter(&mut self, _player: &mut Player) {}

    fn exit(&mut self, _player: &mut Player) {}

    fn handle_event(
        &mut self,
        player: &mut Player,
        event: StateEvent,
    ) -> Option<Box<dyn PlayerState>>;
}
