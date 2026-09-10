use godot::{
    classes::{AnimationPlayer, Sprite2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct DustEffect {
    base: Base<Sprite2D>,
    #[init(node = "%AnimationPlayer")]
    anim: OnReady<Gd<AnimationPlayer>>,
}

// #[godot_api]
// impl ISprite2D for DustEffect {
//     fn ready(&mut self) {
//         self.anim
//             .signals()
//             .animation_finished()
//             .connect_other(&*self, Self::on_effect_finished);
//     }
// }

#[godot_api]
impl DustEffect {
    pub fn play(&mut self, name: &str) {
        self.anim
            .signals()
            .animation_finished()
            .connect_other(&*self, Self::on_effect_finished);
        self.anim.play_ex().name(name).done();
    }

    fn on_effect_finished(&mut self, _: StringName) {
        self.base_mut().queue_free();
    }
}
