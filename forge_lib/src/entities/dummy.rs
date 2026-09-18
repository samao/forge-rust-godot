use godot::{
    classes::{Sprite2D, Tween, object::ConnectFlags, tween::EaseType},
    obj::WithBaseField,
    prelude::*,
};

use crate::{
    entities::attack::AttackArea, message::Message, resource::particles::HitParticleSetting,
};

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Dummy {
    base: Base<Node2D>,
    #[export]
    #[init(val = 4)]
    shake_count: i32,
    #[export]
    #[init(val = 5.0)]
    degree: f32,
    #[export]
    #[init(val = 0.1)]
    speed: f32,

    #[init(val = 0)]
    current_count: i32,

    #[export]
    #[init( val = Array::new())]
    particles: Array<Gd<HitParticleSetting>>,

    #[init(node = "%Body")]
    body: OnReady<Gd<Sprite2D>>,

    hit_dir: Vector2,

    current_tween: Option<Gd<Tween>>,
}

#[godot_api]
impl INode2D for Dummy {
    // fn ready(&mut self) {
    //     self.base_mut().set_process_mode(ProcessMode::DISABLED);
    // }
    // fn process(&mut self, delta: f64) {
    //     if self.current_count <= 0 {
    //         return;
    //     }
    //     let cur_degree = self.base().get_rotation_degrees();
    //     let to_deg = self.shake_count as f32 * self.degree * self.hit_dir.x;
    //     let next_deg = cur_degree.lerp(to_deg, 0.5);

    //     if (next_deg - cur_degree).abs() < 0.1 {
    //         self.hit_dir.x *= -1.0;
    //         self.shake_count -= 1;
    //     } else {
    //         self.base_mut().set_rotation_degrees(next_deg);
    //     }
    // }
}

#[godot_api]
impl Dummy {
    #[func]
    pub fn take_damage(&mut self, pos: Vector2, dir: Vector2, _damage: Gd<AttackArea>) {
        if let Some(mut tween) = self.current_tween.take() {
            tween.kill();
        }
        self.hit_dir = dir.normalized();
        self.current_count = self.shake_count;
        for p in self.particles.iter_shared() {
            Message::singleton()
                .signals()
                .play_particles()
                .emit(pos, dir, &p);
        }

        self.shake();
        self.disable_attack_area();
    }

    fn disable_attack_area(&self) {
        if let Some(parent) = self.base().get_parent() {
            if let Some(attack_node) = parent
                .find_children_ex("*")
                .type_("AttackArea")
                .done()
                .front()
                && let Ok(ref mut attack_area) = attack_node.try_cast::<AttackArea>()
            {
                attack_area.bind_mut().set_active(false);
                let a_id = attack_area.instance_id();
                let timer = self
                    .base()
                    .get_tree()
                    .create_timer(0.5)
                    .signals()
                    .timeout()
                    .to_future();

                godot::task::spawn(async move {
                    timer.await;
                    // godot_print!("受伤状态恢复");
                    let mut attack_area = Gd::<AttackArea>::from_instance_id(a_id);
                    attack_area.bind_mut().set_active(true);
                });
            }
        }
    }

    fn shake(&mut self) {
        // godot_print!("闪： {}, 方向: {}", self.current_count, self.hit_dir.x);
        let mut tween = self.base_mut().create_tween();
        tween.set_ease(EaseType::IN_OUT);
        let to_deg = self.degree * self.current_count as f32 * self.hit_dir.x;
        tween.tween_property(&*self.body, "rotation_degrees", &to_deg.to_variant(), 0.2);

        tween.connect_flags(
            "finished",
            &Callable::from_object_method(&self.to_gd(), "next_shake"),
            ConnectFlags::ONE_SHOT,
        );

        self.current_tween = Some(tween);
    }

    #[func]
    fn next_shake(&mut self) {
        self.current_count -= 1;
        self.hit_dir *= -1.0;

        if self.current_count >= 0 {
            self.shake();
        } else {
            self.hit_dir = Vector2::ZERO;
            self.current_count = 0;
            self.base_mut().set_rotation_degrees(0.0);
        }
    }
}
