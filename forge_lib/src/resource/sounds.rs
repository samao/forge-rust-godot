use godot::{classes::AudioStream, prelude::*};

#[derive(GodotClass, Debug)]
#[class(base = Resource, init)]
pub struct SoundSource {
    base: Base<Resource>,
    #[export]
    #[init(val = None)]
    pub attack: Option<Gd<AudioStream>>,
    #[init(val = None)]
    #[export]
    pub jump: Option<Gd<AudioStream>>,
    #[init(val = None)]
    #[export]
    pub land: Option<Gd<AudioStream>>,
    #[export]
    #[init(val = None)]
    pub dash: Option<Gd<AudioStream>>,
}
