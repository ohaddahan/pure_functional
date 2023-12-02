use functional_macro::pure_functional;

#[pure_functional]
fn test() -> () {}

fn main() {
    test();
}
