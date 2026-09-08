use godot::{classes::AnimationPlayer, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Door {
    base: Base<Node2D>,
    #[init(node = "%AnimationPlayer")]
    animation: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl INode2D for Door {
    fn ready(&mut self) {
        self.animation
            .signals()
            .animation_finished()
            .connect_other(&*self, Self::on_animation_finished);
    }
}

#[godot_api]
impl Door {
    pub fn open(&mut self) {
        self.animation.play_ex().name("open").done();
    }

    pub fn close(&mut self) {
        self.animation.play_ex().name("close").done();
    }

    pub fn set_opened(&mut self) {
        godot_print!("直接设置开门状态");
        self.animation.play_ex().name("opened").done();
    }

    #[func]
    fn on_animation_finished(&mut self, name: StringName) {
        match name.to_string().as_str() {
            "open" => {
                self.animation.play_ex().name("opened").done();
            }
            "close" => {
                self.animation.play_ex().name("closed").done();
            }
            _ => {}
        }
    }
}
