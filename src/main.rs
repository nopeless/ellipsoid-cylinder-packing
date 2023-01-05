use ellipsoid_cylinder_packing::{run, Config};
pub use std::env::args;

fn main() {
    println!("starting program");
    run(Config::from_args(
        args().skip(1).map(|s| s.parse().unwrap()).collect(),
    ));
    print!("exited");
}
