use godot::{classes::Texture2D, prelude::*};

#[derive(GodotClass, Debug)]
#[class(init, base = Resource)]
pub struct HitParticleSetting {
    base: Base<Resource>,

    #[export]
    #[init(val = 8)]
    pub count: i32,

    #[export]
    pub texture: Option<Gd<Texture2D>>,

    #[export]
    #[init(val = Color::WHITE)]
    pub color: Color,
}
