use godot::{
    classes::{Button, Engine},
    prelude::*,
};

use crate::task::worker::Worker;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct TestBox {
    base: Base<Node2D>,
    #[export(file = "*.tscn")]
    scene_path: GString,
    #[init(val = None)]
    worker: Option<Box<Worker>>,
    #[init(node = "%Button")]
    button: OnReady<Gd<Button>>,
}

#[godot_api]
impl INode2D for TestBox {
    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }
        let mut worker = Worker::new();
        worker.run();
        self.worker = Some(Box::new(worker));
        self.button
            .signals()
            .pressed()
            .connect_other(&*self, Self::send_path);
    }

    fn process(&mut self, _delta: f64) {
        self.receive_msg();
    }
}

#[godot_api]
impl TestBox {
    fn send_path(&mut self) {
        if let Some(ref mut worker) = self.worker {
            let path = self.scene_path.clone();
            worker.send_to_worker(path.to_string().as_str());
        }
    }
    fn receive_msg(&mut self) {
        if let Some(ref mut worker) = self.worker {
            match worker.receive_from_worker() {
                i64::MIN => {}
                id => {
                    let resource_id = InstanceId::from_i64(id);
                    match Gd::<PackedScene>::try_from_instance_id(resource_id) {
                        Ok(scene_packed) => godot_print!("还原加载场景: {:?}", scene_packed),
                        Err(e) => godot_print!("无法还原内容: {:?}", e),
                    }
                }
            };
        }
    }
}
