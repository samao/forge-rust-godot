use godot::{
    builtin::math::FloatExt,
    classes::{Camera2D, ICamera2D},
    global::{clampf, randf_range},
    prelude::*,
};

use crate::message::Message;

#[derive(GodotClass)]
#[class(init, base = Camera2D)]
struct PlayerCamera {
    base: Base<Camera2D>,
    #[init(val = 0.0)]
    shake_strength: f32,
}

const DECAY_RATE: f64 = 5.0;
const MAX_SHAKE_STRENGTH: f64 = 20.0;

#[godot_api]
impl ICamera2D for PlayerCamera {
    fn ready(&mut self) {
        Message::singleton()
            .signals()
            .camera_shake()
            .connect_other(&*self, Self::on_camera_shake);
    }

    fn process(&mut self, delta: f64) {
        let mut offset = Vector2::ZERO;
        let shake_strength = self.shake_strength as f64;
        offset += Vector2::new(
            randf_range(-shake_strength, shake_strength) as f32,
            randf_range(-shake_strength, shake_strength) as f32,
        );
        self.shake_strength = self.shake_strength.lerp(0.0, (DECAY_RATE * delta) as f32);
        if self.shake_strength.abs() < 1.0 {
            self.base_mut().set_offset(Vector2::ZERO);
            self.base_mut().set_process(false);
        } else {
            self.base_mut().set_offset(offset);
        }
    }
}

#[godot_api]
impl PlayerCamera {
    fn on_camera_shake(&mut self, strength: f64) {
        self.shake_strength = clampf(strength, 0.0, MAX_SHAKE_STRENGTH) as f32;
        self.base_mut().set_process(true);
    }
}
