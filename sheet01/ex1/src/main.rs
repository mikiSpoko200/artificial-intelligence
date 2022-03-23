mod tests;

use std::collections::{ HashSet, VecDeque };
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::ops::RangeInclusive;

const KING_MOVES: [(Pos, Pos); 8] = [
    (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)
];

const MIN_BOARD_POS: Pos = 0;
const MAX_BOARD_POS: Pos = 7;
const BOARD_BOUNDS: RangeInclusive<Pos> = MIN_BOARD_POS..=MAX_BOARD_POS;


/// Enumeration of pawn colors.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Color {
    Black,
    White,
}

impl Color {
    const fn flip(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Color::Black => "black",
            Color::White => "white",
        };
        write!(f, "{}", color)
    }
}

impl From<&str> for Color {
    fn from(color: &str) -> Self {
        match color {
            "black" => Self::Black,
            "white" => Self::White,
            _ => panic!("Invalid starting color encoding: {}", color),
        }
    }
}

#[derive(Debug)]
enum TraversalResult {
    Inf,
    CheckMate(Vec<State>)
}

type Pos = i8;

/// Representation of a game state.
/// Positions are stored in compressed representation where each pawn has one Pos field that
/// in first 3 bits contains binary representation of tile row number and next 3 bits (3-5) contain
/// similarly encoded tile column number.
/// Fields are numbered from 0-7 increasing just as in typical chess board that is:
/// from left bottom corner upwards and rightwards.
#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    black_king_pos: Pos,
    white_king_pos: Pos,
    white_rook_pos: Pos,
    turn: Color,
}

impl State {
    const POSITION_ENCODING_OFFSET: u8 = 3;
    const ROW_BITMASK: Pos = (1 << State::POSITION_ENCODING_OFFSET) - 1;
    const COL_BITMASK: Pos = State::ROW_BITMASK << State::POSITION_ENCODING_OFFSET;

    fn position_to_string(position: Pos) -> String {
        let (col, row) = State::decode_position(position);
        String::from_utf8(vec![col as u8 + b'a', row as u8 + b'1']).unwrap()
    }

    pub fn from_file(path: &str) -> Self {
        let state = String::from(
            fs::read_to_string(path).expect(format!("Cannot read data from: {}", path).as_str())
        );
        State::from(state.as_str())
    }

    pub const fn new(white_king_pos: Pos, white_rook_pos: Pos, black_king_pos: Pos, turn: Color) -> Self {
        Self {black_king_pos, white_king_pos, white_rook_pos, turn}
    }

    /// Gets pawn's position from compressed representation.
    const fn decode_position(position: Pos) -> (Pos, Pos) {
        (position >> State::POSITION_ENCODING_OFFSET, position & (1 << State::POSITION_ENCODING_OFFSET) - 1)
    }

    /// Encodes the pawn's position.
    const fn encode_position(col: Pos, row: Pos) -> Pos {
        col << State::POSITION_ENCODING_OFFSET & State::COL_BITMASK | row & State::ROW_BITMASK
    }

    /// Checks if given pawn position is within board bounds.
    fn is_position_within_bounds(&self, col: Pos, row: Pos) -> bool {
        BOARD_BOUNDS.contains(&col) && BOARD_BOUNDS.contains(&row)
    }

    /// Checks if current current state is a checkmate.
    pub fn is_checkmate(&self, possible_moves: usize) -> bool {
        let (black_king_col, black_king_row) = State::decode_position(self.black_king_pos);
        let (white_king_col, white_king_row) = State::decode_position(self.white_king_pos);
        let (rook_col, rook_row) = State::decode_position(self.white_rook_pos);
        let king_check = (black_king_col - white_king_col).abs() < 2 && (black_king_row - white_king_row).abs() < 2;
        let rook_check = black_king_col == rook_col || black_king_row == rook_row;
        (king_check || rook_check) && possible_moves == 0
    }

    fn rook_moves(&self) -> impl Iterator<Item=State> + '_ {
        // rook can move to any tile that is not physically occupied by some other pawn.
        // even if the rook is places next to the black king it will not create any issues
        // because of how black king moves are defined.

        let (rook_col, rook_row) = State::decode_position(self.white_rook_pos);
        let (white_king_col, white_king_row) = State::decode_position(self.white_king_pos);
        let (black_king_col, black_king_row) = State::decode_position(self.black_king_pos);

        let mut left_range = MIN_BOARD_POS;
        let mut right_range = MAX_BOARD_POS;
        let mut bottom_range = MIN_BOARD_POS;
        let mut top_range = MAX_BOARD_POS;

        use std::cmp::Ordering;

        for (other_col, other_row) in [
            (black_king_col, black_king_row), (white_king_col, white_king_row)]
        {
            if other_row == rook_row {
                match rook_col.cmp(&other_col) {
                    Ordering::Less => { right_range = right_range.min(other_col - 1); }
                    Ordering::Equal => { panic!("Two pawns share the same tile."); }
                    Ordering::Greater => { left_range = left_range.max(other_col + 1); }
                }
            }
            if other_col == rook_col {
                match rook_row.cmp(&other_row) {
                    Ordering::Less => { top_range = top_range.min(other_row - 1); }
                    Ordering::Equal => { panic!("Two pawns share the same tile."); }
                    Ordering::Greater => { bottom_range = bottom_range.max(other_row + 1); }
                }
            }
        }
        let horizontal_rook_moves = (left_range..=right_range).filter_map(move |col|{
            if col == rook_col || (col - black_king_col).abs() < 2 && (rook_row - black_king_row).abs() < 2 {
                None
            } else {
                Some(State::new(
                    self.white_king_pos,
                    State::encode_position(col, rook_row),
                    self.black_king_pos,
                    self.turn.flip()
                ))
            }
        });
        let vertical_rook_moves = (bottom_range..=top_range).filter_map(move |row|{
            if row == rook_row || (rook_col - black_king_col).abs() < 2 && (row - black_king_row).abs() < 2 {
                None
            } else {
                Some(State::new(
                    self.white_king_pos,
                    State::encode_position(rook_col, row),
                    self.black_king_pos,
                    self.turn.flip()
                ))
            }
        });
        horizontal_rook_moves.chain(vertical_rook_moves)
    }

    /// Produces all valid substates of the current state.
    pub fn substates(&self) -> Vec<State> {
        // Common fail conditions:
        // - Any pawn tries to move into a tile that is already occupied.
        // Rook:
        // - cannot move "over" any pawn. That is if any other pawn has the same col or row
        //   position as the rook, rook can only move up to the tile that is occupied.
        // White king:
        // - both column and row positions must differ by at least 2 so anything below that
        //   should be considered incorrect. This check also includes physical conflict scenario.
        // Black king:
        // - similarly as with white king it must not be in two tile vicinity with regard to

        match self.turn {
            Color::Black => {
                let (col, row) = State::decode_position(self.black_king_pos);
                let substates = KING_MOVES.into_iter().filter_map(move |(mut new_col, mut new_row)| {
                    // move pawn to new position
                    new_col += col;
                    new_row += row;
                    // check if position is valid:
                    // 1. within board
                    if !self.is_position_within_bounds(new_col, new_row) {
                        return None;
                    } else {
                        // 2. not in position guarded by other pawn.
                        let (rook_col, rook_row) = State::decode_position(self.white_rook_pos);
                        let (white_king_col, white_king_row) = State::decode_position(self.white_king_pos);

                        // check if king blocks the currently considered tile.
                        if (new_col - white_king_col).abs() < 2 && (new_row - white_king_row).abs() < 2 {
                            return None;
                        }
                        // check if rook blocks the currently considered tile.
                        if new_col == rook_col || new_row == rook_row {
                            return None;
                        }
                        Some(State::new(
                                self.white_king_pos,
                                self.white_rook_pos,
                                State::encode_position(new_col, new_row),
                                self.turn.flip()
                        ))
                    }
                });
                substates.collect()
            }
            Color::White => {
                let (rook_col, rook_row) = State::decode_position(self.white_rook_pos);
                let (white_king_col, white_king_row) = State::decode_position(self.white_king_pos);
                let (black_king_col, black_king_row) = State::decode_position(self.black_king_pos);

                // possible king moves
                let king_moves = KING_MOVES.iter().filter_map(move |(mut new_col, mut new_row)| {
                    new_col += white_king_col;
                    new_row += white_king_row;
                    if !self.is_position_within_bounds(new_col, new_row) {
                        return None;
                    } else {
                        if (new_col - black_king_col).abs() < 2 && (new_row - black_king_row).abs() < 2 {
                            return None;
                        }
                        if new_col == rook_col && new_row == rook_row {
                            return None;
                        }
                        Some(State::new(
                                State::encode_position(new_col, new_row),
                                self.white_rook_pos,
                                self.black_king_pos,
                                self.turn.flip()
                        ))
                    }
                });
                let rook_moves = self.rook_moves();
                let substates = king_moves.chain(rook_moves);
                substates.collect()
            }
        }
    }

    pub fn solve(self) -> TraversalResult {
        let mut memo = HashSet::<State>::with_capacity(64 * 63 * 62 * 2);
        let path = Vec::new();
        let mut lifo = VecDeque::<(State, Vec<State>)>::from([(self, path)]);
        while let Some((state, mut path)) = lifo.pop_front() {
            path.push(state.clone());
            let substates = state.substates();
            if state.is_checkmate(substates.len()) {
                return TraversalResult::CheckMate(path);
            } else {
                for substate in state.substates() {
                    if !memo.contains(&substate) {
                        memo.insert(substate.clone());
                        lifo.push_back((substate, path.clone()));
                    }
                }
            }
        }
        TraversalResult::Inf
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let white_king_position = State::decode_position(self.white_king_pos);
        let rook_position = State::decode_position(self.white_rook_pos);
        let black_king_position = State::decode_position(self.black_king_pos);
        f.debug_struct("State")
            .field("white king position", &white_king_position)
            .field("rook position", &rook_position)
            .field("black king position", &black_king_position)
            .field("turn", &self.turn)
            .finish()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let wk_pos = State::position_to_string(self.white_king_pos);
        let r_pos = State::position_to_string(self.white_rook_pos);
        let bk_pos = State::position_to_string(self.black_king_pos);
        write!(f, "{} {} {} {}", self.turn, wk_pos, r_pos, bk_pos)
    }
}

impl From<&str> for State {
    fn from(initial_state: &str) -> Self {
        let tokens = initial_state.split_whitespace().take(4).collect::<Vec<&str>>();
        match tokens[..] {
            [color, ref str_positions @ ..] => {
                let converted_positions = str_positions
                    .iter()
                    .map(|&pos| {
                        match pos.to_ascii_lowercase().as_bytes() {
                            [col, row] if (b'a'..=b'h').contains(col) && (b'1'..=b'8').contains(row) => {
                                 State::encode_position((col - b'a') as Pos, (row - b'1') as Pos)
                            }
                            _ => panic!("Invalid position encoding: {}", pos)
                        }
                    }).collect::<Vec<Pos>>();
                if let &[white_king, white_rook, black_king] = &converted_positions[..] {
                    State::new(
                        white_king,
                        white_rook,
                        black_king,
                        Color::from(color)
                    )
                } else {
                    panic!("Incorrect position encodings. Expected 3 positions fround {:?} in {}", converted_positions, converted_positions.len())
                }
            }
            _ => panic!("Incorrect state encoding. Expected 4 tokens fround {:?} in {}", tokens, tokens.len())
        }

    }
}

fn main() {
    let state = State::from_file("zad1_input.txt");
    let outcome = match state.solve() {
        TraversalResult::Inf => { String::from("INF") }
        TraversalResult::CheckMate(path) => { (path.len() - 1).to_string() }
    };
    fs::write("zad1_output.txt", outcome.as_bytes()).expect(format!("Could not write {} to zad1_output.txt", outcome).as_str());
}
