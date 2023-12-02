use functional_macro::pure_functional;

#[pure_functional]
fn test(i1: u32) {
    let _ = i1 + 1;
}

fn main() {
    let i1 = 1;
    assert_eq!(test(i1), ());
}
