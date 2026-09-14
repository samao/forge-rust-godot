use godot::{
    classes::{ISprite2D, Sprite2D},
    prelude::*,
};

use crate::entities::breakable::Breakable;

#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct Crate {
    base: Base<Sprite2D>,
    #[init(node = "%Breakable")]
    breakable: OnReady<Gd<Breakable>>,
}

#[godot_api]
impl ISprite2D for Crate {
    fn ready(&mut self) {
        self.breakable
            .signals()
            .die()
            .connect_other(&*self, Self::on_broken);
    }
}

#[godot_api]
impl Crate {
    fn on_broken(&mut self) {
        // self.base_mut().queue_free();
    }
}
