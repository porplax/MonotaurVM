/*
This file is basically the addition of globals into Monotaur.

It's basically fridge but it's not independent of each VM instance, it's across all VM instances.
*/

use hashbrown::HashMap;

use crate::*;

pub struct Cabinet {
    capacity: usize,
    area: HashMap<u32, Delectable>,
}

impl Cabinet {
    pub fn new(max: usize) -> Cabinet {
        return Cabinet {
            capacity: max,
            area: HashMap::new(),
        };
    }

    pub fn insert(&mut self, slot: u32, value: Delectable) {
        if self.capacity == self.area.len() {
            panic!("\u{001b}[31mUser attempted to add to cabinet when cabinet was full.\u{001b}[0m")
        }
        self.area.insert(slot, value);
        return;
    }

    pub fn retreive(&mut self, slot: &u32) -> &Delectable {
        return self.area.get_key_value(slot).unwrap().1;
    }
}