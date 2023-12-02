use functional_macro::pure_functional;

const GLOBAL: u32 = 1;
#[pure_functional]
fn test(i1: u32) -> u32 {
    i1 + GLOBAL
}

fn main() {
    let i1 = 5;
    let result = test(i1);
    assert_eq!(result, i1 + GLOBAL);
}
