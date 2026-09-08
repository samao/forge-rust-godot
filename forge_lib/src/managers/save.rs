use godot::{
    classes::{
        Engine, FileAccess, InputEvent, InputEventKey, Json, file_access::ModeFlags,
        object::ConnectFlags,
    },
    global::Key,
    obj::WithBaseField,
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::{player::Player, scene_manager::SceneManager};

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct SaveManager {
    base: Base<Node2D>,
    #[init(val = 1)]
    current_slot: u8,
    save_data: VarDictionary,
    discovered_areas: Array<GString>,
    persistent_data: VarDictionary,
}

#[godot_api]
impl INode2D for SaveManager {
    fn ready(&mut self) {
        self.load_save_file();
    }
    fn unhandled_key_input(&mut self, event: Gd<InputEvent>) {
        if event.is_pressed()
            && let Ok(event) = event.try_cast::<InputEventKey>()
        {
            match event.get_keycode() {
                Key::KEY_0 => {
                    self.create_new_game();
                }
                Key::KEY_9 => {
                    self.load_game();
                }
                Key::KEY_8 => {
                    self.save_game();
                }
                Key::KEY_1 => self.current_slot = 1,
                Key::KEY_2 => self.current_slot = 2,
                Key::KEY_3 => self.current_slot = 3,
                _ => {}
            }
        }
    }
}

const DEFAULT_SCENE: &str = "uid://bowrs0wcn2oy6";

#[godot_api]
impl SaveManager {
    fn get_save_path(&self) -> String {
        format!("user://save_{}.sav", self.current_slot)
    }

    pub fn has_saved_point(&self) -> bool {
        for i in 0..3 {
            if FileAccess::file_exists(&format!("user://save_{}.sav", i)) {
                godot_print!("有存档： {i}");
                return true;
            }
        }
        false
    }

    pub fn get_persisdent_data(&mut self, key: String, defautl_val: &Variant) -> Variant {
        self.persistent_data.get_or_insert(key, defautl_val)
    }

    pub fn set_persisdent_data(&mut self, key: String, defautl_val: &Variant) {
        self.persistent_data.set(key, defautl_val);
    }

    pub fn has_slot(&self, index: u8) -> bool {
        FileAccess::file_exists(&format!("user://save_{}.sav", index))
    }

    pub fn start_on_slot(&mut self, index: u8) -> Result<(), String> {
        if self.has_slot(index) {
            return Err(format!("位置 {} 已经有存档了", index));
        }
        self.current_slot = index;
        self.create_new_game();
        Ok(())
    }

    pub fn create_new_game(&mut self) {
        // godot_print!("create new game");
        self.save_data = vdict!(
            "scene_path" => DEFAULT_SCENE,
            "x" => 60.0,
            "y" => 230.0,
            "hp" => 20.0,
            "max_hp" => 20,
            "dash" => false,
            "double_jump" => false,
            "ground_slam" => false,
            "morph_roll" => false,
            "discovered_areas" => &self.discovered_areas,
            "persistent_data" => &self.persistent_data
        );

        if let Some(mut file) = FileAccess::open(self.get_save_path().as_str(), ModeFlags::WRITE) {
            let json_str = Json::stringify(&self.save_data.to_variant());
            file.store_line(&json_str);
            file.close();
        } else {
            godot_print!("无法打开文件");
        }
    }

    #[func]
    pub fn save_game(&mut self) {
        godot_print!("save a point");
        if let Some(main_loop) = Engine::singleton().get_main_loop()
            && let Ok(tree) = main_loop.try_cast::<SceneTree>()
        {
            if let Some(player) = tree.get_first_node_in_group("Player")
                && let Ok(player) = player.try_cast::<Player>()
            {
                godot_print!("保存文件");
                let pos = player.get_global_position();
                if let Some(mut file) =
                    FileAccess::open(self.get_save_path().as_str(), ModeFlags::WRITE)
                {
                    let player = player.bind();
                    if let Ok(transition) =
                        try_get_autoload_by_name::<SceneManager>("SceneTransition")
                    {
                        self.save_data = vdict!(
                            "scene_path" => transition.bind().get_current_path(),
                            "x" => pos.x,
                            "y" => pos.y,
                            "hp" => player.hp,
                            "max_hp" => player.max_hp,
                            "dash" => player.dash,
                            "double_jump" => player.double_jump,
                            "ground_slam" => player.ground_slam,
                            "morph_roll" => player.morph_roll,
                            "discovered_areas" => &self.discovered_areas.to_variant(),
                            "persistent_data" => &self.persistent_data.to_variant()
                        );

                        file.store_line(&Json::stringify(&self.save_data.to_variant()));
                        file.close();
                    }
                }
            }
        }
    }

    pub fn load_from_slot(&mut self, index: u8) {
        self.current_slot = index;
        godot_print!("从存档： {} 开始", index);
        self.load_save_file();
        self.load_game();
    }

    fn load_save_file(&mut self) {
        let save_path = self.get_save_path();
        if FileAccess::file_exists(save_path.as_str()) {
            if let Some(mut file) = FileAccess::open(save_path.as_str(), ModeFlags::READ) {
                let save_str = file.get_line();
                file.close();
                self.save_data = VarDictionary::from_variant(&Json::parse_string(&save_str));
                let dis_areas = self
                    .save_data
                    .get_or_insert("discovered_areas", &Array::<GString>::default());
                self.discovered_areas = match Array::<GString>::try_from_variant(&dis_areas) {
                    Ok(arr) => arr,
                    _ => Array::<GString>::default(),
                };
                self.persistent_data = VarDictionary::from_variant(
                    &self
                        .save_data
                        .get_or_insert("persistent_data", &VarDictionary::new()),
                );
            }
        }
    }

    #[func]
    pub fn load_game(&mut self) {
        godot_print!("加载文件");
        // let save_path = self.get_save_path();
        // if FileAccess::file_exists(save_path.as_str()) {
        //     if let Some(mut file) = FileAccess::open(save_path.as_str(), ModeFlags::READ) {
        //         let save_str = file.get_line();
        //         file.close();
        //         self.save_data = VarDictionary::from_variant(&Json::parse_string(&save_str));
        //         let dis_areas = self
        //             .save_data
        //             .get_or_insert("discovered_areas", &Array::<GString>::default());
        //         self.discovered_areas = match Array::<GString>::try_from_variant(&dis_areas) {
        //             Ok(arr) => arr,
        //             _ => Array::<GString>::default(),
        //         };
        //         self.persistent_data = VarDictionary::from_variant(
        //             &self
        //                 .save_data
        //                 .get_or_insert("persistent_data", &VarDictionary::new()),
        //         );
        let scene_path = self
            .save_data
            .get_or_insert("scene_path", DEFAULT_SCENE)
            .to_string();

        if let Ok(ref mut transition) = try_get_autoload_by_name::<SceneManager>("SceneTransition")
        {
            transition.bind_mut().travel_scene(
                scene_path,
                "".to_owned(),
                crate::level_transition::Side::Bottom,
            );
            self.setup_player();
        }
        // }
        // }
    }

    #[func]
    pub fn setup_player(&mut self) {
        if let Some(main_loop) = Engine::singleton().get_main_loop()
            && let Ok(ref mut tree) = main_loop.try_cast::<SceneTree>()
        {
            let mut this = self.to_gd();
            tree.connect_flags(
                "scene_changed",
                &Callable::from_fn("delay_recovery_player", move |_| {
                    this.call_deferred("recovery_player", &[]);
                }),
                ConnectFlags::ONE_SHOT,
            );
        } else {
            godot_print!("不存在场景树");
        }
    }

    #[func]
    fn recovery_player(&mut self) {
        let hp = f32::from_variant(&self.save_data.get_or_insert("hp", 20.0));
        let max_hp = f32::from_variant(&self.save_data.get_or_insert("max_hp", 20.0));
        let dash = bool::from_variant(&self.save_data.get_or_insert("dash", false));
        let double_jump = bool::from_variant(&self.save_data.get_or_insert("double_jump", false));
        let ground_slam = bool::from_variant(&self.save_data.get_or_insert("ground_slam", false));
        let morph_roll = bool::from_variant(&self.save_data.get_or_insert("morph_roll", false));
        let x = f32::from_variant(&self.save_data.get_or_insert("x", 60.0));
        let y = f32::from_variant(&self.save_data.get_or_insert("y", 230.0));
        godot_print!("setup player: ({},{})", x, y);
        if let Some(main_loop) = Engine::singleton().get_main_loop()
            && let Ok(ref mut tree) = main_loop.try_cast::<SceneTree>()
        {
            if let Some(node) = tree.get_first_node_in_group("Player")
                && let Ok(mut player) = node.try_cast::<Player>()
            {
                godot_print!("这里可以回复玩家数据: ({x}, {y})");
                player.bind_mut().hp = hp;
                player.bind_mut().max_hp = max_hp;
                player.bind_mut().dash = dash;
                player.bind_mut().double_jump = double_jump;
                player.bind_mut().ground_slam = ground_slam;
                player.bind_mut().morph_roll = morph_roll;
                player.set_global_position(Vector2 { x, y });
            } else {
                godot_print!("没有玩家呢");
            }
        }
    }
}
