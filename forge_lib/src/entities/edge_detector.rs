use godot::{
    classes::{IRayCast2D, RayCast2D},
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = RayCast2D)]
pub struct EdgeDetector {
    base: Base<RayCast2D>,
    #[init(val = true)]
    colliding: bool,
}

#[godot_api]
impl IRayCast2D for EdgeDetector {
    fn ready(&mut self) {
        self.colliding = self.base().is_colliding();
    }

    fn physics_process(&mut self, _delta: f64) {
        if !self.base().is_colliding() && self.colliding {
            self.signals().edge_detect().emit();
        }
        self.colliding = self.base().is_colliding();
    }
}

#[godot_api]
impl EdgeDetector {
    #[signal]
    pub fn edge_detect();
}
