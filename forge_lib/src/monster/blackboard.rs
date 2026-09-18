use godot::{classes::Resource, prelude::*};

use crate::{entities::attack::AttackArea, player::Player};

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub(crate) struct Blackboard {
    base: Base<Resource>,

    #[init(val = 3.0)]
    #[var(pub)]
    health: f32,

    #[init(val = None)]
    #[var(pub)]
    target: Option<Gd<Player>>,

    #[init(val = f32::MIN)]
    #[var(pub)]
    distance_to_target: f32,

    #[init(val = true)]
    #[var(pub)]
    can_decide: bool,

    #[init(val = false)]
    #[var(pub)]
    edge_detected: bool,

    #[init(val = None)]
    #[var(pub)]
    damage_source: Option<Gd<AttackArea>>,

    #[init(val = Vector2::RIGHT)]
    #[var(pub)]
    dir: Vector2,
}

impl Blackboard {
    #[allow(dead_code)]
    pub fn update_distance_to_target(&mut self, pos: Vector2) {
        self.distance_to_target = self
            .target
            .as_ref()
            .map_or(f32::MIN, |node| pos.distance_to(node.get_global_position()));
    }
}
