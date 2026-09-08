use godot::classes::object::ConnectFlags;
use godot::classes::{Area2D, IArea2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct Interactive {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Interactive {
    fn ready(&mut self) {
        self.base()
            .signals()
            .body_entered()
            .connect_other(&*self, Self::on_body_enter);
    }
}

#[godot_api]
impl Interactive {
    #[signal]
    pub fn active();
    #[signal]
    pub fn deactive();

    fn on_body_enter(&mut self, _body: Gd<Node2D>) {
        self.signals().active().emit();

        let this = self.to_gd();
        self.base_mut().connect_flags(
            "body_exited",
            &Callable::from_object_method(&this, "on_body_exit"),
            ConnectFlags::ONE_SHOT,
        );
    }
    #[func]
    fn on_body_exit(&mut self, _body: Gd<Node2D>) {
        self.signals().deactive().emit();
    }
}
