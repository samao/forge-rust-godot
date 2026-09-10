use godot::{classes::Engine, prelude::*};
use std::{any::Any, sync::mpsc, thread, time::Duration};
use tokio::time::sleep;

struct ForgeExtension;

#[derive(GodotClass)]
#[class(init, singleton)]
pub struct AsyncHandle {
    runtime: Option<thread::JoinHandle<()>>,
    receiver: Option<mpsc::Receiver<String>>,
    task_handle: Option<mpsc::Sender<AsyncData>>,
}

type Job = Box<dyn FnOnce() -> Box<dyn Any + Send> + Send + 'static>;
type Callback = Box<dyn FnOnce(Box<dyn Any + Send>) + Send + 'static>;

pub enum AsyncData {
    Abort,
    Msg(String),
    Fn(Box<dyn FnMut() -> () + Send + 'static>),
    Task { job: Job, callback: Callback },
}

impl AsyncHandle {
    pub fn run(&mut self) -> &mut Self {
        let time_gap = 1.0 / Engine::singleton().get_physics_ticks_per_second() as f64;
        godot_print!("后台线程刷新率：{time_gap}");
        if self.runtime.is_none() && !Engine::singleton().is_editor_hint() {
            let (shutdown_tx, shutdown_rx) = mpsc::channel();
            let (result_tx, result_rx) = mpsc::channel();

            let handle = thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(async {
                    let send_task = tokio::spawn(async move {
                        loop {
                            match shutdown_rx.try_recv() {
                                Ok(AsyncData::Abort) => {
                                    break;
                                }
                                Ok(AsyncData::Msg(msg)) => {
                                    godot_print!("收到主线程消息: {}", msg);
                                }
                                Ok(AsyncData::Fn(mut call_fn)) => {
                                    (*call_fn)();
                                }
                                Ok(AsyncData::Task { job, callback }) => {
                                    let result = job();
                                    callback(result);
                                }
                                _ => {}
                            }

                            if result_tx.send("detector...".to_string()).is_err() {
                                break;
                            }
                            sleep(Duration::from_secs_f64(time_gap)).await;
                        }
                    });

                    let _ = send_task.await;
                });
            });
            self.runtime = Some(handle);
            self.receiver = Some(result_rx);
            self.task_handle = Some(shutdown_tx);
            godot_print!("Async Ready!");
        }
        self
    }

    pub fn terminate(&mut self) {
        godot_print!("AsyncHandle terminate!");
        if let Some(handle) = self.task_handle.take() {
            let _ = handle.send(AsyncData::Abort);
        }

        if let Some(handle) = self.runtime.take() {
            let _ = handle.join();
        }
        self.receiver = None;
    }

    pub fn send(&mut self, msg: String) {
        if let Some(ref mut handle) = self.task_handle {
            let _ = handle.send(AsyncData::Msg(msg));
        }
    }

    pub fn add_task<T>(&mut self, call_fn: T)
    where
        T: FnMut() -> () + Send + 'static,
    {
        if let Some(ref mut handle) = self.task_handle {
            let _ = handle.send(AsyncData::Fn(Box::new(call_fn)));
        }
    }

    pub fn receive(&mut self) -> Option<String> {
        if let Some(ref mut receiver) = self.receiver {
            return receiver.try_recv().ok();
        }
        None
    }

    pub fn add_job(&mut self, job: Job, callback: Callback) {
        if let Some(ref mut handle) = self.task_handle {
            godot_print!("发送任务给后台执行");
            handle.send(AsyncData::Task { job, callback }).ok();
        }
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for ForgeExtension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            godot_print!("rust 扩展加载完毕");
            AsyncHandle::singleton().bind_mut().run();
        }
    }

    fn on_stage_deinit(stage: InitStage) {
        // godot_print!("销毁all {:?}", stage);
        if stage == InitStage::Scene {
            godot_print!("rust 扩展卸载了");
            AsyncHandle::singleton().bind_mut().terminate();
        }
    }

    fn on_main_loop_frame() {
        if let Some(_msg) = AsyncHandle::singleton().bind_mut().receive() {
            // godot_print!("收到异步消息：{}", msg);
        }
    }
}

pub mod enemy;
pub mod player;
pub mod states;

pub mod entities;
pub mod level;
pub mod level_bounds;
pub mod level_transition;
pub mod managers;
pub mod message;
pub mod transition_mark;
pub mod ui;
