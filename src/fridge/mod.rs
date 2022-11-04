/*
This file is basically the addition of locals into Monotaur. 

My first attempt should include the usage of tables,
we'll use the hashbrown crate for this. 

My idea is that tables allows for easy insertion,
and easy retrieval. 

*/

use hashbrown::HashMap;

use crate::*;

pub struct Fridge {
    capacity: usize,
    area: HashMap<u32, Delectable>,
}

impl Fridge {
    pub fn new(max: usize) -> Fridge {
        return Fridge {
            capacity: max,
            area: HashMap::new(),
        };
    }

    pub fn insert(&mut self, slot: u32, value: Delectable) {
        if self.capacity == self.area.len() {
            panic!("\u{001b}[31mUser attempted to add to fridge when fridge was full.\u{001b}[0m")
        }
        self.area.insert(slot, value);
        return;
    }

    pub fn retreive(&mut self, slot: &u32) -> &Delectable {
        return self.area.get_key_value(slot).unwrap().1;
    }
}