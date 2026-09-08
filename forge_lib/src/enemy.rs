use std::fs;

use godot::{prelude::*, task::TaskHandle};

use crate::AsyncHandle;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
struct Enemy {
    base: Base<Node2D>,
    handle: Option<TaskHandle>,
}

impl Enemy {
    fn godot_path_to_system(gd_path: &str) -> String {
        let project = godot::classes::ProjectSettings::singleton();
        project.globalize_path(gd_path).to_string()
    }

    fn read_file(path: &str) -> Result<String, std::io::Error> {
        let sys_path = Enemy::godot_path_to_system(path);
        fs::read_to_string(sys_path)
    }

    fn write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
        let sys_path = Enemy::godot_path_to_system(path);
        fs::write(sys_path, content)
    }
}

#[godot_api]
impl INode2D for Enemy {
    fn enter_tree(&mut self) {
        let physics_frame = self.base().get_tree().signals().physics_frame().to_future();
        let timeout = self
            .base()
            .get_tree()
            .create_timer(0.05)
            .signals()
            .timeout()
            .to_future();
        self.handle = Some(godot::task::spawn(async move {
            godot_print!("spawn");
            physics_frame.await;
            timeout.await;
            godot_print!("end spawn");
            AsyncHandle::singleton()
                .bind_mut()
                .send("我是一个敌人，我要打你".into());
            AsyncHandle::singleton().bind_mut().add_task(|| {
                godot_print!("工作线程调用了，我的函数");
            });

            AsyncHandle::singleton().bind_mut().add_job(
                Box::new(|| Box::new("你是我的老Baby")),
                Box::new(|msg| {
                    if let Some(msg) = msg.downcast_ref::<&str>() {
                        godot_print!("收到了后台处理结果: {:?}", msg);
                    } else {
                        godot_print!("无法转换 : {:?}", msg);
                    }
                }),
            );
            if let Err(err) = Enemy::write_file("user://save.data", "我是个大牙之人") {
                godot_print!("吸入一次：{:?}", err);
            };
            if let Ok(msg) = Enemy::read_file("user://save.data") {
                godot_print!("读取了文件: {msg}")
            } else {
                godot_print!("无法读取");
            }
        }));
    }

    fn process(&mut self, _delta: f64) {
        if let Some(handle) = &self.handle {
            if handle.is_pending() {
                godot_print!("等着呢");
            } else {
                godot_print!("开通权限");
                self.base_mut().call_deferred("queue_free", &[]);
            }
        }
    }

    fn exit_tree(&mut self) {
        if let Some(handle) = self.handle.take() {
            godot_print!("退出了enemy");
            handle.cancel();
        }
    }
}
