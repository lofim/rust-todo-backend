use iron::typemap::Key;
use todo::{TodoItem};

pub struct TodoItems {}

impl TodoItems {
    pub fn new() -> Vec<TodoItem> {
        Vec::new()
    }
}

// Trait for having items stored in a cross handler State
impl Key for TodoItems {
    type Value = Vec<TodoItem>;
}
