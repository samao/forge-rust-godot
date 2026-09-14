use godot::{classes::AudioStream, prelude::*, tools::try_get_autoload_by_name};

use crate::{
    managers::audio_manager::AudioManager, message::Message,
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
    audio: Option<Gd<AudioStream>>,
}

#[godot_api]
impl Breakable {
    #[signal]
    pub fn die();

    #[func]
    fn take_damage(&mut self, pos: Vector2, dir: Vector2, damage: f32) {
        self.hp -= damage;
        if self.hp <= 0.0 {
            for p in self.particles.iter_shared() {
                Message::singleton()
                    .signals()
                    .play_particles()
                    .emit(pos, dir, &p);
            }
            //爆开
            if let Some(sound) = self.audio.clone()
                && let Ok(mut audio_helper) =
                    try_get_autoload_by_name::<AudioManager>("AudioHelper")
            {
                audio_helper.bind_mut().play_spatial_sound(sound, pos);
            }
            self.signals().die().emit();

            if let Some(mut parent) = self.base().get_parent() {
                parent.call_deferred("queue_free", &[]);
            }
        }
    }
}
