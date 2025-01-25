use uuid::Uuid;

pub enum State {
    Ready,
    InProgress,
    Complete,
    Error, // need an error enum here
}

pub struct Robot {
    pub id: String,
    pub state: State,
    pub location: Location,
}

pub struct Location(i32, i32, i32);

impl Robot {
    pub fn new() -> Self {
        Robot {
            id: Uuid::new_v4().to_string(),
            state: State::Ready,
            location: Location(0, 0, 0),
        }
    }

    pub fn home(&mut self) -> Self {
        todo!();
    }
    pub fn pick(&mut self, location: Location) -> Self {
        todo!();
    }
    pub fn place(&mut self, location: Location) -> Self {
        todo!();
    }
}
