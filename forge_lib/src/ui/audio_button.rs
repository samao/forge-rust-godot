use godot::{
    classes::{Button, IButton},
    prelude::*,
};

use crate::{managers::audio_manager::UIAudio, message::Message};

#[derive(GodotClass)]
#[class(base=Button, init)]
struct AudioButton {
    base: Base<Button>,
}

#[godot_api]
impl IButton for AudioButton {
    fn ready(&mut self) {
        self.base()
            .signals()
            .pressed()
            .connect_other(&*self, Self::on_selected);
        self.base()
            .signals()
            .mouse_entered()
            .connect_other(&*self, Self::on_hover_in);
    }
}

#[godot_api]
impl AudioButton {
    fn on_hover_in(&mut self) {
        Message::singleton()
            .signals()
            .play_ui_audio()
            .emit(UIAudio::Bloop);
    }
    fn on_selected(&mut self) {
        Message::singleton()
            .signals()
            .play_ui_audio()
            .emit(UIAudio::Select);
    }
}
