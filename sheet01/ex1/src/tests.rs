
#[cfg(test)]
mod tests {
    use crate::State;

    #[test]
    fn black_king_boarder() {
        let state = State::from("black c4 c8 h3");
        let substates = vec![
            State::from("white c4 c8 g2"),
            State::from("white c4 c8 g3"),
            State::from("white c4 c8 g4"),
            State::from("white c4 c8 h4"),
            State::from("white c4 c8 h2"),
        ];
        for substate in state.substates() {
            assert!(substates.contains(&substate));
        }
        assert_eq!(state.substates().count(), substates.len());
    }

    #[test]
    fn black_king_corner() {
        let state = State::from("black c4 c8 a1");
        let substates = vec![
            State::from("white c4 c8 a2"),
            State::from("white c4 c8 b1"),
            State::from("white c4 c8 b2"),
        ];
        for substate in state.substates() {
            assert!(substates.contains(&substate));
        }
        assert_eq!(state.substates().count(), substates.len());
    }

    #[test]
    fn black_king_unobstructed() {
        let state = State::from("black a1 b2 f4");
        let substates = vec![
            State::from("white a1 b2 e3"),
            State::from("white a1 b2 e4"),
            State::from("white a1 b2 e5"),
            State::from("white a1 b2 f3"),
            State::from("white a1 b2 f5"),
            State::from("white a1 b2 g3"),
            State::from("white a1 b2 g4"),
            State::from("white a1 b2 g5"),
        ];
        for substate in state.substates() {
            assert!(substates.contains(&substate));
        }
        assert_eq!(state.substates().count(), substates.len());
    }

    #[test]
    fn rook_unobstructed() {
        let state = State::from("white a1 e4 h8");
        let rook_substates = vec![
            // horizontal
            State::from("black a1 a4 h8"),
            State::from("black a1 b4 h8"),
            State::from("black a1 c4 h8"),
            State::from("black a1 d4 h8"),
            State::from("black a1 f4 h8"),
            State::from("black a1 g4 h8"),
            State::from("black a1 h4 h8"),
            // vertical
            State::from("black a1 e1 h8"),
            State::from("black a1 e2 h8"),
            State::from("black a1 e3 h8"),
            State::from("black a1 e5 h8"),
            State::from("black a1 e6 h8"),
            State::from("black a1 e7 h8"),
            State::from("black a1 e8 h8"),
        ];
        for state in state.rook_moves() {
            assert!(rook_substates.contains(&state));
        }
        assert_eq!(state.rook_moves().count(), rook_substates.len());
    }

    #[test]
    fn rook_obstructed1() {
        let state = State::from("white c4 c8 h4");
        let rook_substates = vec![
            // horizontal
            State::from("black c4 a8 h4"),
            State::from("black c4 b8 h4"),
            State::from("black c4 d8 h4"),
            State::from("black c4 e8 h4"),
            State::from("black c4 f8 h4"),
            State::from("black c4 g8 h4"),
            State::from("black c4 h8 h4"),
            // vertical
            State::from("black c4 c5 h4"),
            State::from("black c4 c6 h4"),
            State::from("black c4 c7 h4"),
        ];
        for state in state.rook_moves() {
            assert!(rook_substates.contains(&state));
        }
        assert_eq!(state.rook_moves().count(), rook_substates.len());
    }
}