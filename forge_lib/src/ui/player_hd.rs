use godot::{
    classes::{CanvasLayer, ICanvasLayer, MarginContainer, TextureProgressBar},
    prelude::*,
};

use crate::message::Message;

#[derive(GodotClass)]
#[class(init, base = CanvasLayer)]
struct PlayerHud {
    base: Base<CanvasLayer>,
    #[init(node = "%HPMarginContainer")]
    hp_container: OnReady<Gd<MarginContainer>>,
    #[init(node = "%HPBar")]
    hp_bar: OnReady<Gd<TextureProgressBar>>,
}

#[godot_api]
impl ICanvasLayer for PlayerHud {
    fn ready(&mut self) {
        Message::singleton()
            .signals()
            .player_health_change()
            .connect_other(&*self, Self::on_health_change);
    }
}

#[godot_api]
impl PlayerHud {
    fn on_health_change(&mut self, hp: f32, max_hp: f32) {
        let percent = hp / max_hp;
        // let size = self.hp_container.get_size();
        // self.hp_container.set_size(size + Vector2::new(22.0, 0.0));
        // godot_print!("{}/{} == {}", hp, max_hp, percent);
        self.hp_bar.set_value(percent as f64);
    }
}
