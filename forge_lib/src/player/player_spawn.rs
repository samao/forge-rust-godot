use godot::{
    classes::{ISprite2D, Sprite2D},
    obj::WithBaseField,
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct PlayerSpawn {
    base: Base<Sprite2D>,

    #[init(load = "res://scenes/player.tscn")]
    player_scn: OnReady<Gd<PackedScene>>,
}

#[godot_api]
impl ISprite2D for PlayerSpawn {
    fn ready(&mut self) {
        self.base_mut().set_visible(false);
        self.base_mut().call_deferred("place_player", &[]);
    }
}

#[godot_api]
impl PlayerSpawn {
    #[func]
    fn place_player(&mut self) {
        // godot_print!("PLACE THE PLAYER");
        let mut has_player = false;
        let mut player = match self.base().get_tree().get_first_node_in_group("Player") {
            Some(player) => match player.try_cast::<Player>() {
                Ok(player) => {
                    has_player = true;
                    player
                }
                _ => self.player_scn.instantiate_as::<Player>(),
            },
            _ => self.player_scn.instantiate_as::<Player>(),
        };

        if let Some(mut root) = self.base().get_tree().get_root() {
            let pos = self.base().get_global_position();
            player.set_global_position(pos);
            if has_player {
                let top_index = root.get_child_count() - 1;
                root.move_child(&player, top_index);
                return;
            }
            root.add_child(&player);
        }
    }
}
