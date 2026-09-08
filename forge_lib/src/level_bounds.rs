use godot::{classes::Engine, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(tool, init, base = Node2D)]
pub struct LevelBounds {
    base: Base<Node2D>,
    #[export(range = (480.0, 2048.0, 32.0, suffix="px"))]
    #[var(set, get)]
    #[init(val = 480.0)]
    width: f32,
    #[export(range = (270.0, 2048.0, 32.0, suffix="px"))]
    #[var(set, get)]
    #[init(val = 270.0)]
    height: f32,
}

#[godot_api]
impl INode2D for LevelBounds {
    fn ready(&mut self) {
        self.base_mut().set_z_index(256);
        if Engine::singleton().is_editor_hint() {
            return;
        }
        // use godot::builtin::Side;
        // while let Some(view_port) = self.base().get_viewport() {
        //     if let Some(mut camera) = view_port.get_camera_2d() {
        //         let position = self.base().get_global_position();
        //         camera.set_limit(Side::LEFT, position.x as i32);
        //         camera.set_limit(Side::TOP, position.y as i32);
        //         camera.set_limit(Side::RIGHT, (position.x + self.width) as i32);
        //         camera.set_limit(Side::BOTTOM, (position.y + self.height) as i32);
        //         break;
        //     }
        // }

        self.try_bind_camera();
    }

    fn draw(&mut self) {
        if Engine::singleton().is_editor_hint() {
            let r = Rect2::new(Vector2::ZERO, Vector2::new(self.width, self.height));
            self.base_mut()
                .draw_rect_ex(r, Color::from_rgb(0.0, 0.45, 1.0))
                .filled(false)
                .width(3.0)
                .done();
        }
    }
}

#[godot_api]
impl LevelBounds {
    fn try_bind_camera(&mut self) {
        let lb = self.to_gd().clone();
        let position = lb.get_global_position();
        godot_print!("绑定摄像头");
        use godot::builtin::Side;
        if let Some(view_port) = lb.get_viewport() {
            if let Some(mut camera) = view_port.get_camera_2d() {
                camera.set_limit(Side::LEFT, position.x as i32);
                camera.set_limit(Side::TOP, position.y as i32);
                camera.set_limit(Side::RIGHT, (position.x + self.width) as i32);
                camera.set_limit(Side::BOTTOM, (position.y + self.height) as i32);
                self.base_mut().call_deferred("queue_free", &[]);
                return;
            }
        };

        let timeout = self
            .base()
            .get_tree()
            .create_timer(0.05)
            .signals()
            .timeout()
            .to_future();
        let mut gdnode = self.to_gd().clone();
        godot::task::spawn(async move {
            timeout.await;
            gdnode.bind_mut().try_bind_camera();
        });
    }

    #[func]
    fn set_width(&mut self, v: f32) {
        self.width = v;
        self.base_mut().queue_redraw();
    }
    #[func]
    fn set_height(&mut self, v: f32) {
        self.height = v;
        self.base_mut().queue_redraw();
    }

    #[func]
    pub fn get_width(&self) -> f32 {
        self.width
    }
    #[func]
    pub fn get_height(&self) -> f32 {
        self.height
    }
}
