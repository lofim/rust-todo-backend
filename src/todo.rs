use uuid::Uuid;

pub enum State {
    Open,
    Closed
}

pub struct TodoItem {
    text: String,
    id: String,
    state: State
}

impl TodoItem {
    pub fn new(text: String) -> TodoItem {
        TodoItem {
            text: text,
            id: Uuid::new_v4().to_string(),
            state: State::Open
        }
    }
}
