use godot::{classes::AudioStream, prelude::*};

use crate::message::Message;

#[derive(GodotConvert, Default, Export, Var, PartialEq, Eq, Clone, Copy, Debug)]
#[godot(via = GString)]
pub enum AudioEffectType {
    #[default]
    None,
    SMALL,
    MEDUIM,
    LARGE,
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AudioTrigger {
    base: Base<Node>,
    #[export]
    #[init(val = None)]
    stream: Option<Gd<AudioStream>>,
    #[export]
    #[init(val = AudioEffectType::default())]
    effect_type: AudioEffectType,
}

#[godot_api]
impl INode for AudioTrigger {
    fn ready(&mut self) {
        if let Some(ref mut stream) = self.stream {
            Message::singleton()
                .signals()
                .play_music()
                .emit(&*stream, self.effect_type);
        }
    }
}
