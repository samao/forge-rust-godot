use godot::{
    classes::{
        AnimationPlayer, AudioStream, CharacterBody2D, Engine, ICharacterBody2D, Sprite2D,
        notify::CanvasItemNotification,
    },
    prelude::*,
};

use crate::{
    entities::{attack::AttackArea, damage::DamageArea, edge_detector::EdgeDetector},
    message::Message,
    monster::{
        blackboard::Blackboard, decision_engine::DecisionEngine,
        enemy_state_machine::EnemyStateMachine,
    },
};

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub(crate) struct Enemy {
    base: Base<CharacterBody2D>,

    #[export]
    #[init(val = 3.0)]
    health: f32,

    #[export]
    #[init(val = true)]
    affected_by_gravity: bool,

    #[export]
    #[init(val = false)]
    #[var(pub)]
    face_left_on_start: bool,

    #[export_group(name = "Audio")]
    #[export]
    death_sound: Option<Gd<AudioStream>>,
    #[export]
    hit_sound: Option<Gd<AudioStream>>,

    //normal var
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

    //ai
    #[init(node = "%StateMachine")]
    state_machine: OnReady<Gd<EnemyStateMachine>>,
    #[init(node = "%DecisionEngine")]
    decision_engine: OnReady<Gd<DecisionEngine>>,
    #[init(val = None)]
    blackboard: Option<Gd<Blackboard>>,
}

#[godot_api]
impl Enemy {
    #[signal]
    fn direction_change(new_dir: Vector2);
    #[signal]
    fn was_hit();
    #[signal]
    fn was_killed();
    #[func]
    fn take_damage(&mut self, pos: Vector2, _dir: Vector2, attack_area: Gd<AttackArea>) {
        if let Some(board) = self.blackboard.as_mut() {
            board
                .bind_mut()
                .set_damage_source(Some(attack_area.clone()));
            let health = board.bind().get_health() - attack_area.bind().get_damage();
            board.bind_mut().set_health(health);

            if let Some(ref audio) = if health <= 0.0 {
                //同步取消？
                self.damage_area.queue_free();
                self.attack_area.queue_free();
                self.signals().was_killed().emit();
                self.death_sound.clone()
            } else {
                self.hit_sound.clone()
            } {
                Message::singleton()
                    .signals()
                    .play_spatial_audio()
                    .emit(audio, pos);
                self.turn_back();
            }
        }
        self.signals().was_hit().emit();
    }
}

impl Enemy {
    fn setup(&mut self) {
        let mut blackboard = Blackboard::new_gd();
        blackboard.bind_mut().set_health(self.health);
        self.blackboard = Some(blackboard.clone());
        self.state_machine.bind_mut().setup(blackboard.clone());
        self.decision_engine.bind_mut().setup(blackboard);

        self.edge_detector
            .signals()
            .edge_detect()
            .connect_other(&*self, Self::on_edge_detected);
    }

    fn on_edge_detected(&mut self) {
        self.turn_back();
    }

    fn turn_back(&mut self) {
        if let Some(board) = self.blackboard.as_mut() {
            let new_dir = board.bind().get_dir();
            self.change_dir(-new_dir);
        }
    }

    pub(crate) fn change_dir(&mut self, new_dir: Vector2) {
        if let Some(board) = self.blackboard.as_mut() {
            board.bind_mut().set_dir(new_dir);
        }

        self.signals().direction_change().emit(new_dir);
        match new_dir {
            Vector2 { x: ..0.0, .. } => {
                self.body.set_scale(Vector2::new(-1.0, 1.0));
            }
            Vector2 {
                x: xpos @ 0.0.., ..
            } if xpos > 0.0 => {
                self.body.set_scale(Vector2::new(1.0, 1.0));
            }
            _ => {}
        }
    }

    pub(super) fn get_current_animation_length(&self) -> f64 {
        self.anim.get_current_animation_length()
    }

    pub(super) fn play_animation(&mut self, anim_name: &str) {
        if self.anim.has_animation(anim_name) {
            godot_print!("Enemy 播放动画: {anim_name}");
            if self.anim.get_current_animation() == anim_name {
                self.anim.seek(0.0);
                return;
            }
            self.anim.play_ex().name(anim_name).done();
        } else {
            godot_error!("animation missing: {}", anim_name);
        }
    }

    fn check_node(&self, node_type: &str) -> Result<(), ()> {
        if self
            .base()
            .find_children_ex("*")
            .type_(node_type)
            .recursive(false)
            .done()
            .is_empty()
        {
            return Err(());
        }

        Ok(())
    }
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            self.base_mut().set_physics_process(false);
            self.base_mut().set_process(false);
            return;
        }
        self.setup();
    }

    fn physics_process(&mut self, delta: f64) {
        if let Some(next_state) = self.decision_engine.bind_mut().decide() {
            self.state_machine.bind_mut().change_state(next_state);
            // godot_print!("跳转状态: {}", next_state.get_name());
        }

        if self.affected_by_gravity {
            let mut velocity = self.base().get_velocity();
            velocity += self.base().get_gravity() * Vector2::splat(delta as f32);
            self.base_mut().set_velocity(velocity);
        }

        self.base_mut().move_and_slide();
    }

    fn on_notification(&mut self, what: CanvasItemNotification) {
        match what {
            CanvasItemNotification::EXTENSION_RELOADED => {}
            _ => {}
        }
    }

    fn get_configuration_warnings(&self) -> PackedStringArray {
        let mut warnings: PackedArray<GString> = PackedArray::new();
        if self.check_node("AnimationPlayer").is_err() {
            warnings.push("Requires an AnimationPlayer!");
        }
        if self.check_node("Sprite2D").is_err() {
            warnings.push("Requires a Sprite2D!");
        }
        if self.check_node("DamageArea").is_err() {
            warnings.push("Requires a DamageArea node!");
        }
        if self.check_node("AttackArea").is_err() {
            warnings.push("Requires a AttackArea node!");
        }
        if self.check_node("EnemyStateMachine").is_err() {
            warnings.push("Requires a EnemyStateMachine node!");
        }
        if self.check_node("DecisionEngine").is_err() {
            warnings.push("Requires a DecisionEngine node!");
        }
        warnings
    }
}
