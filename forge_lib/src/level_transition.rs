use godot::{
    classes::{Area2D, Engine, Sprite2D, node::ProcessMode, notify::CanvasItemNotification},
    obj::WithBaseField,
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::managers::scene_manager::SceneManager;

#[derive(Debug, GodotConvert, Var, Export, Default, Clone, PartialEq, Eq, Copy)]
#[godot(via = GString)]
pub enum Side {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(GodotClass, Debug)]
#[class(init, tool, base = Node2D)]
pub struct LevelTransition {
    base: Base<Node2D>,

    #[export(range = (2.0, 16.0, 1.0, or_greater))]
    #[var(set)]
    #[init(val = 2)]
    size: i32,

    #[export]
    #[var(set, get)]
    #[init(val = Side::Left)]
    location: Side,

    #[export(file = "*.tscn")]
    #[init(val = "".into())]
    target_level: GString,

    #[export]
    #[init(val = "".into())]
    target_area_name: GString,

    #[init(val = None)]
    area_2d: Option<Gd<Area2D>>,
    #[init(val = None)]
    debug_texture: Option<Gd<Sprite2D>>,
    #[init(val = None)]
    transition: Option<Gd<SceneManager>>,
}

#[godot_api]
impl INode2D for LevelTransition {
    fn ready(&mut self) {
        self.area_2d = self.base().try_get_node_as::<Area2D>("%Area2D");
        self.debug_texture = self.base().try_get_node_as::<Sprite2D>("%Sprite2D");
        self.apply_area_setting();
        if Engine::singleton().is_editor_hint() {
            return;
        }
        self.transition =
            if let Ok(transition) = try_get_autoload_by_name::<SceneManager>("SceneTransition") {
                transition
                    .signals()
                    .load_scene_finished()
                    .connect_other(&*self, Self::on_scene_loaded);

                Some(transition)
            } else {
                None
            };

        if let Some(mut debug_texture) = self.debug_texture.take() {
            debug_texture.call_deferred("queue_free", &[]);
        }
        self.base_mut().set_visible(false);
    }

    fn on_notification(&mut self, what: CanvasItemNotification) {
        match what {
            CanvasItemNotification::EXTENSION_RELOADED => {
                if self.base().get_tree_or_null().is_some() {
                    self.area_2d = self.base().try_get_node_as::<Area2D>("%Area2D");
                    self.debug_texture = self.base().try_get_node_as::<Sprite2D>("%Sprite2D");
                }
            }
            _ => {}
        }
    }
}

#[godot_api]
impl LevelTransition {
    fn on_player_enter(&mut self, mut player: Gd<Node2D>) {
        if let Some(mut area2d) = self.area_2d.clone() {
            area2d.call_deferred("set_monitoring", &[false.to_variant()]);
            godot_print!("player is enter.");
            player.set_visible(false);
            player.set_process_mode(ProcessMode::DISABLED);
            if let Some(ref mut transition) = self.transition {
                transition.bind_mut().transition_scene(
                    self.target_level.clone().into(),
                    self.target_area_name.clone().into(),
                    self.location,
                );
            }
            area2d.call_deferred("set_monitoring", &[true.to_variant()]);
        }
    }

    fn on_scene_loaded(&mut self) {
        if let Some(area_2d) = &self.area_2d {
            // godot_print!("on scene loaded connecting");
            area_2d
                .signals()
                .body_entered()
                .connect_other(&*self, Self::on_player_enter);
        }
    }

    #[func]
    fn set_size(&mut self, v: i32) {
        self.size = v;
        self.apply_area_setting();
    }

    #[func]
    fn set_location(&mut self, v: Side) {
        self.location = v;
        self.apply_area_setting();
    }

    #[func]
    pub fn get_location(&self) -> Side {
        self.location
    }

    #[func]
    fn apply_area_setting(&mut self) {
        let location = &self.location;
        let size = self.size as f32;
        // godot_print!("更新UI: {:?} -> {}", location, size);
        let scale = match location {
            Side::Left => Vector2::new(1.0, -size),
            Side::Right => Vector2::new(-1.0, -size),
            Side::Bottom => Vector2::new(-size, -1.0),
            Side::Top => Vector2::new(-size, 1.0),
        };
        // if let Some(mut area_2d) = self.area_2d.clone() {
        // godot_print!("更新了吧.");
        self.base_mut().set_scale(scale);
        // }

        // if let Some(mut texture) = self.debug_texture.clone() {
        //     // godot_print!("更新了 react");
        //     texture.set_scale(scale);
        // }
    }
}
