use godot::{classes::AudioStream, prelude::*, tools::try_get_autoload_by_name};

use crate::{
    entities::attack::AttackArea, managers::audio_manager::AudioManager, message::Message,
    resource::particles::HitParticleSetting,
};

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Breakable {
    base: Base<Node2D>,

    #[export]
    #[init(val = 4.0)]
    hp: f32,
    #[export]
    fix_count: bool,

    #[export]
    #[init(val = Array::new())]
    particles: Array<Gd<HitParticleSetting>>,

    #[export]
    #[init(val = None)]
    break_audio: Option<Gd<AudioStream>>,

    #[export]
    #[init(val = true)]
    auto_remove: bool,

    #[export]
    #[init(val = None)]
    hit_audio: Option<Gd<AudioStream>>,
}

#[godot_api]
impl Breakable {
    #[signal]
    pub fn die();

    #[signal]
    pub fn take_damage(damage: f32);

    #[func]
    fn take_damage(&mut self, pos: Vector2, dir: Vector2, damage: Gd<AttackArea>) {
        let damage = damage.bind().get_damage();
        self.hp -= damage;
        self.signals().take_damage().emit(damage);
        if self.hp <= 0.0 {
            for p in self.particles.iter_shared() {
                Message::singleton()
                    .signals()
                    .play_particles()
                    .emit(pos, dir, &p);
            }
            //爆开
            if let Some(sound) = self.break_audio.clone()
                && let Ok(mut audio_helper) =
                    try_get_autoload_by_name::<AudioManager>("AudioHelper")
            {
                audio_helper.bind_mut().play_spatial_sound(sound, pos);
            }
            self.signals().die().emit();

            if let Some(mut parent) = self.base().get_parent()
                && self.auto_remove
            {
                parent.call_deferred("queue_free", &[]);
            }
        } else {
            if let Some(audio) = self.hit_audio.clone() {
                Message::singleton()
                    .signals()
                    .play_spatial_audio()
                    .emit(&audio, pos);
            }
        }
    }
}
