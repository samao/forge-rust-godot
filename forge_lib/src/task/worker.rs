use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread::{self, JoinHandle},
    time::Duration,
};

use godot::{classes::ResourceLoader, global::godot_print, obj::Singleton};

pub struct Worker {
    main_sender: Option<Box<Sender<String>>>,
    worker_receiver: Option<Box<Receiver<i64>>>,
    handle: Option<Box<JoinHandle<()>>>,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            main_sender: None,
            worker_receiver: None,
            handle: None,
        }
    }
    pub fn run(&mut self) {
        let (main_sender, main_rx) = channel::<String>();
        let (worker_sender, worker_rx) = channel();
        let handle = thread::spawn(move || {
            loop {
                if let Ok(msg) = main_rx.try_recv() {
                    match msg.as_str() {
                        "abort" => break,
                        msg => {
                            godot_print!("加载场景: {msg}");

                            if let Some(resource) = ResourceLoader::singleton().load(msg) {
                                let r_id = resource.instance_id();
                                godot_print!("加载成功后： {:?}", r_id);
                                let _ = worker_sender.send(r_id.to_i64());
                            } else {
                                godot_print!("加载失败: {msg}");
                            }
                        }
                    }
                }
                use std::thread::sleep;
                sleep(Duration::from_secs_f64(0.016));
            }
        });

        self.main_sender = Some(Box::new(main_sender));
        self.worker_receiver = Some(Box::new(worker_rx));
        self.handle = Some(Box::new(handle));
    }

    pub fn stop(&mut self) {
        if let Some(handle) = self.handle.take() {
            self.send_to_worker("abort");
            handle.join().ok();
        }
    }

    pub fn send_to_worker(&mut self, scene_path: &str) {
        if let Some(ref mut main_sender) = self.main_sender {
            let _ = main_sender.send(scene_path.to_owned());
        }
    }

    pub fn receive_from_worker(&mut self) -> i64 {
        if let Some(ref mut worker_rx) = self.worker_receiver {
            if let Ok(msg) = worker_rx.try_recv() {
                godot_print!("收到id: {msg}");
                return msg;
            }
        }
        i64::MIN
    }
}
