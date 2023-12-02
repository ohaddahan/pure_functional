use functional_macro::pure_functional;

struct Test {
    i1: u32,
}

impl Test {
    #[pure_functional]
    pub fn test(&self) -> u32 {
        self.i1 + 1
    }
}

fn main() {
    let t = Test { i1: 1 };
    assert_eq!(t.test(), 2);
}
