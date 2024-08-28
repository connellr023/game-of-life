#[link(name = "lib")]
extern "C" {
    fn say_hello();
}

fn main() {
    println!("Below is from C++");

    unsafe {
        say_hello();
    }
}
