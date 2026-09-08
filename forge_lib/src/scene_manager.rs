use godot::{
    classes::{CanvasLayer, Control, Engine, ICanvasLayer, ResourceUid, notify::NodeNotification},
    obj::WithBaseField,
    prelude::*,
};

use crate::{AsyncHandle, level_transition::Side, message::Message, ui::pause_menu::PauseMenu};

#[derive(GodotClass)]
#[class(init, base = CanvasLayer)]
pub struct SceneManager {
    base: Base<CanvasLayer>,

    #[init(node = "%Fade")]
    #[var(pub)]
    pub fade: OnReady<Gd<Control>>,

    #[init(val = "uid://bowrs0wcn2oy6".into())]
    pub current_scene_path_uid: GString,

    #[init(val = None)]
    pause_menu: Option<Gd<PauseMenu>>,
}

#[godot_api]
impl ICanvasLayer for SceneManager {
    fn ready(&mut self) {
        self.base_mut().call_deferred("init_scene", &[]);
        if let Some(ref mut tree) = self.base().get_tree_or_null() {
            tree.set_auto_accept_quit(false);
        }

        Message::singleton()
            .signals()
            .toggle_pause()
            .connect_other(&*self, Self::on_toggle_pause);
        Message::singleton()
            .signals()
            .game_end()
            .connect_other(&*self, Self::on_game_end);
    }
    fn on_notification(&mut self, what: NodeNotification) {
        match what {
            NodeNotification::WM_CLOSE_REQUEST => {
                AsyncHandle::singleton().bind_mut().terminate();
                self.base().get_tree().quit();
            }
            _ => {}
        }
    }
}

#[godot_api]
impl SceneManager {
    #[signal]
    pub fn load_scene_started();
    #[signal]
    pub fn new_scene_ready(target_name: String, dir: Side);
    #[signal]
    pub fn load_scene_finished();
    #[signal]
    pub fn scene_entry(path: GString);
    #[signal]
    pub fn scene_exit(path: GString);

    #[func]
    fn init_scene(&mut self) {
        let path = self
            .base()
            .get_tree()
            .get_current_scene()
            .unwrap()
            .get_scene_file_path();
        // godot_print!("initiate scene {}", path);
        let scene_uid = ResourceUid::path_to_uid(&path);
        self.current_scene_path_uid = scene_uid.clone();
        self.signals().scene_entry().emit(&scene_uid);
        self.signals().load_scene_finished().emit();
    }

    pub fn get_current_path(&self) -> String {
        self.current_scene_path_uid.to_string()
    }

    pub fn on_toggle_pause(&mut self) {
        if let Some(loop_main) = Engine::singleton().get_main_loop()
            && let Ok(tree) = loop_main.try_cast::<SceneTree>()
        {
            if let Some(menu) = self.pause_menu.take() {
                if menu.is_inside_tree() {
                    godot_print!("dismiss");
                    if let Some(mut parent) = menu.get_parent() {
                        // menu.get_parent().unwrap().remove_child(&menu);
                        parent.call_deferred("remove_child", &[menu.to_variant()]);
                    } else {
                        godot_print!("没有腹肌");
                    }
                } else {
                    if let Some(mut root) = tree.get_root() {
                        root.add_child(&menu);
                    }
                }
                self.pause_menu = Some(menu);
            } else {
                if let Ok(scene) = try_load::<PackedScene>("uid://eedmxm0nkckg") {
                    let pause_menu = scene.instantiate_as::<PauseMenu>();
                    if let Some(mut root) = tree.get_root() {
                        root.add_child(&pause_menu);
                    }
                    self.pause_menu = Some(pause_menu);
                }
            }
        }
    }

    fn on_game_end(&mut self) {
        if let Some(menu) = &self.pause_menu {
            if menu.is_inside_tree() {
                self.on_toggle_pause();
            }
        }
    }

    #[func]
    pub fn travel_scene(&mut self, scene_path: String, target_name: String, dir: Side) {
        let origin_path = self.current_scene_path_uid.clone();
        let scene_uid = ResourceUid::path_to_uid(&scene_path);
        self.current_scene_path_uid = scene_uid.clone();
        let this = self.to_gd();
        let fade_pos = self.get_fade_position(&dir);
        let mut _guard = self.to_gd();

        godot::task::spawn(async move {
            if let Some(main_loop_instance) = Engine::singleton().get_main_loop()
                && let Ok(mut tree) = main_loop_instance.try_cast::<SceneTree>()
            {
                // let mut tree = this.get_tree();
                tree.signals().process_frame().to_future().await;
                this.signals().load_scene_started().emit();
                Self::set_fade_visiable(this.clone(), true);
                let tween = Self::fade_screen_ex(this.clone(), fade_pos, Vector2::ZERO);
                tween.signals().finished().to_future().await;
                this.signals().scene_exit().emit(&origin_path);
                tree.change_scene_to_file(&scene_uid);
                this.signals().scene_entry().emit(&scene_uid);
                tree.signals().scene_changed().to_future().await;
                this.signals().new_scene_ready().emit(target_name, dir);
                let tween = Self::fade_screen_ex(this.clone(), Vector2::ZERO, fade_pos);
                tween.signals().finished().to_future().await;
                this.signals().load_scene_finished().emit();
                Self::set_fade_visiable(this, false);
            }
        });
    }

    fn set_fade_visiable(mut this: Gd<Self>, v: bool) {
        this.bind_mut().fade.set_visible(v);
    }

    fn fade_screen_ex(mut this: Gd<Self>, from: Vector2, to: Vector2) -> Gd<godot::classes::Tween> {
        this.bind_mut().fade.set_visible(true);
        this.bind_mut().fade.set_global_position(from);
        let mut tween = this.create_tween();
        tween.tween_property(
            this.bind().fade.to_godot(),
            "position",
            &to.to_variant(),
            0.2,
        );
        return tween;
    }

    #[func]
    pub fn transition_scene(&mut self, new_scene: String, target_area: String, dir: Side) {
        self.travel_scene(new_scene, target_area, dir);
    }

    fn get_fade_position(&self, dir: &Side) -> Vector2 {
        let direction = match dir {
            Side::Left => Vector2::LEFT,
            Side::Right => Vector2::RIGHT,
            Side::Top => Vector2::UP,
            Side::Bottom => Vector2::DOWN,
        };
        direction * Vector2::new(480.0, 270.0)
    }
}
