use crate::states::PlayerState;

pub struct DieState {}

impl DieState {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlayerState for DieState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        player.set_horizontal_speed(0.0);
        player.play_anim("die");
    }

    fn exit(&mut self, _player: &mut crate::player::Player) {}

    fn handle_event(
        &mut self,
        _player: &mut crate::player::Player,
        _event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        None
    }
}
