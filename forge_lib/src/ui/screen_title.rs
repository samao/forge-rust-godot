use godot::classes::object::ConnectFlags;
use godot::classes::{AnimationPlayer, Button, CanvasLayer, ICanvasLayer, VBoxContainer};
use godot::prelude::*;
use godot::tools::try_get_autoload_by_name;

use crate::level_transition::Side;
use crate::managers::save::SaveManager;
use crate::managers::scene_manager::SceneManager;

#[derive(GodotClass)]
#[class(init, base = CanvasLayer)]
struct ScreenTitle {
    base: Base<CanvasLayer>,

    #[init(node = "%MainMenu")]
    main_menu: OnReady<Gd<VBoxContainer>>,
    #[init(node = "%NewGame")]
    new_menu: OnReady<Gd<VBoxContainer>>,
    #[init(node = "%LoadGame")]
    load_menu: OnReady<Gd<VBoxContainer>>,

    #[init(node = "%AnimationPlayer")]
    animation: OnReady<Gd<AnimationPlayer>>,
    #[init(node = "%NewGameButton")]
    new_game_btn: OnReady<Gd<Button>>,
    #[init(node = "%LoadGameButton")]
    load_game_btn: OnReady<Gd<Button>>,
    #[init(node = "%Begin_Slot1")]
    begin_slot1: OnReady<Gd<Button>>,
    #[init(node = "%Begin_Slot2")]
    begin_slot2: OnReady<Gd<Button>>,
    #[init(node = "%Begin_Slot3")]
    begin_slot3: OnReady<Gd<Button>>,

    #[init(node = "%Load_Slot1")]
    load_slot1: OnReady<Gd<Button>>,
    #[init(node = "%Load_Slot2")]
    load_slot2: OnReady<Gd<Button>>,
    #[init(node = "%Load_Slot3")]
    load_slot3: OnReady<Gd<Button>>,

    #[init(node = "%Select")]
    select: OnReady<Gd<VBoxContainer>>,
    #[init(node = "%BackButton")]
    back_btn: OnReady<Gd<Button>>,
}

#[godot_api]
impl ICanvasLayer for ScreenTitle {
    fn ready(&mut self) {
        self.new_menu.set_visible(false);
        self.load_menu.set_visible(false);
        self.select.set_visible(false);
        self.animation.play_ex().name("start").done();
        let mut this = self.to_gd();
        self.animation.connect_flags(
            "animation_finished",
            &Callable::from_fn("start_finished", move |_| {
                this.bind_mut().animation.play_ex().name("loop").done();
            }),
            ConnectFlags::ONE_SHOT,
        );

        self.new_game_btn
            .signals()
            .pressed()
            .connect_other(&*self, Self::on_new_game_pressed);
        if let Ok(save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper")
            && save_helper.bind().has_saved_point()
        {
            self.load_game_btn.set_disabled(false);
            self.load_game_btn
                .signals()
                .pressed()
                .connect_other(&*self, Self::on_load_game_pressed);
        } else {
            self.load_game_btn.set_disabled(true);
        }

        self.back_btn
            .signals()
            .pressed()
            .connect_other(&*self, Self::on_back_pressed);

        self.setup_slot();
    }
}

enum ButtonType {
    New,
    Load,
}

#[godot_api]
impl ScreenTitle {
    fn on_back_pressed(&mut self) {
        self.new_menu.set_visible(false);
        self.load_menu.set_visible(false);
        self.select.set_visible(false);
        self.main_menu.set_visible(true);
    }

    fn on_new_game_pressed(&mut self) {
        self.main_menu.set_visible(false);
        self.new_menu.set_visible(true);
        self.select.set_visible(true);
    }
    fn on_load_game_pressed(&mut self) {
        self.main_menu.set_visible(false);
        self.load_menu.set_visible(true);
        self.select.set_visible(true);
    }

    fn has_save_point(&self, index: u8) -> bool {
        if let Ok(save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            return save_helper.bind().has_slot(index);
        }

        false
    }

    fn set_slot_label(&mut self, b_type: ButtonType, index: u8) {
        let button = match b_type {
            ButtonType::New => match index {
                1 => &self.begin_slot1,
                2 => &self.begin_slot2,
                3 => &self.begin_slot3,
                _ => unreachable!(),
            },
            ButtonType::Load => match index {
                1 => &self.load_slot1,
                2 => &self.load_slot2,
                3 => &self.load_slot3,
                _ => unreachable!(),
            },
        };
        let has_saved = self.has_save_point(index);
        let mut button = button.to_godot().clone();
        match b_type {
            ButtonType::New => {
                button.set_text(
                    if has_saved {
                        format!("Replace Slot {}", index)
                    } else {
                        format!("Begin Slot {}", index)
                    }
                    .as_str(),
                );
            }
            ButtonType::Load => {
                button.set_text(
                    if has_saved {
                        format!("Load Slot {}", index)
                    } else {
                        "Empty".to_owned()
                    }
                    .as_str(),
                );
            }
        }
    }

    fn setup_slot(&mut self) {
        self.set_slot_label(ButtonType::New, 1);
        let on_begin_press = Callable::from_object_method(&self.to_gd(), "on_begin_pressed");
        self.begin_slot1
            .connect("pressed", &on_begin_press.bind(&[1.to_variant()]));
        self.set_slot_label(ButtonType::New, 2);
        self.begin_slot2
            .connect("pressed", &on_begin_press.bind(&[2.to_variant()]));
        self.set_slot_label(ButtonType::New, 3);
        self.begin_slot3
            .connect("pressed", &on_begin_press.bind(&[3.to_variant()]));
        let on_load_press = Callable::from_object_method(&self.to_gd(), "on_load_pressed");
        self.set_slot_label(ButtonType::Load, 1);
        self.load_slot1
            .connect("pressed", &on_load_press.bind(&[1.to_variant()]));
        self.set_slot_label(ButtonType::Load, 2);
        self.load_slot2
            .connect("pressed", &on_load_press.bind(&[2.to_variant()]));
        self.set_slot_label(ButtonType::Load, 3);
        self.load_slot3
            .connect("pressed", &on_load_press.bind(&[3.to_variant()]));
    }

    #[func]
    fn on_begin_pressed(&self, index: u8) {
        if let Ok(mut save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            if let Err(msg) = save_helper.bind_mut().start_on_slot(index) {
                godot_print!("{}", msg);
            } else {
                godot_print!("在存档位置 {index} 开始游戏");
                save_helper.bind_mut().create_new_game();

                if let Ok(mut transition) =
                    try_get_autoload_by_name::<SceneManager>("SceneTransition")
                {
                    transition.bind_mut().travel_scene(
                        "uid://bowrs0wcn2oy6".to_owned(),
                        "".to_owned(),
                        Side::Bottom,
                    );
                }
            }
        }
    }

    #[func]
    fn on_load_pressed(&self, index: u8) {
        if let Ok(mut save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            if save_helper.bind().has_slot(index) {
                save_helper.bind_mut().load_from_slot(index);
            }
        }
    }
}
