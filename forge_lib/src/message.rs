use godot::{classes::AudioStream, prelude::*};

use crate::{audio_manager::UIAudio, entities::audio_tirgger::AudioEffectType};

#[derive(GodotClass)]
#[class(singleton,init, base = Object)]
pub struct Message {
    base: Base<Object>,
}

#[godot_api]
impl Message {
    #[signal]
    pub fn interactive();
    #[signal]
    pub fn input_hint_change(name: String);
    #[signal]
    pub fn player_health_change(hp: f32, max_hp: f32);

    #[signal]
    pub fn toggle_pause();
    #[signal]
    pub fn game_end();

    #[signal]
    pub fn play_music(audio: Gd<AudioStream>, effect: AudioEffectType);
    #[signal]
    pub fn play_ui_audio(audio_type: UIAudio);
}
