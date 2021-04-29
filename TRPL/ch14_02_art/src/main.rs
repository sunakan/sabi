extern crate ch14_02_art;

use crate::ch14_02_art::kinds::PrimaryColor;
use crate::ch14_02_art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
