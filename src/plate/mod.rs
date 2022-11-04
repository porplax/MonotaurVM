use crate::SupportedTypes;

#[derive(Debug)]
pub struct GetValueOfType {
    pub int: u32,
    pub float: f32,
    pub str: String,
}

#[derive(Debug)]
pub struct PackedIngredients {
    pub istype: SupportedTypes,
    pub value: GetValueOfType,
}

// Make it easier. :)
pub type Food = PackedIngredients;

// The actual plate.
pub struct Plate {
    // The capacity of the plate, if exceeded, terminate.
    capacity: usize,
    // The actual plate.
    area: Vec<Food>,
}

impl Plate {
    // The function to create plates.
    pub fn new(max: usize) -> Plate {
        return Plate {
            capacity: max,
            area: vec![],
        };
    }

    // Plop Plop!
    pub fn plop(&mut self, food: Food) {
        if self.capacity == self.area.len() {
            panic!("\u{001b}[31mUser attempted to add to plate when capacity was met.\u{001b}[0m")
        }
        return self.area.push(food);
    }

    // Taken't?
    pub fn take(&mut self) -> Food {
        if self.area.len() == 0 {
            panic!("\u{001b}[31mUser attempted to take from an empty plate.\u{001b}[0m")
        }
        return self.area.pop().unwrap(); // Swapped to prevent E0308.
    }

    // No more!
    pub fn devour(&mut self) {
        self.area.clear();
        return;
    }

    // This function is really internal since it has no use in instructions.
    pub fn peek(&self) -> &Food { return self.area.last().unwrap(); }
    // This whole example is inspired by a few code on github and tutorials.
}
