use functional_macro::pure_functional;

#[pure_functional]
fn test(i1: u32) -> u32 {
    i1 + 1
}

fn main() {
    let i1 = 1u32;
    assert_eq!(test(i1), 2u32);
}
