use godot::prelude::*;

use crate::monster::{
    blackboard::Blackboard, enemy::Enemy, enemy_state_machine::EnemyStateMachine,
};

#[allow(dead_code)]
pub(super) trait IStateTrait {
    fn base_node(&mut self) -> Gd<Node>;
    //从状态里面拿状态机
    fn get_state_machine(&mut self) -> Option<Gd<EnemyStateMachine>> {
        self.base_node()
            .get_parent()
            .and_then(|p| p.try_cast::<EnemyStateMachine>().ok())
    }
    //获取代理的节点
    fn get_owner_node(&mut self) -> Option<Gd<Enemy>> {
        self.get_state_machine().and_then(|sm| {
            sm.get_owner()
                .and_then(|node| node.try_cast::<Enemy>().ok())
        })
    }

    fn get_current_animation_length(&mut self) -> f64 {
        self.get_owner_node()
            .map_or(0.0, |e| e.bind().get_current_animation_length())
    }

    fn get_blackboard(&mut self) -> Option<Gd<Blackboard>> {
        self.get_state_machine()
            .and_then(|sm| sm.bind().get_blackboard())
    }

    fn change_state(&mut self, name: &str) {
        if let Some(mut sm) = self.get_state_machine() {
            sm.bind_mut().change_state(name.into());
        }
    }

    fn play_animation(&mut self, anim_name: &str) {
        if let Some(mut sm) = self.get_state_machine() {
            sm.bind_mut().play_animation(anim_name);
        }
    }
}
