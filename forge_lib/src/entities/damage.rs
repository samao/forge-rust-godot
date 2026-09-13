use godot::{
    classes::{Area2D, AudioStream, IArea2D, node::ProcessMode, object::ConnectFlags},
    obj::WithBaseField,
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::{
    entities::attack::AttackArea,
    managers::{audio_manager::AudioManager, visual_effect::VisualEffectType},
    message::Message,
    resource::particles::HitParticleSetting,
};

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct DamageArea {
    base: Base<Area2D>,

    #[export]
    audio: Option<Gd<AudioStream>>,

    #[init(load = "uid://cpi7sdopbcndq")]
    hit_particle: OnReady<Gd<HitParticleSetting>>,
}

#[godot_api]
impl IArea2D for DamageArea {
    fn ready(&mut self) {
        self.signals()
            .area_entered()
            .connect_self(Self::on_attacked);
    }
}

#[godot_api]
impl DamageArea {
    fn on_attacked(&mut self, area: Gd<Area2D>) {
        if let Ok(attack_area) = area.try_cast::<AttackArea>() {
            if let Some(ref mut parent) = self.base().get_parent() {
                // godot_print!("{:?}", parent);
                parent.call_deferred(
                    "take_damage",
                    &[attack_area.bind().get_damage().to_variant()],
                );

                if let Some(audio) = self.audio.clone() {
                    if let Ok(mut audio_helper) =
                        try_get_autoload_by_name::<AudioManager>("AudioHelper")
                    {
                        let pos = self.base().get_global_position();
                        audio_helper.bind_mut().play_spatial_sound(audio, pos);
                    }
                }

                let pos = self.base().get_global_position();
                Message::singleton()
                    .signals()
                    .play_effect()
                    .emit(VisualEffectType::Hit, pos);
            }
        }
    }

    pub fn make_invulnerable(&mut self, duration: Option<f64>) {
        let duration = duration.unwrap_or(1.0);
        self.base_mut().set_process_mode(ProcessMode::DISABLED);
        self.base().get_tree().create_timer(duration).connect_flags(
            "timeout",
            &Callable::from_object_method(&self.to_gd(), "recovery_invulnerable"),
            ConnectFlags::ONE_SHOT,
        );
    }

    #[func]
    fn recovery_invulnerable(&mut self) {
        self.base_mut().set_process_mode(ProcessMode::INHERIT);
    }
}
