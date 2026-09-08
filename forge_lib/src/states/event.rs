use godot::classes::Area2D;
use godot::prelude::*;

pub enum StateEvent {
    Physics { delta: f32 },
    InputJustPressed { action: StringName },
    InputJustRelease { action: StringName },
    InputPressed { action: StringName },
    InputRelease { action: StringName },

    TimerTimeout { timer_name: StringName },
    AnimationFinished { animation_name: StringName },
    AnimationFrameChanged { frame: i32 },

    BodyEntered { body: Gd<Node2D> },
    AreaEntered { area: Gd<Area2D> },

    //custom event
    TakeDamage { damage: f32, knockback: Vector2 },
    Heal { amount: f32 },
    PickupItem { item_type: StringName },
}
