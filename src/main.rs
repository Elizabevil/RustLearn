use content::base::cc;
use content::composite::{self, rarray, rstruct, string, tup};
use content::martch;


mod content;

use concat;
use crate::content::composite::rstruct::User;

fn main() {
    let i = martch::value_in_direction(martch::Direction::East);
    println!("{}", i);
    martch::full::asds();
}
