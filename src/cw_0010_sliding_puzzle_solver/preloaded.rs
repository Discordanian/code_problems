use super::is_solvable;

pub fn assert_valid_solution(start: &[Vec<u8>], solution: Option<Vec<u8>>) {
    let n = start.len();
    assert!(n >= 3, "puzzle must be at least 3x3");
    assert!(start.iter().all(|row| row.len() == n), "puzzle must be square");

    match solution {
        None => {
            assert!(
                !is_solvable(start),
                "returned None, but this puzzle is solvable"
            );
        }
        Some(moves) => {
            assert!(
                is_solvable(start),
                "returned a solution for an unsolvable puzzle"
            );
            let mut board = start.to_vec();
            let (mut zr, mut zc) = find_zero(&board);
            for tile in moves {
                let (r, c) = find_tile(&board, tile);
                let dist = r.abs_diff(zr) + c.abs_diff(zc);
                assert_eq!(
                    dist, 1,
                    "tile {tile} at ({r},{c}) is not adjacent to empty at ({zr},{zc})"
                );
                board[zr][zc] = tile;
                board[r][c] = 0;
                zr = r;
                zc = c;
            }
            for r in 0..n {
                for c in 0..n {
                    let expected = if r == n - 1 && c == n - 1 {
                        0
                    } else {
                        (r * n + c + 1) as u8
                    };
                    assert_eq!(
                        board[r][c], expected,
                        "board is not solved after applying the move list"
                    );
                }
            }
        }
    }
}

fn find_zero(board: &[Vec<u8>]) -> (usize, usize) {
    find_tile(board, 0)
}

fn find_tile(board: &[Vec<u8>], tile: u8) -> (usize, usize) {
    for (r, row) in board.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == tile {
                return (r, c);
            }
        }
    }
    panic!("tile {tile} not found on the board");
}
