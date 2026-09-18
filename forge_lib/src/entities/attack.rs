use godot::{
    classes::{Area2D, AudioStream, IArea2D},
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::managers::audio_manager::AudioManager;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct AttackArea {
    base: Base<Area2D>,
    #[export]
    #[init(val = 1.0)]
    #[var(pub)]
    damage: f32,
    #[export]
    audio: Option<Gd<AudioStream>>,
}

#[godot_api]
impl IArea2D for AttackArea {
    fn ready(&mut self) {
        // self.base_mut().set_monitoring(false);
        // self.set_active(false);
    }
}

#[godot_api]
impl AttackArea {
    pub fn set_active(&mut self, v: bool) {
        self.base_mut().set_monitorable(v);
        self.base_mut().set_visible(v);

        if let Some(audio) = self.audio.clone()
            && v
        {
            if let Ok(mut audio_helper) = try_get_autoload_by_name::<AudioManager>("AudioHelper") {
                let pos = self.base().get_global_position();
                audio_helper.bind_mut().play_spatial_sound(audio, pos);
            }
        }
    }
}
