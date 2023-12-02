use functional_macro::pure_functional;

#[pure_functional]
fn test(i1: &mut String) {
    *i1 = "world".to_string();
}

fn main() {
    let mut i1 = "hello".to_string();
    test(&mut i1);
}
