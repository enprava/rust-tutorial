pub struct Carrot {
    pub color: String,
    pub length: f32,
}

impl Carrot {
    pub fn new() -> Self {
        Carrot {
            color: String::from("orange"),
            length: 10.0,
        }
    }

    pub fn describe(&self) -> String {
        format!("A {} carrot that is {}cm long", self.color, self.length)
    }
}
