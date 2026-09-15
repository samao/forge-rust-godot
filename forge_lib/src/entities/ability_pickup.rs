use godot::{
    classes::{
        AnimationPlayer, Engine, Sprite2D, notify::CanvasItemNotification, object::ConnectFlags,
    },
    prelude::*,
};

use crate::entities::breakable::Breakable;

#[derive(GodotClass)]
#[class(init, base = Node2D, tool)]
pub struct AbilityPickup {
    base: Base<Node2D>,
    #[init(val = None)]
    ability_anim: Option<Gd<AnimationPlayer>>,
    #[init(val = None)]
    orb_anim: Option<Gd<AnimationPlayer>>,
    #[export]
    #[init(val = AbilityType::DoubleJump)]
    #[var(set, get)]
    ability_type: AbilityType,

    #[export]
    #[init(val = 3.0)]
    hp: f32,
}

#[derive(Var, Debug, Default, GodotConvert, Export, Clone, Copy)]
#[godot(via = GString)]
pub enum AbilityType {
    #[default]
    DoubleJump,
    Slam,
    Dash,
    MorphRoll,
}

#[godot_api]
impl INode2D for AbilityPickup {
    fn ready(&mut self) {
        self.setup_animation_node();
        self.setup_animation();
        if Engine::singleton().is_editor_hint() {
            return;
        }
        if let Some(breakable) = self.base().try_get_node_as::<Breakable>("%Breakable") {
            breakable
                .signals()
                .take_damage()
                .connect_other(&*self, Self::take_damage);
        }
    }

    fn on_notification(&mut self, what: CanvasItemNotification) {
        match what {
            CanvasItemNotification::EXTENSION_RELOADED => {
                self.setup_animation_node();
            }
            _ => {}
        }
    }
}

#[godot_api]
impl AbilityPickup {
    fn setup_animation(&mut self) {
        let animation_name = match self.ability_type {
            AbilityType::DoubleJump => "double_jump",
            AbilityType::Slam => "slam",
            AbilityType::Dash => "dash",
            AbilityType::MorphRoll => "morph_roll",
        };
        if let Some(ref mut ab_anim) = self.ability_anim {
            ab_anim.play_ex().name(animation_name).done();
        }
    }

    fn setup_animation_node(&mut self) {
        self.ability_anim = self.base().try_get_node_as("%AbilityAnim");
        self.orb_anim = self.base().try_get_node_as("%OrbAnim");
    }

    #[func]
    pub fn take_damage(&mut self, _damage: f32) {
        self.hp -= 1.0;

        if self.hp == 0.0
            && let Some(ref mut orb_anim) = self.orb_anim.clone()
        {
            let mut this = self.to_gd();
            // godot_print!("播放销毁动画， 获得能力");
            orb_anim.play_ex().name("destroy").done();
            orb_anim.connect_flags(
                "animation_finished",
                &Callable::from_fn("orb_destroy", move |_| {
                    this.call_deferred("queue_free", &[]);
                }),
                ConnectFlags::ONE_SHOT,
            );
        } else {
            self.update_orb_frame();
        }
    }

    fn update_orb_frame(&self) {
        if let Some(mut orb) = self.base().try_get_node_as::<Sprite2D>("%OrbSprite") {
            orb.set_frame(23 - self.hp as i32);
        }
    }

    #[func]
    pub fn set_ability_type(&mut self, v: AbilityType) {
        self.ability_type = v;
        self.setup_animation();
    }

    #[func]
    pub fn get_ability_type(&self) -> AbilityType {
        self.ability_type
    }
}
