mod helper;
mod owner_borrow;

use owner_borrow::owner_borrow::{owner, borrow};

fn main() {
    owner();
    borrow();
}
