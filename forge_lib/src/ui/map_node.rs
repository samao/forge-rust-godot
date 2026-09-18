use godot::{
    classes::{
        ColorRect, Control, Engine, IControl, Label, ResourceLoader, ResourceUid, Sprite2D,
        node::ProcessMode, notify::ControlNotification,
    },
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::{
    level_bounds::LevelBounds,
    level_transition::{LevelTransition, Side},
    managers::scene_manager::SceneManager,
    player::Player,
};

#[derive(GodotClass)]
#[class(tool, init, base=Control)]
pub struct MapNode {
    base: Base<Control>,
    #[export(file = "*.tscn")]
    scene: GString,
    #[export_tool_button(fn = Self::update, name="手动更新")]
    update: PhantomVar<Callable>,
    blocks: Option<Gd<Control>>,
    inditor: Option<Gd<Sprite2D>>,
    #[init(val = Vector2::new(480.0, 270.0))]
    bound_size: Vector2,
    #[init(val = Vector2::ZERO)]
    bound_pos: Vector2,
    #[init(val = 0.0)]
    time_gap: f64,
}

const FACTOR: f32 = 12.0;

#[godot_api]
impl IControl for MapNode {
    fn ready(&mut self) {
        self.blocks = self.base().try_get_node_as::<Control>("%Blocks");
        self.inditor = match self.base().try_get_node_as::<Sprite2D>("%Inditor") {
            Some(mut node) => {
                node.set_visible(false);
                Some(node)
            }
            _ => None,
        };
        if Engine::singleton().is_editor_hint() {
            return;
        }
        self.base().get_node_as::<Label>("%Label").queue_free();
        // self.update();

        self.listener_scene_change();
    }
    fn on_notification(&mut self, what: ControlNotification) {
        match what {
            ControlNotification::EXTENSION_RELOADED => {
                self.blocks = self.base().try_get_node_as::<Control>("%Blocks");
                self.inditor = self.base().try_get_node_as::<Sprite2D>("%Inditor");
            }
            ControlNotification::ENTER_TREE => {
                if Engine::singleton().is_editor_hint() {
                    return;
                }
                let scene_path = self
                    .base()
                    .get_tree()
                    .get_current_scene()
                    .unwrap()
                    .get_scene_file_path();
                let scene_uid = ResourceUid::path_to_uid(&scene_path);
                self.on_player_enter(scene_uid);
            }
            _ => {}
        }
    }

    fn process(&mut self, _delta: f64) {
        self.time_gap += _delta;
        if self.time_gap < 0.2 {
            return;
        }
        self.time_gap = 0.0;
        if let Some(player_node) = self.base().get_tree().get_first_node_in_group("Player")
            && let Ok(player) = player_node.try_cast::<Player>()
        {
            let player_pos = player.get_global_position();
            let view_pos = self.bound_pos;
            let place_pos = player_pos - view_pos;
            if let Some(ref mut inditor) = self.inditor {
                // godot_print!("紧跟: {}", place_pos);
                inditor.set_position(Vector2::new(
                    (place_pos.x / FACTOR).round(),
                    (place_pos.y / FACTOR).round(),
                ));
            }
        }
    }
}

#[godot_api]
impl MapNode {
    fn on_player_enter(&mut self, scene_id: GString) {
        if scene_id == self.scene {
            //进入当前场景
            // godot_print!("进入场景MapNode: {}", scene_id);
            self.base_mut()
                .call_deferred("set_inditor_visible", &[true.to_variant()]);
            self.base_mut().set_process_mode(ProcessMode::INHERIT);
        }
    }

    fn on_player_exit(&mut self, scene_id: GString) {
        if scene_id == self.scene {
            //进入当前场景
            // godot_print!("退出景MapNode: {}", scene_id);
            self.set_inditor_visible(false);
            self.base_mut().set_process_mode(ProcessMode::DISABLED);
        }
    }

    fn listener_scene_change(&mut self) {
        if let Ok(transition) = try_get_autoload_by_name::<SceneManager>("SceneTransition") {
            transition
                .signals()
                .scene_entry()
                .connect_other(&*self, Self::on_player_enter);
            transition
                .signals()
                .scene_exit()
                .connect_other(&*self, Self::on_player_exit);
        }
    }

    #[func]
    pub fn set_inditor_visible(&mut self, v: bool) {
        if let Some(ref mut inditor) = self.inditor {
            inditor.set_visible(v);
        } else {
            godot_print!("没有提示器");
        }
    }

    #[func]
    pub fn update(&mut self) {
        // godot_print!("更新它: {}", self.scene);
        self.clear_blocks();
        if let Some(scene) = ResourceLoader::singleton().load(&self.scene) {
            if let Ok(scene_packed) = scene.try_cast::<PackedScene>() {
                if let Some(mut scene_instance) = scene_packed.try_instantiate_as::<Node2D>() {
                    let scene_path = scene_instance.get_scene_file_path();
                    self.set_lable_text(scene_path.clone());

                    if let Some(level_bound) =
                        scene_instance.try_get_node_as::<LevelBounds>("LevelBounds")
                    {
                        let (x, y, width, height) = (
                            level_bound.get_position().x,
                            level_bound.get_position().y,
                            level_bound.bind().get_width(),
                            level_bound.bind().get_height(),
                        );
                        // godot_print!(
                        //     "场景<{}>: ({}, {}), size: ({}, {})",
                        //     scene_path,
                        //     x,
                        //     y,
                        //     width,
                        //     height
                        // );
                        self.bound_pos.x = x;
                        self.bound_pos.y = y;
                        self.set_size(width, height);

                        let nodes_in_map = scene_instance
                            .find_children_ex("LevelTransition*")
                            .type_("LevelTransition")
                            // .recursive(true)
                            .done();
                        let mut blocks_to_add = Vec::new();
                        for node in nodes_in_map.iter_shared() {
                            if let Ok(transition) = node.try_cast::<LevelTransition>() {
                                let trans_pos = transition.get_position();
                                // godot_print!(
                                //     "传输点{}: ({}, {})",
                                //     transition.get_name(),
                                //     trans_pos.x,
                                //     trans_pos.y
                                // );

                                let mut pos = trans_pos - Vector2::new(x, y);
                                pos.x = pos.x / FACTOR;
                                pos.y = pos.y / FACTOR;
                                // godot_print!("设置位置: {:?}", pos);
                                let mut block = self.add_block(transition.bind().get_location());
                                block.set_position(pos);
                                blocks_to_add.push(block);
                            }
                        }
                        if let Some(ref mut container) = self.blocks {
                            for node in blocks_to_add {
                                container.add_child(&node);
                            }
                        }
                        scene_instance.queue_free();
                    } else {
                        godot_print!("没有shu");
                    }
                }
            }
        }
    }

    fn add_block(&mut self, _location: Side) -> Gd<ColorRect> {
        let mut block = ColorRect::new_alloc();
        block.set_custom_minimum_size(Vector2 { x: 4.0, y: 4.0 });
        block.set_modulate(Color::RED);
        block.set_pivot_offset_ratio(Vector2::splat(0.5));
        block
    }

    fn clear_blocks(&mut self) {
        if let Some(ref mut container) = self.blocks {
            for mut block in container.get_children().iter_shared() {
                block.queue_free();
            }
        }
    }

    fn set_size(&mut self, width: f32, height: f32) {
        self.bound_size.x = width;
        self.bound_size.y = height;
        self.base_mut().set_size(Vector2::new(
            (width / FACTOR).round(),
            (height / FACTOR).round(),
        ));
    }

    fn set_lable_text(&mut self, msg: GString) {
        if let Some(mut label) = self.base_mut().try_get_node_as::<Label>("%Label") {
            let text = msg.replace("res://levels/", "").replace(".tscn", "");
            label.set_text(&text);
        }
    }
}
