extern {
    fn yajue()->i32;
}
fn main() {
    println!("Hello, world {}!", unsafe{yajue()});
}
