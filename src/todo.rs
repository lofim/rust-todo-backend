use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub enum State {
    Open,
    Done
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Deserialize)]
pub struct TodoPartial {
    pub text: String,
}
