use godot::{classes::AudioStream, prelude::*};

use crate::managers::visual_effect::VisualEffectType;
use crate::resource::particles::HitParticleSetting;
use crate::{entities::audio_tirgger::AudioEffectType, managers::audio_manager::UIAudio};

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
    #[signal]
    pub fn play_effect(v_type: VisualEffectType, pos: Vector2);

    #[signal]
    pub fn play_particles(pos: Vector2, direction: Vector2, cfg: Gd<HitParticleSetting>);

    #[signal]
    pub fn camera_shake(strength: f64);
}
