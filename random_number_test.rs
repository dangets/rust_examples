use std::rand;
use std::num::{min, max};


struct MinComputer {
    val: int,
}

impl MinComputer {
    fn new() -> MinComputer {
        MinComputer { val: std::int::max_value }
    }

    fn add(&mut self, other: int) {
        self.val = min(self.val, other);
    }

    fn get_value(&self) -> int {
        self.val
    }
}


struct MaxComputer {
    val: int,
}

impl MaxComputer {
    fn new() -> MaxComputer {
        MaxComputer { val: std::int::min_value }
    }

    fn add(&mut self, other: int) {
        self.val = max(self.val, other);
    }

    fn get_value(&self) -> int {
        self.val
    }
}



fn main() {
    let mut min_comp = MinComputer::new();
    let mut max_comp = MaxComputer::new();

    do 10.times {
        let r = (rand::random::<int>() % 100).abs();
        min_comp.add(r);
        max_comp.add(r);
    }

    println!("min: {}", min_comp.get_value());
    println!("max: {}", max_comp.get_value());
}
