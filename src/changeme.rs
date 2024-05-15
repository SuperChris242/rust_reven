#[derive(Debug)]
pub struct MyCallbackData;

pub struct MyCallback {
    pub callback: Box<dyn Fn(&MyCallbackData)>,
}

pub trait MyTrait {
    fn set_callback(&mut self, cb: MyCallback);
    fn do_something(&self);
}

pub struct MyStruct {
    callbacks: Vec<MyCallback>,
}

impl MyStruct {
    pub fn new() -> Self {
        MyStruct {
            callbacks: Vec::new(),
        }
    }
}

impl MyTrait for MyStruct {
    fn set_callback(&mut self, cb: MyCallback) {
        self.callbacks.push(cb);
    }

    fn do_something(&self) {
        for cb in &self.callbacks {
            let cb_data = MyCallbackData;
            (cb.callback)(&cb_data);
        }
    }
}
