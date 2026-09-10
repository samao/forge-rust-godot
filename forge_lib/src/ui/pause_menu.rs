use godot::{
    classes::{AudioServer, Button, CanvasLayer, Control, HSlider, ICanvasLayer, InputEvent},
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::{managers::save::SaveManager, managers::scene_manager::SceneManager, message::Message};

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
    #[init(node = "%MusicSlider")]
    music_slider: OnReady<Gd<HSlider>>,
    #[init(node = "%SFXSlider")]
    sfx_slider: OnReady<Gd<HSlider>>,
    #[init(node = "%UiSlider")]
    ui_slider: OnReady<Gd<HSlider>>,
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
        self.init_slider();
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
        // godot_print!("游戏恢复");
        self.base().get_tree().set_pause(false);
    }

    fn enter_tree(&mut self) {
        // godot_print!("游戏暂停");
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

    fn init_slider(&mut self) {
        let (music_volume, sfx_volume, ui_volume) = self.get_volumes();
        self.music_slider.set_value(music_volume);
        self.music_slider
            .signals()
            .value_changed()
            .connect_other(&*self, Self::set_music_linear);
        self.sfx_slider.set_value(sfx_volume);
        self.sfx_slider
            .signals()
            .value_changed()
            .connect_other(&*self, Self::set_sfx_linear);
        self.ui_slider.set_value(ui_volume);
        self.ui_slider
            .signals()
            .value_changed()
            .connect_other(&*self, Self::set_ui_linear);
    }

    pub fn set_music_linear(&mut self, volume: f64) {
        AudioServer::singleton().set_bus_volume_linear(2, volume as f32);
        self.save_to_file();
    }

    pub fn set_sfx_linear(&mut self, volume: f64) {
        AudioServer::singleton().set_bus_volume_linear(3, volume as f32);
        self.save_to_file();
    }

    pub fn set_ui_linear(&mut self, volume: f64) {
        AudioServer::singleton().set_bus_volume_linear(4, volume as f32);
        self.save_to_file();
    }

    fn save_to_file(&self) {
        if let Ok(mut save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            save_helper.bind_mut().save_cfg();
        }
    }
    fn get_volumes(&self) -> (f64, f64, f64) {
        if let Ok(save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            return save_helper.bind().get_volume();
        }
        (0.5, 0.5, 0.5)
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
