use godot::{
    classes::{Button, CanvasLayer, ICanvasLayer, MarginContainer, TextureProgressBar},
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
    #[init(node = "%Button")]
    button: OnReady<Gd<Button>>,
}

#[godot_api]
impl ICanvasLayer for PlayerHud {
    fn ready(&mut self) {
        Message::singleton()
            .signals()
            .player_health_change()
            .connect_other(&*self, Self::on_health_change);
        self.button
            .signals()
            .pressed()
            .connect_other(&*self, Self::on_test);
    }
}

#[godot_api]
impl PlayerHud {
    fn on_test(&mut self) {
        Message::singleton().signals().camera_shake().emit(20.0);
        // if let Some(node) = self.base().get_tree().get_first_node_in_group("Player")
        //     && let Ok(player) = node.try_cast::<Node2D>()
        // {
        //     godot_print!("激发特效");
        //     Message::singleton().signals().camera_shake().emit(20.0);
        //     Message::singleton()
        //         .signals()
        //         .play_effect()
        //         .emit(VisualEffectType::Jump, player.get_global_position());
        // } else {
        //     godot_print!("没看见有玩家");
        // }
    }
    fn on_health_change(&mut self, hp: f32, max_hp: f32) {
        let percent = hp / max_hp;
        // let size = self.hp_container.get_size();
        // self.hp_container.set_size(size + Vector2::new(22.0, 0.0));
        // godot_print!("{}/{} == {}", hp, max_hp, percent);
        self.hp_bar.set_value(percent as f64);
    }
}
