use godot::prelude::*;

use crate::{
    entities::{dust_effect::DustEffect, hit_particle::HitParticle},
    message::Message,
    resource::particles::HitParticleSetting,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct VisualEffect {
    base: Base<Node>,
    #[init(load = "uid://dc10k5s7ltt0")]
    dust_effect_scene: OnReady<Gd<PackedScene>>,
    #[init(load = "uid://6aqi4g8mrdog")]
    hit_particle_scene: OnReady<Gd<PackedScene>>,
}

#[derive(GodotConvert, Debug, Clone, Copy, PartialEq, Eq)]
#[godot(via = GString)]
pub enum VisualEffectType {
    Jump,
    Land,
    Hit,
}

#[godot_api]
impl INode for VisualEffect {
    fn ready(&mut self) {
        godot_print!("好了，等待激发");
        Message::singleton()
            .signals()
            .play_effect()
            .connect_other(&*self, Self::on_play_effect);
        Message::singleton()
            .signals()
            .play_particles()
            .connect_other(&*self, Self::play_particle);
    }
}

#[godot_api]
impl VisualEffect {
    fn on_play_effect(&mut self, v_type: VisualEffectType, pos: Vector2) {
        godot_print!("播放特效: {:?} @ {:?}", v_type, pos);
        let anim_name = match v_type {
            VisualEffectType::Jump => "jump",
            VisualEffectType::Land => "land",
            VisualEffectType::Hit => "hit",
        };
        let mut dust_effect = self.dust_effect_scene.instantiate_as::<DustEffect>();
        dust_effect.set_global_position(pos);
        self.base_mut().add_child(&dust_effect);
        dust_effect.bind_mut().play(anim_name);
    }

    fn play_particle(&mut self, pos: Vector2, dir: Vector2, cfg: Gd<HitParticleSetting>) {
        let mut particle = self.hit_particle_scene.instantiate_as::<HitParticle>();
        particle.set_global_position(pos);
        self.base_mut().add_child(&particle);
        particle.bind_mut().play_particle(dir, cfg);
    }
}
