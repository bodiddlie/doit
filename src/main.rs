mod doit;
mod doit_node;

fn main() {
    match doit::run() {
        Ok(_) => println!("Exiting and spawning command..."),
        Err(msg) => eprintln!("{msg}"),
    }
}
