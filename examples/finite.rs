extern crate feat;

use feat::{Finite, Natural};

fn main() {
    let three = Natural(3);
    let five = Natural(5);

    for i in (&three).product(&five).iter().enumerate() {
        println!("{:?}", i);
    }

    for s in (&five).map(|i| i.to_string()).iter().enumerate() {
        println!("{:?}", s);
    }

    for u in five.union(three).iter().enumerate() {
        println!("{:?}", u);
    }
}
