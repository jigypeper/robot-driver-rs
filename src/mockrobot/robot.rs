use uuid::Uuid;

pub enum State {
    Ready,
    InProgress,
    Complete,
    Error,
}

pub struct Robot {
    pub id: String,
    pub state: State,
}

impl Robot {
    fn new() -> Self {
        Robot {
            id: Uuid::new_v4().to_string(),
            state: State::Ready,
        }
    }
}
