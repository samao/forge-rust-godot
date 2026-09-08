use crate::states::{
    PlayerState, event::StateEvent, hurt::HurtState, idle::IdelState, run::RunState,
};
use godot::{classes::Input, prelude::*};

pub struct AttackState {
    combo: u32,
    hitbox_spawned: bool,
    timer: f32,
    duration: f32,
}

impl AttackState {
    pub fn new(combo: u32) -> Self {
        Self {
            combo,
            hitbox_spawned: false,
            timer: 0.0,
            duration: 0.35,
        }
    }
}

impl PlayerState for AttackState {
    fn enter(&mut self, player: &mut crate::player::Player) {
        godot_print!("[状态] 进入攻击 (连击{})", self.combo);
        player.play_anim(&format!("attack_{}", self.combo));
        player.start_timer("attack_timer", self.duration);
        // self.hitbox_spawned= false;
    }
    fn exit(&mut self, _player: &mut crate::player::Player) {
        godot_print!("[退出] 状态 攻击");
    }
    fn handle_event(
        &mut self,
        player: &mut crate::player::Player,
        event: super::event::StateEvent,
    ) -> Option<Box<dyn PlayerState>> {
        match event {
            StateEvent::Physics { delta } => {
                self.timer += delta;

                if !self.hitbox_spawned && self.timer > 0.12 {
                    self.hitbox_spawned = true;
                    player.spawn_attack_hitbox(30.0);
                    godot_print!("攻击判断框生成");
                }
            }
            StateEvent::InputJustPressed { action } if action == "attack" && self.combo < 1 => {
                return Some(Box::new(AttackState::new(self.combo + 1)));
            }
            StateEvent::TimerTimeout { timer_name } if timer_name == "attack_timer" => {
                let input = Input::singleton();
                let h = input.get_axis("left", "right");
                if h != 0.0 {
                    return Some(Box::new(RunState::new(h)));
                } else {
                    return Some(Box::new(IdelState::new()));
                }
            }
            StateEvent::TakeDamage { damage, knockback } => {
                player.apply_damage(damage, knockback);
                return Some(Box::new(HurtState::new(0.3)));
            }
            _ => {}
        }
        None
    }
}
