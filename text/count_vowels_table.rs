use std::os;

struct VowelCounter {
    num_a: int,
    num_e: int,
    num_i: int,
    num_o: int,
    num_u: int,
}

impl VowelCounter {
    fn new() -> VowelCounter {
        VowelCounter {
            num_a: 0,
            num_e: 0,
            num_i: 0,
            num_o: 0,
            num_u: 0,
        }
    }

    fn add(&mut self, c: char) {
        match c {
            'a' => { self.num_a += 1; }
            'e' => { self.num_e += 1; }
            'i' => { self.num_i += 1; }
            'o' => { self.num_o += 1; }
            'u' => { self.num_u += 1; }
            _   => { }
        }
    }

    fn total(&self) -> int {
        self.num_a + self.num_e + self.num_i + self.num_o + self.num_u
    }

    fn to_str(&self) -> ~str {
        format!("total:{}  a:{} e:{} i:{} o:{} u:{}",
            self.total(), self.num_a, self.num_e, self.num_i, self.num_o, self.num_u)
    }
}


fn count_vowels(input: &str) -> VowelCounter {
    let mut vc = VowelCounter::new();
    for c in input.iter() {
        vc.add(c);
    }
    vc
}


fn main() {
    let args = os::args();

    for arg in args.iter() {
        println(count_vowels(*arg).to_str());
    }
}
