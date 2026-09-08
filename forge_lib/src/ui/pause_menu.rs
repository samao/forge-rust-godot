use godot::{
    classes::{Button, CanvasLayer, Control, ICanvasLayer, InputEvent},
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::{message::Message, scene_manager::SceneManager};

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct PauseMenu {
    base: Base<CanvasLayer>,
    #[init(node = "%PauseScreen")]
    pause_screen: OnReady<Gd<Control>>,
    #[init(node = "%System")]
    system_screen: OnReady<Gd<Control>>,
    #[init(node = "%SystemButton")]
    system_button: OnReady<Gd<Button>>,
    #[init(node = "%BackToMapButton")]
    back_map_button: OnReady<Gd<Button>>,
    #[init(node = "%BackToTitleButton")]
    back_title_button: OnReady<Gd<Button>>,
}

#[godot_api]
impl ICanvasLayer for PauseMenu {
    fn ready(&mut self) {
        self.system_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::show_system);
        self.back_map_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::back_to_map);
        self.back_title_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::goto_to_menu);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("pause") {
            Message::singleton().signals().toggle_pause().emit();
            if let Some(mut view_port) = self.base().get_viewport() {
                view_port.set_input_as_handled();
            }
        }
    }

    fn exit_tree(&mut self) {
        godot_print!("游戏恢复");
        self.base().get_tree().set_pause(false);
    }

    fn enter_tree(&mut self) {
        godot_print!("游戏暂停");
        self.base().get_tree().set_pause(true);
    }
}
#[godot_api]
impl PauseMenu {
    fn show_system(&mut self) {
        self.pause_screen.set_visible(false);
        self.system_screen.set_visible(true);
        self.system_button.set_visible(false);
    }

    fn goto_to_menu(&mut self) {
        self.base_mut().call_deferred("back_to_title", &[]);
    }

    #[func]
    pub fn back_to_title(&mut self) {
        if let Ok(mut scene) = try_get_autoload_by_name::<SceneManager>("SceneTransition") {
            scene.bind_mut().travel_scene(
                "uid://cu7ofetipfdnf".into(),
                "".into(),
                crate::level_transition::Side::Top,
            );
            Message::singleton().signals().game_end().emit();
        }
    }
    fn back_to_map(&mut self) {
        self.pause_screen.set_visible(true);
        self.system_screen.set_visible(false);
        self.system_button.set_visible(true);
    }
}
