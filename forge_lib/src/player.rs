use std::collections::VecDeque;

use godot::classes::node::ProcessMode;
use godot::classes::{
    AnimationPlayer, CharacterBody2D, CollisionShape2D, Engine, ICharacterBody2D, Input,
    InputEvent, InputEventKey, Light2D, Os, ShapeCast2D, Sprite2D,
};
use godot::global::Key;
use godot::obj::{Singleton, WithBaseField};
use godot::prelude::*;
use godot::tools::try_get_autoload_by_name;

use crate::entities::attack::AttackArea;
use crate::entities::damage::DamageArea;
use crate::level::{Level, SceneTheme};
use crate::managers::audio_manager::AudioManager;
use crate::managers::scene_manager::SceneManager;
use crate::message::Message;
use crate::resource::sounds::SoundSource;
use crate::states::PlayerState;
use crate::states::die::DieState;
use crate::states::event::StateEvent;
use crate::states::idle::IdelState;

pub mod player_spawn;

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,

    #[var]
    pub speed: f32,
    #[var]
    pub jump_velocity: f32,

    pub gravity: f32,

    pub hp: f32,
    pub max_hp: f32,
    pub dash: bool,
    pub double_jump: bool,
    pub ground_slam: bool,
    pub morph_roll: bool,

    current_state: Option<Box<dyn PlayerState>>,
    event_queue: VecDeque<StateEvent>,

    timers: Vec<(StringName, f32)>,

    health: f32,
    is_invincible: bool,
    invincible_timer: f32,

    pub coyote_timer: f32,
    coyote_duration: f32,

    one_way_ray: Option<Gd<ShapeCast2D>>,
    animation_player: Option<Gd<AnimationPlayer>>,
    sprite: Option<Gd<Sprite2D>>,

    light: Option<Gd<Light2D>>,

    direction: Vector2,

    slam_attack: Option<Gd<AttackArea>>,
    attack_area: Option<Gd<AttackArea>>,

    #[export]
    sounds: Option<Gd<SoundSource>>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        let player = Self {
            base,
            speed: 200.0,
            jump_velocity: -500.0,
            gravity: 1200.0,
            current_state: None,
            event_queue: VecDeque::new(),
            timers: Vec::new(),
            health: 100.0,
            is_invincible: false,
            invincible_timer: 0.0,
            coyote_timer: 0.0,
            coyote_duration: 0.08,
            hp: 5.0,
            max_hp: 20.0,
            double_jump: false,
            dash: false,
            morph_roll: false,
            ground_slam: false,
            one_way_ray: None,
            animation_player: None,
            sprite: None,
            light: None,
            direction: Vector2::RIGHT,
            attack_area: None,
            sounds: None,
            slam_attack: None,
        };
        // player.switch_state(IdelState::new());
        godot_print!("Rust 玩家已经初始化");
        player
    }
    fn ready(&mut self) {
        self.base_mut().call_deferred("turn_light", &[]);
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.one_way_ray = self.base().try_get_node_as::<ShapeCast2D>("ShapeCast2D");
        self.sprite = self.base().try_get_node_as::<Sprite2D>("Sprite2D");
        self.light = self.base().try_get_node_as::<Light2D>("PointLight2D");
        self.attack_area = self.base().try_get_node_as::<AttackArea>("%AttackArea");
        self.slam_attack = self.base().try_get_node_as::<AttackArea>("%SlamAttackArea");
        self.animation_player = self
            .base()
            .try_get_node_as::<AnimationPlayer>("AnimationPlayer");
        self.switch_state(IdelState::new());

        if let Some(transition) = self
            .base()
            .try_get_node_as::<SceneManager>("/root/SceneTransition")
        {
            transition
                .signals()
                .load_scene_finished()
                .connect_other(&*self, Self::turn_light);
        }

        self.signals().tree_entered().connect_self(Self::turn_light);

        let wait_frame = self.base().get_tree().signals().process_frame().to_future();
        godot::task::spawn(async move {
            wait_frame.await;
            godot_print!("这里等了一帧");
        });

        Message::singleton()
            .signals()
            .game_end()
            .connect_other(&*self, Self::release_player);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        for action in &["jump", "attack", "down", "dash", "roll"] {
            if event.clone().is_action_pressed(*action) {
                self.event_queue.push_back(StateEvent::InputJustPressed {
                    action: (*action).into(),
                });
            }
            if event.is_action_released(*action) {
                self.event_queue.push_back(StateEvent::InputJustRelease {
                    action: (*action).into(),
                });
            }
        }
        if event.is_action_pressed("action") {
            Message::singleton().signals().interactive().emit();
            return;
        }
        if event.is_action_pressed("pause") {
            Message::singleton().signals().toggle_pause().emit();
            return;
        }
        if Os::singleton().is_debug_build() {
            if let Ok(event) = event.try_cast::<InputEventKey>()
                && event.is_pressed()
            {
                let current_hp = self.hp;
                // godot_print!("键盘输入 {:?}", event.get_keycode() == Key::MINUS);
                if event.get_keycode() == Key::MINUS {
                    self.set_hp(current_hp - 2.0);
                }
                if event.get_keycode() == Key::EQUAL {
                    self.set_hp(current_hp + 2.0);
                }
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        self.update_direction();
    }

    fn physics_process(&mut self, delta: f64) {
        let dt = delta as f32;
        let input = Input::singleton();

        for action in &["jump", "attack", "down", "dash", "action"] {
            if input.is_action_pressed(*action) {
                self.event_queue.push_back(StateEvent::InputPressed {
                    action: (*action).into(),
                });
            }
        }

        self.event_queue
            .push_back(StateEvent::Physics { delta: dt });

        let mut to_remove = Vec::new();
        for (i, (name, remaining)) in self.timers.iter_mut().enumerate() {
            *remaining -= dt;
            if *remaining <= 0.0 {
                // godot_print!("一个超时发生: {:?}", name);
                self.event_queue.push_back(StateEvent::TimerTimeout {
                    timer_name: name.clone(),
                });

                to_remove.push(i);
            }
        }

        for i in to_remove.into_iter().rev() {
            self.timers.remove(i);
        }

        if self.is_invincible {
            self.invincible_timer -= dt;

            if self.invincible_timer <= 0.0 {
                self.is_invincible = false;
                godot_print!("无敌结束");
            }
        }

        let mut iterations = 0;
        while let Some(event) = self.event_queue.pop_front() {
            iterations += 1;

            if iterations > 50 {
                godot_print!("时间队列过深，强制打断");
                break;
            }

            if let Some(mut state) = self.current_state.take() {
                match state.handle_event(self, event) {
                    Some(new_state) => {
                        state.exit(self);
                        self.switch_state_with_box(new_state);
                    }
                    None => {
                        self.current_state = Some(state);
                    }
                }
            }
        }

        if !self.base().is_on_floor() {
            let mut v = self.base().get_velocity();
            v.y += self.gravity * dt;
            self.base_mut().set_velocity(v);
        }
        self.base_mut().move_and_slide();

        //土狼时间
        if self.base().is_on_floor() {
            self.coyote_timer = self.coyote_duration;
        } else {
            self.coyote_timer = (self.coyote_timer - dt).max(0.0);
        }
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn die();

    pub fn get_face_direction(&self) -> f32 {
        self.sprite
            .clone()
            .map(|sprite| sprite.get_scale().x)
            .unwrap_or(1.0)
    }

    fn release_player(&mut self) {
        godot_print!("销毁玩家");
        self.base_mut().call_deferred("queue_free", &[]);
    }

    pub fn set_gravity_disable(&mut self, v: bool) {
        self.gravity = if !v { 1200.0 } else { 0.0 };
    }

    #[func]
    fn set_hp(&mut self, v: f32) {
        self.hp = v.clamp(0.0, self.max_hp);
        Message::singleton()
            .signals()
            .player_health_change()
            .emit(self.hp, self.max_hp);
        if self.hp <= 0.0 {
            self.signals().die().emit();
            self.switch_state(DieState::new());
            self.set_player_disable(true);

            Message::singleton().signals().game_over().emit();
            self.release_player();
        }
    }

    pub fn set_player_disable(&mut self, v: bool) {
        godot_print!("关闭受伤检测");
        if let Some(ref mut node) = self.base().try_get_node_as::<DamageArea>("%DamageArea") {
            node.set_process_mode(if v {
                ProcessMode::DISABLED
            } else {
                ProcessMode::INHERIT
            });
        } else {
            godot_print!("不存在检测组件");
        }
    }
    #[func]
    fn set_max_hp(&mut self, v: f32) {
        self.max_hp = v;
        Message::singleton()
            .signals()
            .player_health_change()
            .emit(self.hp, self.max_hp);
    }

    #[func]
    fn turn_light(&mut self) {
        // godot_print!("turn light");
        if let Some(node) = self.base().get_tree().get_first_node_in_group("LevelTheme") {
            if let Ok(lt) = node.try_cast::<Level>()
                && let Some(ref mut light) = self.light
            {
                match lt.bind().theme {
                    SceneTheme::Light => {
                        light.set_enabled(false);
                    }
                    SceneTheme::Dark => {
                        light.set_enabled(true);
                    }
                }
            }
        } else {
            godot_print!("there is no level node");
        }
    }

    #[func]
    pub fn attack(&self) {
        godot_print!("Player Attack!!!");
    }

    pub fn is_one_way_collidering(&self) -> bool {
        if let Some(mut ray) = self.one_way_ray.clone() {
            ray.force_shapecast_update();
            return ray.is_colliding();
        }

        false
    }

    fn update_direction(&mut self) {
        let input = Input::singleton();
        let x_axis = input.get_axis("left", "right");
        let y_axis = input.get_axis("down", "up");
        let velocity = Vector2::new(x_axis, y_axis);

        if self.direction.x == x_axis {
            return;
        }

        if let Some(mut sprite) = self.sprite.clone() {
            match x_axis {
                a @ 0.0.. if a > 0.0 => {
                    sprite.set_scale(Vector2::new(1.0, 1.0));
                }
                ..0.0 => {
                    sprite.set_scale(Vector2::new(-1.0, 1.0));
                }
                _ => {}
            }
        }

        self.direction = velocity;
    }

    pub fn prepar_fall(&mut self) {
        // godot_print!("从平台跳下");
        let mut position = self.base().get_global_position();
        position.y += 4.0;
        self.base_mut().set_global_position(position);
    }

    fn switch_state<S: PlayerState + 'static>(&mut self, state: S) {
        self.switch_state_with_box(Box::new(state));
    }

    fn switch_state_with_box(&mut self, mut state: Box<dyn PlayerState>) {
        state.enter(self);
        self.current_state = Some(state);
    }

    pub fn play_sound_effect(&mut self, s_type: &str) {
        let pos = self.base().get_global_position();
        if let Some(sound) = self.sounds.clone()
            && let Ok(sound) = sound.try_cast::<SoundSource>()
        {
            if let Some(sound) = match s_type {
                "dash" => sound.bind().dash.clone(),
                "attack" => sound.bind().attack.clone(),
                "jump" => sound.bind().jump.clone(),
                "land" => sound.bind().land.clone(),
                "slam" => sound.bind().slam.clone(),
                "morph" => sound.bind().morph.clone(),
                _ => None,
            } {
                Message::singleton()
                    .signals()
                    .play_spatial_audio()
                    .emit(&sound, pos);
            }
        } else {
            godot_print!("无法播放");
        }
    }

    pub fn create_tail_shadow(&mut self) {
        if let Some(mut parent) = self.base().get_parent() {
            if let Some(ref body) = self.sprite {
                let pos = body.get_global_position();
                let mut tail = body.duplicate_node();
                tail.set_global_position(pos);
                parent.add_child(&tail);
                self.create_tail_tween(tail.upcast());
            }
        }
    }

    fn create_tail_tween(&mut self, mut tail: Gd<Node2D>) {
        let mut tween = self.base_mut().create_tween();
        let color = tail.get_modulate();
        tween.tween_property(&tail, "modulate", &color.with_alpha(0.0).to_variant(), 0.2);
        let finished = tween.signals().finished().to_future();
        godot::task::spawn(async move {
            finished.await;
            tail.queue_free();
        });
    }

    pub fn set_ver_speed(&mut self, y: f32) {
        let mut v = self.base().get_velocity();
        v.y = y;
        self.base_mut().set_velocity(v);
    }

    pub fn set_horizontal_speed(&mut self, x: f32) {
        let mut v = self.base().get_velocity();
        v.x = x;
        self.base_mut().set_velocity(v);
    }

    pub fn set_attack_enabled(&mut self, v: bool) {
        if let Some(ref mut attack_area) = self.attack_area {
            attack_area.bind_mut().set_active(v);
        }
    }

    pub fn play_anim(&self, name: &str) {
        // godot_print!("播放动画: {}", name);
        match self.animation_player.clone() {
            Some(mut anim) if anim.has_animation(name) => {
                anim.play_ex().name(name).done();
            }
            _ => {}
        }
    }

    pub fn start_timer(&mut self, name: &str, duration: f32) {
        // godot_print!("添加一个计时：{} -> {}", name, duration);
        self.timers.push((name.into(), duration));
    }

    pub fn spawn_attack_hitbox(&self, _damage: f32) {
        // godot_print!("生成攻击判定框，伤 {}", damage);
    }

    pub fn set_slam_active(&mut self, v: bool) {
        godot_print!("设置slam attack, {v}");
        if let Some(mut attack) = self.slam_attack.clone() {
            attack.set_monitorable(v);
        }
    }

    pub fn set_morph_roll(&mut self, v: bool) {
        if let Some(mut stand_shape) = self
            .base()
            .try_get_node_as::<CollisionShape2D>("%Stand_Collision")
            && let Some(mut morph_shape) = self
                .base()
                .try_get_node_as::<CollisionShape2D>("%Morph_Roll_Collision")
        {
            stand_shape.set_disabled(v);
            morph_shape.set_disabled(!v);
        } else {
            godot_print!("没有碰撞形状");
        }
    }

    #[func]
    pub fn take_damage(&mut self, _pos: Vector2, dir: Vector2, damage: f32) {
        // godot_print!("你敢扎我: {damage}");
        let next_velocity = Vector2::splat(self.speed * 0.8) * dir.normalized();
        self.base_mut().set_velocity(next_velocity);
        self.event_queue.push_back(StateEvent::TakeDamage {
            damage,
            knockback: dir,
        });
        let next_hp = (self.hp - damage).clamp(0.0, self.max_hp);
        Message::singleton().signals().camera_shake().emit(10.0);
        self.set_hp(next_hp);
    }

    pub fn apply_damage(&mut self, damage: f32, knockback: Vector2) {
        if self.is_invincible {
            return;
        }
        self.health -= damage;
        self.is_invincible = true;
        self.invincible_timer = 0.2;
        // godot_print!("受伤了，剩余血量：{}", self.health);

        let mut v = self.base().get_velocity();
        v.x += knockback.x;
        v.y = knockback.y;
        self.base_mut().set_velocity(v);

        if self.health <= 0.0 {
            // godot_print!("玩家死亡");
            self.signals().die().emit();
        }
    }

    pub fn play_sound(&self, sound_type: &str) {
        if let Some(ref sounds) = self.sounds.clone() {
            if let Some(sound) = match sound_type {
                "attack" => sounds.bind().attack.clone(),
                "jump" => sounds.bind().jump.clone(),
                "land" => sounds.bind().land.clone(),
                _ => unreachable!(),
            } {
                if let Ok(mut audio_helper) =
                    try_get_autoload_by_name::<AudioManager>("AudioHelper")
                {
                    audio_helper
                        .bind_mut()
                        .play_spatial_sound(sound, self.base().get_global_position());
                }
            }
        }
    }
}
