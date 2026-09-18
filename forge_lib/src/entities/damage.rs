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
    #[export]
    #[init(val = Vector2::ZERO)]
    offset: Vector2,
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
                let attack_pos = self.base().get_global_position() + self.offset;
                let pos = attack_area.get_global_position().direction_to(attack_pos);
                if parent.has_method("take_damage") {
                    if let Err(msg) = parent.try_call_deferred(
                        "take_damage",
                        &[
                            attack_pos.to_variant(),
                            pos.to_variant(),
                            attack_area.to_variant(),
                        ],
                    ) {
                        godot_print!("{:?}", msg);
                    }
                }

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
                    .emit(VisualEffectType::Hit, pos + self.offset);
            }
            // self.base_mut()
            //     .call_deferred("make_invulnerable", &[1.0.to_variant()]);
        }
    }

    #[func]
    pub fn make_invulnerable(&mut self, duration: f64) {
        self.base_mut().set_process_mode(ProcessMode::DISABLED);
        godot_print!("无敌了{duration}");
        self.base().get_tree().create_timer(duration).connect_flags(
            "timeout",
            &Callable::from_object_method(&self.to_gd(), "recovery_invulnerable"),
            ConnectFlags::ONE_SHOT,
        );
    }

    #[func]
    fn recovery_invulnerable(&mut self) {
        godot_print!("BU无敌了");
        self.base_mut().set_process_mode(ProcessMode::INHERIT);
    }
}
