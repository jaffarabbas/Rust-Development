mod helper;
mod owner_borrow;
mod scoping;
mod dataTypes;
mod inputs;

use owner_borrow::owner_borrow::{owner, borrow};
use scoping::scoping;
use dataTypes::dataTypes;
use inputs::inputs;

fn main() {
    // owner();
    // borrow();
    // scoping();
    // dataTypes();
    inputs();
}
