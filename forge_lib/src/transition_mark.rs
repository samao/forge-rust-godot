use godot::classes::node::ProcessMode;
use godot::classes::{Engine, IMarker2D, Label, Marker2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::level_transition::Side;
use crate::managers::scene_manager::SceneManager;

#[derive(GodotClass)]
#[class(init, tool, base = Marker2D)]
pub struct TransitionMark2D {
    #[export]
    #[var(set, get)]
    target_area_name: GString,
    base: Base<Marker2D>,

    #[export]
    #[init(val = false)]
    align_origin: bool,
}

#[godot_api]
impl IMarker2D for TransitionMark2D {
    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.base_mut().call_deferred("clear_debug_indicator", &[]);
        if let Some(transition) = self
            .base()
            .try_get_node_as::<SceneManager>("/root/SceneTransition")
        {
            transition
                .signals()
                .new_scene_ready()
                .connect_other(&*self, Self::on_new_scene_ready);
        }

        self.base_mut().set_visible(false);
    }
}

#[godot_api]
impl TransitionMark2D {
    fn on_new_scene_ready(&mut self, target_name: String, dir: Side) {
        if target_name != self.target_area_name.to_string() {
            return;
        }
        if let Some(player) = self.base().get_tree().get_first_node_in_group("Player") {
            if let Ok(mut player) = player.try_cast::<Node2D>() {
                let pos = self.base().get_global_position();
                godot_print!("放置跳转位置: {}", pos);
                let place_pos = self.get_offset(pos, &player, dir);
                player.set_global_position(place_pos);
                player.set_visible(true);
                player.set_process_mode(ProcessMode::INHERIT);
            }
        }
    }

    fn get_offset(&self, marker_pos: Vector2, player: &Gd<Node2D>, dir: Side) -> Vector2 {
        if self.align_origin {
            return marker_pos;
        }
        let player_pos = player.get_global_position();
        let mut offset = Vector2::ZERO;
        match dir {
            Side::Left | Side::Right => {
                offset.y = player_pos.y;
                offset.x = marker_pos.x;
            }
            Side::Top | Side::Bottom => {
                offset.x = player_pos.x;
                offset.y = marker_pos.y;
            }
        }
        offset
    }

    #[func]
    fn clear_debug_indicator(&mut self) {
        let children = self.base().get_children();
        for mut node in children.iter_shared() {
            node.queue_free();
        }
    }

    #[func]
    pub fn get_target_area_name(&self) -> GString {
        self.target_area_name.clone()
    }

    #[func]
    fn set_target_area_name(&mut self, v: GString) {
        self.target_area_name = v.clone();
        if let Some(mut label) = self.base().try_get_node_as::<Label>("Label") {
            label.set_text(&v.to_string());
        }
    }
}
