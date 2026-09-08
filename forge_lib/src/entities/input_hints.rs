use std::{collections::HashMap, sync::LazyLock};

use godot::{
    classes::{
        Input, InputEvent, InputEventJoypadButton, InputEventKey, InputEventMouseButton, Sprite2D,
    },
    prelude::*,
};

use crate::message::Message;

static HINT_MAP: LazyLock<HashMap<String, HashMap<String, i32>>> = std::sync::LazyLock::new(|| {
    HashMap::<String, HashMap<String, i32>>::from([
        (
            "keyboard".into(),
            HashMap::<String, i32>::from([
                ("interact".into(), 15),
                ("attack".into(), 10),
                ("jump".into(), 9),
                ("dash".into(), 11),
                ("up".into(), 13),
            ]),
        ),
        (
            "playstation".into(),
            HashMap::<String, i32>::from([
                ("interact".into(), 0),
                ("attack".into(), 2),
                ("jump".into(), 1),
                ("dash".into(), 3),
                ("up".into(), 4),
            ]),
        ),
        (
            "xbox".into(),
            HashMap::<String, i32>::from([
                ("interact".into(), 8),
                ("attack".into(), 3),
                ("jump".into(), 5),
                ("dash".into(), 6),
                ("up".into(), 4),
            ]),
        ),
    ])
});

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct InputHints {
    base: Base<Node2D>,
    #[init(node = "%Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,
    #[init(val = "".into())]
    controller_type: String,
}

#[godot_api]
impl INode2D for InputHints {
    fn ready(&mut self) {
        self.base_mut().set_visible(false);
        Message::singleton()
            .signals()
            .input_hint_change()
            .connect_other(&*self, Self::on_hint_change);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.clone().try_cast::<InputEventMouseButton>().is_ok()
            || event.clone().try_cast::<InputEventKey>().is_ok()
        {
            self.controller_type = "keyboard".into();
        } else if let Ok(event) = event.try_cast::<InputEventJoypadButton>() {
            self.get_controller_type(event.get_device());
        }
    }
}

#[godot_api]
impl InputHints {
    fn get_controller_type(&mut self, device_id: i32) {
        let n = Input::singleton().get_joy_name(device_id);
        self.controller_type = if n.contains("xbox") {
            "xbox".into()
        } else if n.contains("playstation") || n.contains("ps") || n.contains("dualsense") {
            "playstation".into()
        } else if n.contains("nintendo") || n.contains("switch") {
            "xbox".into()
        } else {
            "unkown".into()
        };

        self.base_mut().set_process_input(false);
    }

    fn get_hint_frame(&self, hint: String) -> i32 {
        if let Some(n) = HINT_MAP.get(&self.controller_type) {
            if let Some(frame) = n.get(&hint) {
                return frame.to_owned();
            }
        }
        0
    }

    fn on_hint_change(&mut self, hint: String) {
        let visible = match hint.as_str() {
            "interact" => {
                let frame = self.get_hint_frame(hint);
                self.sprite.set_frame(frame);
                true
            }
            _ => false,
        };
        self.base_mut().set_visible(visible);
    }
}
