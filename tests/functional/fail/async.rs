use functional_macro::pure_functional;

#[pure_functional]
async fn test() {}

fn main() {
    test();
}
