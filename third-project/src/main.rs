fn main() {
    println!("Hello, world!");
}

fn div(x: i32, y: i32) -> i32 {
    x / y * 2
}

#[test]
fn div_test() {
    assert_eq!(div(10, 3), 6);
}

#[test]
#[should_panic]
fn div_panic_test() {
    div(2, 0);
}
