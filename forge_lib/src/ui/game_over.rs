use godot::{
    classes::{CanvasLayer, ICanvasLayer},
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::{
    managers::{save::SaveManager, scene_manager::SceneManager},
    message::Message,
    ui::audio_button::AudioButton,
};

#[derive(GodotClass)]
#[class(init, base = CanvasLayer)]
pub struct GameOver {
    base: Base<CanvasLayer>,
    #[init(node = "%Button")]
    back_button: OnReady<Gd<AudioButton>>,
    #[init(node = "%Restart")]
    restart_button: OnReady<Gd<AudioButton>>,
}

#[godot_api]
impl ICanvasLayer for GameOver {
    fn ready(&mut self) {
        Message::singleton()
            .signals()
            .game_over()
            .connect_other(&*self, Self::on_game_over);
        self.base_mut().set_visible(false);
        self.back_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::back_to_title);
        self.restart_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::on_restart);
    }
}

#[godot_api]
impl GameOver {
    fn on_game_over(&mut self) {
        self.base_mut().set_visible(true);
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
            self.base_mut().set_visible(false);
        }
    }

    fn on_restart(&mut self) {
        if let Ok(mut save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            save_helper.bind_mut().load_game();
            self.base_mut().set_visible(false);
        }
    }
}
