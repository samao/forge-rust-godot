use godot::{classes::Engine, prelude::*};

use crate::monster::{blackboard::Blackboard, enemy::Enemy};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct DecisionEngine {
    base: Base<Node>,

    #[export]
    #[init(val = None)]
    enemy: Option<Gd<Enemy>>,

    #[allow(dead_code)]
    #[init(val = None)]
    current_state: Option<StringName>,

    #[init(val = None)]
    blackboard: Option<Gd<Blackboard>>,
}

impl DecisionEngine {
    pub(crate) fn setup(&mut self, blackboard: Gd<Blackboard>) {
        self.blackboard = Some(blackboard);
    }
    pub(crate) fn decide(&mut self) -> Option<StringName> {
        if let Some(board) = self.blackboard.as_ref() {
            if board.bind().get_damage_source().is_some() {
                if board.bind().get_health() <= 0.0 {
                    return Some("death".into());
                } else {
                    return Some("stun".into());
                }
            }
            let can_decide = board.bind().get_can_decide();
            match self.current_state {
                Some(ref a) if a.to_string() == "death".to_owned() => return None,
                _ if !can_decide => return None,
                _ => {}
            }
        }

        Some("walk".into())
    }
}

#[godot_api]
impl DecisionEngine {
    #[func]
    fn update_init_direction(&mut self) {
        if let Some(enemy) = self.enemy.as_mut() {
            let face_direction_left = enemy.bind().get_face_left_on_start();
            enemy.bind_mut().change_dir(if face_direction_left {
                Vector2::LEFT
            } else {
                Vector2::RIGHT
            });
        }
    }
}

#[godot_api]
impl INode for DecisionEngine {
    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }
        self.base_mut().call_deferred("update_init_direction", &[]);
    }
}
