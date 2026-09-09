use godot::classes::{AnimationPlayer, Sprite2D};
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::signal::ConnectHandle;
use godot::tools::try_get_autoload_by_name;

use crate::audio_manager::UIAudio;
use crate::entities::interactive::Interactive;
use crate::managers::save::SaveManager;
use crate::message::Message;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
struct SavePoint {
    base: Base<Node2D>,
    #[init(node = "AnimationPlayer")]
    animation: OnReady<Gd<AnimationPlayer>>,
    #[init(node = "Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,
    #[init(node = "Interactive")]
    interactive: OnReady<Gd<Interactive>>,

    #[init(val = None)]
    message_handle: Option<ConnectHandle>,
}

#[godot_api]
impl INode2D for SavePoint {
    fn ready(&mut self) {
        self.interactive
            .signals()
            .active()
            .connect_other(&*self, Self::on_active);
        self.interactive
            .signals()
            .deactive()
            .connect_other(&*self, Self::on_deactive);
    }
}

#[godot_api]
impl SavePoint {
    fn on_active(&mut self) {
        // godot_print!("active");
        let this = self.to_gd().clone();
        self.message_handle = Some(
            Message::singleton()
                .signals()
                .interactive()
                .connect_other(&this, Self::on_interactive),
        );
        Message::singleton()
            .signals()
            .input_hint_change()
            .emit("interact".to_owned());
    }

    fn on_deactive(&mut self) {
        // godot_print!("deactive");
        if let Some(handle) = self.message_handle.take() {
            handle.disconnect();
        }
        Message::singleton()
            .signals()
            .input_hint_change()
            .emit("".to_owned())
    }

    fn on_interactive(&mut self) {
        self.animation.play_ex().name("saved").done();
        if let Ok(mut save_handle) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            Message::singleton()
                .signals()
                .play_ui_audio()
                .emit(UIAudio::Success);
            save_handle.call_deferred("save_game", &[]);
        }
    }
}
