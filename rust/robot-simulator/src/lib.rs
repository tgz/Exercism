// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone , Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    x: i32,
    y: i32,
    direction: Direction
}

impl Direction {
    fn get_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North
        }
    }

    fn get_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South
        }
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Robot { direction: self.direction.get_right(), ..self }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot { direction: self.direction.get_left(), ..self}
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::East => Robot { x: self.x + 1, ..self },
            Direction::North => Robot { y: self.y + 1, ..self },
            Direction::West => Robot { x: self.x - 1, ..self },
            Direction::South => Robot { y: self.y - 1, ..self},
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, char| {
            match char {
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                'R' => robot.turn_right(),
                _ => robot
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
