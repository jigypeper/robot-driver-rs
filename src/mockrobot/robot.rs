use rand::random;
use thiserror::Error;
use uuid::Uuid;

pub enum State {
    Ready,
    InProgress,
    Complete,
    RobotError(RobotError),
}

pub enum Action {
    Idle,
    Picking,
    Placing,
    GoingHome,
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
    pub action: Action,
}

pub struct Location(pub i32, pub i32, pub i32);

impl Robot {
    pub fn new() -> Self {
        Robot {
            id: Uuid::new_v4().to_string(),
            state: State::Ready,
            location: Location(0, 0, 0),
            action: Action::Idle,
        }
    }

    pub fn home(mut self) -> Result<Self, RobotError> {
        self.simulate();
        match self.state {
            State::RobotError(err) => Err(err)
            _ => Ok(self)
        }
    }
    pub fn pick(&mut self, location: Location) -> Result<Self, RobotError> {
        self.simulate();
        match self.state {
            State::RobotError(err) => Err(err)
            _ => Ok(self)
        }
    }
    pub fn place(&mut self, location: Location) -> Result<Self, RobotError> {
        self.simulate();
        match self.state {
            State::RobotError(err) => Err(err)
            _ => Ok(self)
        }
    }

    pub fn disconnect(&mut self) -> Result<(), RobotError> {
        // TODO: Either add a disconnect state or delete the robot?
        todo!();
    }

    fn simulate(&mut self) -> &mut Self {
        // TODO: need to think of how to simulate errors
        let random_number: u32 = random();
        match self.action {
            Action::Idle => self.state = State::Ready,
            Action::Picking => {
                if random_number % 5 == 0 {
                    self.state = State::Complete;
                } else {
                    self.state = State::RobotError(RobotError::PickError);
                }
            }
            Action::Placing => {
                if random_number % 3 == 0 {
                    self.state = State::Complete;
                } else {
                    self.state = State::RobotError(RobotError::PlaceError);
                }
            }
            Action::GoingHome => {
                if random_number % 2 == 0 {
                    self.state = State::Complete;
                } else {
                    self.state = State::RobotError(RobotError::UknownError);
                }
            }
        }
        self
    }
}
