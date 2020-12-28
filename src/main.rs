extern crate rand;

use crate::rand::Rng;

fn main() {
    println!("Hello, world!");
    println!("hi");

    let mut rand = rand::thread_rng();

    let random_number = rand.gen_range(0..10);

    println!("{}", random_number);

}
// joe mama balls