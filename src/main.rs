mod content;

use content::base::cc;
use content::composite::{self, string, tup, rstruct, rarray};
use content::martch;

use concat;
use crate::content::composite::rstruct::User;

fn main() {
    let i = martch::value_in_direction(martch::Direction::East);
    println!("{}", i);
    martch::action_test();
}
