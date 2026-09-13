use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct TestBox {
    base: Base<Node2D>,
}

#[derive(Debug, GodotClass)]
#[class(init, base = Object)]
pub struct Msg {
    base: Base<Object>,
    #[init(val = "".into())]
    pub name: String,
    #[init(val = 0)]
    pub age: i32,
}

#[godot_api]
impl IObject for Msg {
    fn to_string(&self) -> GString {
        format!("Msg (name: {}, age: {})", self.name, self.age).to_gstring()
    }
}

#[godot_api]
impl Msg {
    pub fn create(name: String, age: i32) -> Gd<Self> {
        let mut msg = Msg::new_alloc();
        msg.bind_mut().name = name;
        msg.bind_mut().age = age;
        msg
    }
}

#[godot_api]
impl INode2D for TestBox {
    fn ready(&mut self) {
        let ins_id = self.base().instance_id();
        godot::task::spawn(async move {
            if let Ok(t_box) = Gd::<TestBox>::try_from_instance_id(ins_id) {
                t_box.get_tree().signals().process_frame().to_future().await;
                godot_print!("要等2秒");
                t_box
                    .get_tree()
                    .create_timer(2.0)
                    .signals()
                    .timeout()
                    .to_future()
                    .await;
                godot_print!("触发");
                t_box
                    .signals()
                    .change()
                    .emit(&Msg::create("王二小".to_owned(), 18));
            }
        });

        self.signals().change().connect_self(Self::on_changed);
    }
}

#[godot_api]
impl TestBox {
    #[signal]
    pub fn change(msg: Gd<Msg>);

    fn on_changed(&mut self, msg: Gd<Msg>) {
        godot_print!("异步而来的信号 {:?}", msg.to_string());
    }
}
