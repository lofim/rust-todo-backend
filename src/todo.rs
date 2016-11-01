use uuid::Uuid;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum State {
    Open,
    Done
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn update(&mut self, partial: TodoPartial) {
        self.text = partial.text;
    }
}

#[derive(RustcDecodable)]
pub struct TodoPartial {
    pub text: String,
}
