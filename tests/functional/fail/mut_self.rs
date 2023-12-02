use functional_macro::pure_functional;

struct Test {
    i1: u32,
}

impl Test {
    #[pure_functional]
    pub fn test(&mut self) -> u32 {
        self.i1 + 1
    }
}

fn main() {
    let mut t = Test { i1: 1 };
    t.test();
}
