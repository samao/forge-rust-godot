use godot::{
    classes::{GpuParticles2D, IGpuParticles2D, ParticleProcessMaterial},
    prelude::*,
};

use crate::resource::particles::HitParticleSetting;

#[derive(GodotClass)]
#[class(init, base = GpuParticles2D)]
pub struct HitParticle {
    base: Base<GpuParticles2D>,
}

#[godot_api]
impl IGpuParticles2D for HitParticle {
    fn ready(&mut self) {
        self.base()
            .signals()
            .finished()
            .connect_other(&*self, Self::on_play_finished);
    }
}

#[godot_api]
impl HitParticle {
    fn on_play_finished(&mut self) {
        self.base_mut().queue_free();
    }

    pub fn play_particle(&mut self, dir: Vector2, p_cfg: Gd<HitParticleSetting>) {
        // godot_print!("发射粒子");
        self.base_mut().set_amount(p_cfg.bind().count);
        self.base_mut().set_modulate(p_cfg.bind().color);
        if let Some(texture) = p_cfg.bind().texture.clone() {
            self.base_mut().set_texture(&texture);
        }
        if let Some(material) = self.base().get_process_material()
            && let Ok(mut process_mat) = material.try_cast::<ParticleProcessMaterial>()
        {
            process_mat.set_direction(Vector3::new(dir.x, dir.y, 0.0));

            self.base_mut().set_material(&process_mat);
        }

        self.base_mut().set_emitting(true);
    }
}
