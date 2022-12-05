fn main() {
    let name = std::env::args().next().unwrap();
    println!("{}: {}", name, std::process::id());
}
