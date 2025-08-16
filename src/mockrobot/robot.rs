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

#[derive(Debug, Error, Copy, Clone)]
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
            State::RobotError(err) => Err(err),
            _ => Ok(self)
        }
    }
    pub fn pick(&mut self, _location: Location) -> Result<(), RobotError> {
        self.simulate();
        match self.state {
            State::RobotError(err) => Err(err),
            _ => Ok(())
        }
    }
    pub fn place(&mut self, _location: Location) -> Result<(), RobotError> {
        self.simulate();
        match self.state {
            State::RobotError(err) => Err(err),
            _ => Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_robot_creation() {
        let robot = Robot::new();
        assert!(!robot.id.is_empty());
        assert!(matches!(robot.state, State::Ready));
        assert_eq!(robot.location.0, 0);
        assert_eq!(robot.location.1, 0);
        assert_eq!(robot.location.2, 0);
        assert!(matches!(robot.action, Action::Idle));
    }

    #[test]
    fn test_robot_id_uniqueness() {
        let robot1 = Robot::new();
        let robot2 = Robot::new();
        assert_ne!(robot1.id, robot2.id);
    }

    #[test]
    fn test_location_creation() {
        let location = Location(10, 20, 30);
        assert_eq!(location.0, 10);
        assert_eq!(location.1, 20);
        assert_eq!(location.2, 30);
    }

    #[test]
    fn test_robot_home_success() {
        let mut robot = Robot::new();
        robot.action = Action::GoingHome;
        
        let mut success_found = false;
        for _ in 0..100 {
            let test_robot = Robot {
                id: robot.id.clone(),
                state: State::Ready,
                location: Location(0, 0, 0),
                action: Action::GoingHome,
            };
            
            if test_robot.home().is_ok() {
                success_found = true;
                break;
            }
        }
        assert!(success_found, "home() should succeed at least once in 100 attempts");
    }

    #[test]
    fn test_robot_home_error() {
        let mut robot = Robot::new();
        robot.action = Action::GoingHome;
        
        let mut error_found = false;
        for _ in 0..100 {
            let test_robot = Robot {
                id: robot.id.clone(),
                state: State::Ready,
                location: Location(0, 0, 0),
                action: Action::GoingHome,
            };
            
            if let Err(RobotError::UknownError) = test_robot.home() {
                error_found = true;
                break;
            }
        }
        assert!(error_found, "home() should fail with UknownError at least once in 100 attempts");
    }

    #[test]
    fn test_robot_pick_success() {
        let mut robot = Robot::new();
        robot.action = Action::Picking;
        
        let mut success_found = false;
        for _ in 0..100 {
            let mut test_robot = Robot {
                id: robot.id.clone(),
                state: State::Ready,
                location: Location(0, 0, 0),
                action: Action::Picking,
            };
            
            if test_robot.pick(Location(10, 20, 30)).is_ok() {
                success_found = true;
                break;
            }
        }
        assert!(success_found, "pick() should succeed at least once in 100 attempts");
    }

    #[test]
    fn test_robot_pick_error() {
        let mut robot = Robot::new();
        robot.action = Action::Picking;
        
        let mut error_found = false;
        for _ in 0..100 {
            let mut test_robot = Robot {
                id: robot.id.clone(),
                state: State::Ready,
                location: Location(0, 0, 0),
                action: Action::Picking,
            };
            
            if let Err(RobotError::PickError) = test_robot.pick(Location(10, 20, 30)) {
                error_found = true;
                break;
            }
        }
        assert!(error_found, "pick() should fail with PickError at least once in 100 attempts");
    }

    #[test]
    fn test_robot_place_success() {
        let mut robot = Robot::new();
        robot.action = Action::Placing;
        
        let mut success_found = false;
        for _ in 0..100 {
            let mut test_robot = Robot {
                id: robot.id.clone(),
                state: State::Ready,
                location: Location(0, 0, 0),
                action: Action::Placing,
            };
            
            if test_robot.place(Location(5, 10, 15)).is_ok() {
                success_found = true;
                break;
            }
        }
        assert!(success_found, "place() should succeed at least once in 100 attempts");
    }

    #[test]
    fn test_robot_place_error() {
        let mut robot = Robot::new();
        robot.action = Action::Placing;
        
        let mut error_found = false;
        for _ in 0..100 {
            let mut test_robot = Robot {
                id: robot.id.clone(),
                state: State::Ready,
                location: Location(0, 0, 0),
                action: Action::Placing,
            };
            
            if let Err(RobotError::PlaceError) = test_robot.place(Location(5, 10, 15)) {
                error_found = true;
                break;
            }
        }
        assert!(error_found, "place() should fail with PlaceError at least once in 100 attempts");
    }

    #[test]
    fn test_robot_error_display() {
        let pick_error = RobotError::PickError;
        let place_error = RobotError::PlaceError;
        let unknown_error = RobotError::UknownError;
        
        assert_eq!(pick_error.to_string(), "Unable to pick");
        assert_eq!(place_error.to_string(), "Unable to place");
        assert_eq!(unknown_error.to_string(), "Unknown error occured");
    }

    #[test]
    fn test_simulate_idle_action() {
        let mut robot = Robot::new();
        robot.action = Action::Idle;
        robot.simulate();
        assert!(matches!(robot.state, State::Ready));
    }

    #[test]
    #[should_panic]
    fn test_disconnect_not_implemented() {
        let mut robot = Robot::new();
        robot.disconnect().unwrap();
    }
}
