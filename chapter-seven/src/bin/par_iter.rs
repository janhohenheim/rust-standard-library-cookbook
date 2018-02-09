extern crate rayon;
use rayon::prelude::*;
fn main() {
    let values = [
        "Did",
        "you",
        "ever",
        "hear",
        "the",
        "tragedy",
        "of",
        "Darth",
        "Plagueis",
        "the",
        "wise",
        "?",
    ];
    values.par_iter().for_each(|val| println!("{}", val));
}
