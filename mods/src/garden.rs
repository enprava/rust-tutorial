pub mod vegetables;

pub fn plant_vegetable() -> String {
    String::from("Planting in garden!")
}

pub struct Garden {
    pub size: u32,
    location: String, // private field
}

impl Garden {
    pub fn new(size: u32, location: String) -> Self {
        Garden { size, location }
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }
}
