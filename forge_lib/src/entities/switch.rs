use godot::{
    classes::{ResourceUid, Sprite2D},
    prelude::*,
    signal::ConnectHandle,
    tools::try_get_autoload_by_name,
};

use crate::{
    entities::{door::Door, interactive::Interactive},
    managers::save::SaveManager,
    message::Message,
};

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Switch {
    base: Base<Node2D>,
    #[init(node = "%Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,
    #[init(node = "%Interactive")]
    area: OnReady<Gd<Interactive>>,

    #[export]
    #[init(val = None)]
    door: Option<Gd<Door>>,

    #[init(val = None)]
    handle: Option<ConnectHandle>,
}

#[godot_api]
impl INode2D for Switch {
    fn ready(&mut self) {
        let opened = self.get_door_persisdent();
        godot_print!("门开了: {opened}");
        if opened {
            godot_print!("已经开过了");
            if let Some(ref mut door) = self.door {
                door.bind_mut().set_opened();
            }
            return;
        }
        self.area
            .signals()
            .active()
            .connect_other(&*self, Self::on_active);
        self.area
            .signals()
            .deactive()
            .connect_other(&*self, Self::on_deactive);
        self.get_unique_uid();
    }
}

#[godot_api]
impl Switch {
    fn on_active(&mut self) {
        // godot_print!("激活");
        self.handle = Some(
            Message::singleton()
                .signals()
                .interactive()
                .connect_other(&*self, Self::on_switch),
        );
        Message::singleton()
            .signals()
            .input_hint_change()
            .emit("interact".to_owned());
    }

    fn on_deactive(&mut self) {
        // godot_print!("取消激活");
        if let Some(handle) = self.handle.take() {
            handle.disconnect();
        }

        Message::singleton()
            .signals()
            .input_hint_change()
            .emit("".to_owned());
    }

    fn get_door_persisdent(&self) -> bool {
        if let Ok(mut save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            godot_print!("获取");
            return bool::from_variant(
                &save_helper
                    .bind_mut()
                    .get_persisdent_data(self.get_unique_uid(), &false.to_variant()),
            );
        } else {
            godot_print!("没办法");
        }

        false
    }
    fn set_door_persisdent(&self, open: bool) {
        if let Ok(mut save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            save_helper
                .bind_mut()
                .set_persisdent_data(self.get_unique_uid(), &open.to_variant());
        }
    }

    fn get_unique_uid(&self) -> String {
        if let Some(root) = self.base().get_owner() {
            let root_path = ResourceUid::path_to_uid(&root.get_scene_file_path()).to_string();
            let uni_path = format!("{}/{}", root_path, self.base().get_name());
            return uni_path;
        }
        panic!("给个路径啊")
    }

    fn on_switch(&mut self) {
        let flip = self.sprite.is_flipped_h();
        self.sprite.set_flip_h(!flip);

        match self.door {
            Some(ref mut door) if flip => {
                door.bind_mut().close();
                self.set_door_persisdent(false);
            }
            Some(ref mut door) if !flip => {
                door.bind_mut().open();
                self.set_door_persisdent(true);
            }
            _ => {}
        }
    }
}
