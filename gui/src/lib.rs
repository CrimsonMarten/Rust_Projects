pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        // code to actually draw a TextField
    }
}