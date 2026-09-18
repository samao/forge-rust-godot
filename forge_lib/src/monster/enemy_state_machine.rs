use std::collections::HashMap;

use godot::{classes::InputEvent, obj::WithBaseField, prelude::*};

use crate::monster::{blackboard::Blackboard, enemy::Enemy};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct EnemyStateMachine {
    base: Base<Node>,

    #[export]
    #[init(val = None)]
    enemy: Option<Gd<Enemy>>,

    #[init(val = None)]
    blackboard: Option<Gd<Blackboard>>,

    #[init(val = HashMap::new())]
    states: HashMap<StringName, Gd<Node>>,

    #[init(val = None)]
    current_state: Option<Gd<Node>>,
    #[init(val = None)]
    prev_state: Option<Gd<Node>>,

    #[export]
    init_state: StringName,
}

impl EnemyStateMachine {
    pub(super) fn setup(&mut self, b: Gd<Blackboard>) {
        // godot_print!("setup ESM");
        for child in self.base().get_children().iter_shared() {
            if child.is_in_group("state") {
                let state_name = child.get_name();
                self.states.insert(state_name.clone(), child.clone());
            }
        }

        self.blackboard = Some(b);

        let init_name = self.init_state.clone();

        if !init_name.is_empty() {
            self.change_state(init_name);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_blackboard(&self) -> Option<Gd<Blackboard>> {
        self.blackboard.clone()
    }

    //切换状态
    pub(crate) fn change_state(&mut self, state_name: StringName) {
        if !self.states.contains_key(&state_name) {
            return;
        }

        if state_name == self.get_current_state_name() {
            self.re_enter_state();
            return;
        }
        if let Some(mut prev_state) = self.current_state.take() {
            if prev_state.has_method("exit") {
                prev_state.call_deferred("exit", &[]);
                self.prev_state = Some(prev_state);
            } else {
                // godot_warn!("节点上不存在方法：exit");
            }
        }

        if let Some(mut new_state) = self.states.get(&state_name).cloned() {
            // let mut new_state_clone = new_state.clone();
            if new_state.has_method("enter") {
                new_state.call_deferred("enter", &[]);
            } else {
                // godot_warn!("节点上不存在方法：enter");
            }
            self.current_state = Some(new_state);
        }
    }

    fn re_enter_state(&mut self) {
        if let Some(mut current_state) = self.current_state.take() {
            if current_state.has_method("re_enter") {
                current_state.call_deferred("re_enter", &[]);
            } else {
                // godot_warn!("节点上不存在方法：re_enter");
            }
            self.current_state = Some(current_state);
        }
    }

    pub(crate) fn get_current_state_name(&self) -> StringName {
        self.current_state
            .as_ref()
            .map(|s| s.get_name())
            .unwrap_or_default()
    }

    pub(crate) fn play_animation(&mut self, anim_name: &str) {
        if let Some(enemy) = self.enemy.as_mut() {
            enemy.bind_mut().play_animation(anim_name);
        }
    }
}

#[godot_api]
impl INode for EnemyStateMachine {
    fn physics_process(&mut self, delta: f64) {
        if let Some(current_state) = self.current_state.clone().as_mut() {
            if current_state.has_method("update") {
                current_state.call_deferred("update", &[delta.to_variant()]);
            } else {
                // godot_warn!("节点上不存在方法： update");
            }
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Some(current_state) = self.current_state.clone().as_mut() {
            if current_state.has_method("handle_input") {
                current_state.call_deferred("handle_input", &[event.to_variant()]);
            } else {
                // godot_warn!("节点上不存在方法： handle_input");
            }
        }
    }
}
