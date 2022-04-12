use std::char::decode_utf16;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Map;
use std::mem::discriminant;

fn main() {
    println!("Hello, world!");
}

// region Direction

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn pick_random() -> Direction {
        todo!("Implement choice randomization.")
    }

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Up    => 'U',
            Direction::Down  => 'D',
            Direction::Left  => 'L',
            Direction::Right => 'R',
        })
    }
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "D" => Ok(Direction::Down),
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _   => Err(())
        }
    }
}

// endregion


// region Position

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn step(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}


// endregion


// region Labyrinth

struct Labyrinth {
    buffer: Vec<Vec<u8>>,
    starting_positions: HashSet<Position>,
    finishing_positions: HashSet<Position>,
}

type MovedPositions = Map<Position, fn(Position) -> Position>;

impl Labyrinth {
    const MAX_STARTING_MOVE_COUNT: usize = 107;
    const MAX_STARTING_POSITION_COUNT: usize = 2;

    pub fn new(buffer: Vec<Vec<u8>>) -> Self {
        assert!(buffer.len() > 0, "Buffer must not be empty.");
        let max_capacity = buffer.len() * buffer[0].len();
        let mut starting_positions = HashSet::with_capacity(max_capacity);
        let mut finishing_positions = HashSet::with_capacity(max_capacity);
        for (y, row) in buffer.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    b'G' => { finishing_positions.insert(Position::new(x, y)); }
                    b'S' => { starting_positions.insert(Position::new(x, y)); }
                    b'B' => {
                        finishing_positions.insert(Position::new(x, y));
                        starting_positions.insert(Position::new(x, y));
                    }
                    _ => continue
                };
            }
        }
        Self { buffer, starting_positions, finishing_positions }
    }

    #[inline(always)]
    pub fn is_position_valid(&self, position: &Position) -> bool {
        self.buffer[position.y][position.x] != b'#'
    }

    #[inline(always)]
    pub fn is_position_final(&self, position: Position) -> bool {
        self.finishing_positions.contains(&position)
    }

    #[inline(always)]
    pub fn move_all_positions<'a, I: 'a, J: 'a>(&'a self, positions: I, direction: &'a Direction) -> MovedPositions
        where
            I: Iterator<Item=Position>, J: Iterator<Item=&'a Position> + 'a
    {
        positions.map(move |position| {
            let new_position = position.step(&direction);
            if self.is_position_valid(&new_position) { new_position } else { position }
        })
    }

    pub fn reduce_starting_positions(&self) -> Vec<Direction> {
        loop {
            let mut prev_direction: Option<Direction> = None;
            let mut starting_positions = self.starting_positions.clone();
            let mut moves = Vec::<Direction>::with_capacity(150);
            for _ in 0..Labyrinth::MAX_STARTING_MOVE_COUNT {
                if starting_positions.len() > Labyrinth::MAX_STARTING_POSITION_COUNT {
                    let mut direction = Direction::pick_random();
                    if let Some(prev_direction_val) = prev_direction {
                        while prev_direction_val == direction.reverse() {
                            direction = direction.reverse();
                        }
                        starting_positions.clear();
                        for elem in self.move_all_positions(starting_positions.into_iter(), &direction) {
                            starting_positions.insert(elem);
                        }
                        prev_direction = Some(direction);
                        moves.push(direction);
                    } else {
                        return moves;
                    }
                }
            }
        }
    }
}

impl From<&str> for Labyrinth {
    fn from(data: &str) -> Self {
        let buffer = data.lines().map(|line| {
            Vec::from(line)
        }).collect();
        Self::new(buffer)
    }
}
// endregion