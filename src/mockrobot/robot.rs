use uuid::Uuid;
use thiserror::Error

pub enum State {
    Ready,
    InProgress,
    Complete,
    RobotError(RobotError),
}

#[derive(Debug, Error)]
pub enum RobotError {
    #[error("Unable to pick")]
    PickError,
    #[error("Unable to place")]
    PlaceError,
    #[error("Unknown error occured")]
    UknownError,
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

    pub fn simulate(&mut self) -> Self {
        todo!();
    }
}
