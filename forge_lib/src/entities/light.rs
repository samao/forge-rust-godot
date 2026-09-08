use godot::{
    classes::{IPointLight2D, PointLight2D, Timer},
    global::randf_range,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base = PointLight2D, init)]
pub struct FlickerLight {
    base: Base<PointLight2D>,

    #[export]
    #[init(val = 0.1)]
    flicker_intensity: f32,

    #[export]
    #[init(val = 0.2)]
    flicker_frequency: f32,

    #[init(val = 1.0)]
    og_energy: f32,

    #[init(node = "Timer")]
    timer: OnReady<Gd<Timer>>,
}

#[godot_api]
impl IPointLight2D for FlickerLight {
    fn ready(&mut self) {
        self.og_energy = self.base().get_energy();
        self.timer
            .signals()
            .timeout()
            .connect_other(&*self, Self::on_timeout_flicker);
    }
}

#[godot_api]
impl FlickerLight {
    fn on_timeout_flicker(&mut self) {
        self.timer
            .set_wait_time((1.0 + randf_range(-0.3, 0.3)) * self.flicker_frequency as f64);

        let intensity = randf_range(-1.0, 1.0) * self.flicker_intensity as f64;
        let energy = self.og_energy + intensity as f32;
        self.base_mut().set_energy(energy);
    }
}
