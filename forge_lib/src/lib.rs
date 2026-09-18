use godot::prelude::*;

struct ForgeExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ForgeExtension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            godot_print!("rust 扩展加载完毕");
            // AsyncHandle::singleton().bind_mut().run();
        }
    }

    fn on_stage_deinit(stage: InitStage) {
        // godot_print!("销毁all {:?}", stage);
        if stage == InitStage::Scene {
            godot_print!("rust 扩展卸载了");
            // AsyncHandle::singleton().bind_mut().terminate();
        }
    }

    fn on_main_loop_frame() {
        // if let Some(_msg) = AsyncHandle::singleton().bind_mut().receive() {
        //     // godot_print!("收到异步消息：{}", msg);
        // }
    }
}

pub mod monster;
pub mod player;
pub mod states;
// pub mod task;
// use task::AsyncHandle;

pub mod entities;
pub mod level;
pub mod level_bounds;
pub mod level_transition;
pub mod managers;
pub mod message;
pub mod resource;
pub mod transition_mark;
pub mod ui;
