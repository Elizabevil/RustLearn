use content::base::cc;
use content::composite::{self, rarray, rstruct, string, tup};
use content::martch;

use crate::content::composite::rstruct::User;

mod content;

use concat;

fn main() {
    let i = martch::value_in_direction(martch::Direction::East);
    println!("{}", i);
    martch::full::asds();
}
