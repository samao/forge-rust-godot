use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, Copy, PartialEq, Eq)]
#[godot(via = GString)]
pub enum SceneTheme {
    Light,
    Dark,
}

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Level {
    #[export]
    #[init(val = SceneTheme::Light)]
    pub theme: SceneTheme,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Level {
    fn ready(&mut self) {
        self.base_mut().add_to_group("LevelTheme");
    }
}
