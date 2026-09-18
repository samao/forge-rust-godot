use godot::{
    classes::{
        AnimationPlayer, AudioStream, CharacterBody2D, ICharacterBody2D, Sprite2D, Tween,
        object::ConnectFlags,
    },
    prelude::*,
};

use crate::{
    entities::{attack::AttackArea, damage::DamageArea, edge_detector::EdgeDetector},
    message::Message,
};

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub struct Slime {
    base: Base<CharacterBody2D>,
    #[export]
    #[init(val = 3.0)]
    hp: f32,
    #[init(val = 1.0)]
    #[var(set, get)]
    dir: f32,
    #[export]
    #[init(val = true)]
    face_right_on_start: bool,
    #[export]
    #[init(val = 30.0)]
    speed: f32,
    #[init(node = "%Sprite2D")]
    body: OnReady<Gd<Sprite2D>>,
    #[init(node = "%AnimationPlayer")]
    anim: OnReady<Gd<AnimationPlayer>>,
    #[init(node = "%DamageArea")]
    damage_area: OnReady<Gd<DamageArea>>,
    #[init(node = "%AttackArea")]
    attack_area: OnReady<Gd<AttackArea>>,
    #[init(node = "%EdgeDetector")]
    edge_detector: OnReady<Gd<EdgeDetector>>,
    #[init(val = None)]
    tween: Option<Gd<Tween>>,

    #[export_group(name = "audio")]
    #[export]
    hit: Option<Gd<AudioStream>>,
    #[export]
    death: Option<Gd<AudioStream>>,
}

#[godot_api]
impl ICharacterBody2D for Slime {
    fn ready(&mut self) {
        if self.face_right_on_start {
            self.set_dir(1.0);
        } else {
            self.set_dir(-1.0);
        }

        self.anim
            .signals()
            .animation_finished()
            .connect_other(&*self, Self::on_animation_finished);
        self.edge_detector
            .signals()
            .edge_detect()
            .connect_other(&*self, Self::on_edge_detected);
    }

    fn physics_process(&mut self, _delta: f64) {
        // let dt = _delta as f32;
        let mut velocity = if self.dir > 0.0 {
            Vector2::RIGHT * self.speed
        } else {
            Vector2::LEFT * self.speed
        };
        if !self.base().is_on_floor() {
            velocity += self.base().get_gravity();
            velocity.x *= 0.3;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        if self.base().is_on_wall() {
            self.set_dir(-self.dir);
        }
    }
}

#[godot_api]
impl Slime {
    #[func]
    pub fn take_damage(&mut self, pos: Vector2, dir: Vector2, damage: Gd<AttackArea>) {
        // godot_print!(
        //     "我 {}， 收到 {} 伤害, 方向: {}",
        //     self.base().get_name(),
        //     damage,
        //     dir
        // );
        self.hp -= damage.bind().get_damage();
        self.disabled();
        if self.hp > 0.0 {
            self.anim.play_ex().name("stun").done();
            self.stun_tween(dir);
            self.play_sound(&self.hit.clone(), pos);
        } else {
            //死球了
            self.anim.play_ex().name("death").done();
            self.play_sound(&self.death.clone(), pos);
        }
    }

    fn play_sound(&mut self, sound: &Option<Gd<AudioStream>>, pos: Vector2) {
        if let Some(sound) = sound {
            Message::singleton()
                .signals()
                .play_spatial_audio()
                .emit(sound, pos);
        }
    }

    pub fn stun_tween(&mut self, attack_dir: Vector2) {
        if let Some(mut tween) = self.tween.take() {
            tween.kill();
        }
        let mut tween = self.base_mut().create_tween();
        let pos = self.body.get_position();
        tween.tween_property(
            self.body.to_godot(),
            "position",
            &Vector2::new(attack_dir.x * 16.0, pos.y).to_variant(),
            0.2,
        );

        let new_dir = match attack_dir.x {
            a @ 0.0.. if a != 0.0 => -1.0,
            ..0.0 => 1.0,
            _ => self.dir,
        };
        self.set_dir(new_dir);

        tween.tween_property(self.body.to_godot(), "position", &pos.to_variant(), 0.2);
        tween.connect_flags(
            "finished",
            &Callable::from_object_method(&self.to_gd(), "on_stun_finished"),
            ConnectFlags::ONE_SHOT,
        );

        self.tween = Some(tween);
    }

    fn on_edge_detected(&mut self) {
        self.set_dir(-self.dir);
    }

    fn on_animation_finished(&mut self, anim: StringName) {
        match anim.to_string().as_str() {
            "death" => {
                // godot_print!("死球聊");
                self.base_mut().queue_free();
            }
            _ => {}
        }
    }

    #[func]
    pub fn on_stun_finished(&mut self) {
        self.anim.play_ex().name("walk").done();
        self.enabled();
    }

    fn disabled(&mut self) {
        self.damage_area.set_monitoring(false);
        self.attack_area.set_monitorable(false);
    }

    pub fn enabled(&mut self) {
        self.damage_area.set_monitoring(true);
        self.attack_area.set_monitorable(true);
    }

    #[func]
    fn get_dir(&self) -> f32 {
        self.dir
    }

    #[func]
    fn set_dir(&mut self, new_dir: f32) {
        self.dir = new_dir;
        self.update_direction();
    }

    fn update_direction(&mut self) {
        if self.dir > 0.0 {
            self.body.set_scale(Vector2::new(1.0, 1.0));
        } else if self.dir < 0.0 {
            self.body.set_scale(Vector2::new(-1.0, 1.0));
        }
    }
}
