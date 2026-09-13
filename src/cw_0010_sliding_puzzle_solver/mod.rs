/*
A sliding puzzle is a combination puzzle that challenges a player to slide (frequently flat) pieces along certain routes (usually on a board) to establish a certain end-configuration.

Your goal for this kata is to write a function that produces a sequence of tile movements that solves the puzzle.

Input

An n x n array/list comprised of integer values ranging from 0 to n^2 - 1 (inclusive), which represents a square grid of tiles. Note that there will always be one empty tile (represented by 0) to allow for movement of adjacent tiles.

Output

An array/list comprised of any (but not necessarily all) of the integers from 1 to n^2 - 1, inclusive. This represents the sequence of tile moves for a successful transition from the initial unsolved state to the solved state. If the puzzle is unsolvable, return null(JavaScript, Java, PHP) or None(Python) or the vector {0} (C++).

Test Example

let simple_example = &[
    vec![ 1, 2, 3, 4],
    vec![ 5, 0, 6, 8],
    vec![ 9,10, 7,11],
    vec![13,14,15,12]
];
slide_puzzle(simple_example);  // Some([6, 7, 11, 12])

// TRANSITION SEQUENCE:
[ 1, 2, 3, 4]    [ 1, 2, 3, 4]    [ 1, 2, 3, 4]    [ 1, 2, 3, 4]    [ 1, 2, 3, 4]
[ 5, 0, 6, 8]    [ 5, 6, 0, 8]    [ 5, 6, 7, 8]    [ 5, 6, 7, 8]    [ 5, 6, 7, 8]
[ 9,10, 7,11] -> [ 9,10, 7,11] -> [ 9,10, 0,11] -> [ 9,10,11, 0] -> [ 9,10,11,12]
[13,14,15,12]    [13,14,15,12]    [13,14,15,12]    [13,14,15,12]    [13,14,15, 0]

// NOTE: Your solution does not need to follow this exact sequence to pass
Technical Details

Input will always be valid.
The range of values for n are: 10 >= n >= 3
*/
pub fn slide_puzzle(arr: &[Vec<u8>]) -> Option<Vec<u8>> {
    if !is_solvable(arr) {
        return None;
    }
    let mut puzzle = Puzzle::new(arr);
    puzzle.solve();
    Some(puzzle.moves)
}

fn is_solvable(board: &[Vec<u8>]) -> bool {
    let n = board.len();
    let mut tiles = Vec::with_capacity(n * n - 1);
    let mut blank_row = 0;
    for (r, row) in board.iter().enumerate() {
        for &v in row {
            if v == 0 {
                blank_row = r;
            } else {
                tiles.push(v);
            }
        }
    }
    let mut inversions = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            if tiles[i] > tiles[j] {
                inversions += 1;
            }
        }
    }
    if n % 2 == 1 {
        inversions % 2 == 0
    } else {
        (inversions + blank_row) % 2 == 1
    }
}

struct Puzzle {
    n: usize,
    board: Vec<Vec<u8>>,
    locked: Vec<Vec<bool>>,
    zr: usize,
    zc: usize,
    moves: Vec<u8>,
}

impl Puzzle {
    fn new(arr: &[Vec<u8>]) -> Self {
        let n = arr.len();
        let board = arr.to_vec();
        let mut zr = 0;
        let mut zc = 0;
        for (r, row) in board.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                if v == 0 {
                    zr = r;
                    zc = c;
                }
            }
        }
        Self {
            n,
            board,
            locked: vec![vec![false; n]; n],
            zr,
            zc,
            moves: Vec::new(),
        }
    }

    fn goal(n: usize, r: usize, c: usize) -> u8 {
        if r + 1 == n && c + 1 == n {
            0
        } else {
            (r * n + c + 1) as u8
        }
    }

    fn solve(&mut self) {
        for r in 0..self.n - 2 {
            self.solve_row(r);
        }
        for c in 0..self.n - 2 {
            self.solve_column(c);
        }
        self.solve_2x2();
    }

    fn solve_row(&mut self, r: usize) {
        for c in 0..self.n - 2 {
            let val = Self::goal(self.n, r, c);
            self.move_tile_to(val, r, c);
            self.locked[r][c] = true;
        }
        let a = Self::goal(self.n, r, self.n - 2);
        let b = Self::goal(self.n, r, self.n - 1);
        if self.board[r][self.n - 2] == a && self.board[r][self.n - 1] == b {
            self.locked[r][self.n - 2] = true;
            self.locked[r][self.n - 1] = true;
            return;
        }
        self.arrange(a, r, self.n - 1, b, r + 1, self.n - 1, r, self.n - 2);
        self.slide(r, self.n - 1);
        self.slide(r + 1, self.n - 1);
        self.locked[r][self.n - 2] = true;
        self.locked[r][self.n - 1] = true;
    }

    fn solve_column(&mut self, c: usize) {
        let top = self.n - 2;
        let bot = self.n - 1;
        let a = Self::goal(self.n, top, c);
        let b = Self::goal(self.n, bot, c);
        if self.board[top][c] == a && self.board[bot][c] == b {
            self.locked[top][c] = true;
            self.locked[bot][c] = true;
            return;
        }
        self.arrange(a, bot, c, b, bot, c + 1, top, c);
        self.slide(bot, c);
        self.slide(bot, c + 1);
        self.locked[top][c] = true;
        self.locked[bot][c] = true;
    }

    fn solve_2x2(&mut self) {
        if self.is_solved() {
            return;
        }
        let n = self.n;
        let cells = [(n - 2, n - 2), (n - 2, n - 1), (n - 1, n - 2), (n - 1, n - 1)];
        let start = self.pack_2x2();
        let goal = {
            let a = Self::goal(n, n - 2, n - 2) as u32;
            let b = Self::goal(n, n - 2, n - 1) as u32;
            let c = Self::goal(n, n - 1, n - 2) as u32;
            a << 24 | b << 16 | c << 8
        };

        let mut queue = std::collections::VecDeque::from([start]);
        let mut prev: std::collections::HashMap<u32, (u32, u8)> = std::collections::HashMap::new();
        prev.insert(start, (start, 0));

        while let Some(state) = queue.pop_front() {
            if state == goal {
                let mut tiles = Vec::new();
                let mut cur = state;
                while cur != start {
                    let (p, tile) = prev[&cur];
                    tiles.push(tile);
                    cur = p;
                }
                tiles.reverse();
                for tile in tiles {
                    let (r, c) = self.pos(tile);
                    self.slide(r, c);
                }
                return;
            }

            let vals = unpack(state);
            let zi = vals.iter().position(|&v| v == 0).unwrap();
            let (zr, zc) = cells[zi];
            for (i, &(r, c)) in cells.iter().enumerate() {
                if r.abs_diff(zr) + c.abs_diff(zc) != 1 {
                    continue;
                }
                let mut next_vals = vals;
                next_vals.swap(zi, i);
                let next = pack(next_vals);
                if prev.contains_key(&next) {
                    continue;
                }
                prev.insert(next, (state, vals[i]));
                queue.push_back(next);
            }
        }
    }

    fn is_solved(&self) -> bool {
        for r in 0..self.n {
            for c in 0..self.n {
                if self.board[r][c] != Self::goal(self.n, r, c) {
                    return false;
                }
            }
        }
        true
    }

    fn pack_2x2(&self) -> u32 {
        let n = self.n;
        pack([
            self.board[n - 2][n - 2],
            self.board[n - 2][n - 1],
            self.board[n - 1][n - 2],
            self.board[n - 1][n - 1],
        ])
    }

    fn pos(&self, val: u8) -> (usize, usize) {
        for (r, row) in self.board.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                if v == val {
                    return (r, c);
                }
            }
        }
        unreachable!("missing tile {val}");
    }

    fn arrange(
        &mut self,
        a: u8,
        ar: usize,
        ac: usize,
        b: u8,
        br: usize,
        bc: usize,
        er: usize,
        ec: usize,
    ) {
        let (asr, asc) = self.pos(a);
        let (bsr, bsc) = self.pos(b);
        if (asr, asc) == (ar, ac) && (bsr, bsc) == (br, bc) && (self.zr, self.zc) == (er, ec) {
            return;
        }

        type State = (u8, u8, u8);
        let encode = |r: usize, c: usize| (r * self.n + c) as u8;
        let start: State = (encode(asr, asc), encode(bsr, bsc), encode(self.zr, self.zc));
        let goal: State = (encode(ar, ac), encode(br, bc), encode(er, ec));

        let mut prev: std::collections::HashMap<State, (State, (usize, usize))> =
            std::collections::HashMap::new();
        prev.insert(start, (start, (0, 0)));
        let mut q = std::collections::VecDeque::from([start]);
        let mut found = false;

        while let Some(state) = q.pop_front() {
            if state == goal {
                found = true;
                break;
            }
            let (ap, bp, ep) = state;
            let (er0, ec0) = (ep as usize / self.n, ep as usize % self.n);
            for (nr, nc) in self.neighbors(er0, ec0) {
                if self.locked[nr][nc] {
                    continue;
                }
                let np = encode(nr, nc);
                let (na, nb) = if np == ap {
                    (ep, bp)
                } else if np == bp {
                    (ap, ep)
                } else {
                    (ap, bp)
                };
                let next = (na, nb, np);
                if prev.contains_key(&next) {
                    continue;
                }
                prev.insert(next, (state, (nr, nc)));
                q.push_back(next);
            }
        }

        if !found {
            panic!(
                "no path to arrange {a} -> ({ar},{ac}), {b} -> ({br},{bc}), empty -> ({er},{ec})\nboard: {:?}\nlocked: {:?}",
                self.board, self.locked
            );
        }

        let mut cur = goal;
        let mut slides = Vec::new();
        while cur != start {
            let (p, cell) = prev[&cur];
            slides.push(cell);
            cur = p;
        }
        slides.reverse();
        for (r, c) in slides {
            self.slide(r, c);
        }
    }

    fn move_tile_to(&mut self, val: u8, tr: usize, tc: usize) {
        let (sr, sc) = self.pos(val);
        if (sr, sc) == (tr, tc) {
            return;
        }

        type State = (usize, usize, usize, usize);
        let start: State = (sr, sc, self.zr, self.zc);
        let mut prev: std::collections::HashMap<State, (State, (usize, usize))> =
            std::collections::HashMap::new();
        prev.insert(start, (start, (0, 0)));
        let mut q = std::collections::VecDeque::from([start]);
        let mut found = None;

        while let Some((tile_r, tile_c, er, ec)) = q.pop_front() {
            if (tile_r, tile_c) == (tr, tc) {
                found = Some((tile_r, tile_c, er, ec));
                break;
            }
            for (nr, nc) in self.neighbors(er, ec) {
                if self.locked[nr][nc] {
                    continue;
                }
                let (ntr, ntc) = if (nr, nc) == (tile_r, tile_c) {
                    (er, ec)
                } else {
                    (tile_r, tile_c)
                };
                let next = (ntr, ntc, nr, nc);
                if prev.contains_key(&next) {
                    continue;
                }
                prev.insert(next, ((tile_r, tile_c, er, ec), (nr, nc)));
                q.push_back(next);
            }
        }

        let mut cur = found.unwrap_or_else(|| {
            panic!(
                "no path for tile {val} to ({tr},{tc})\nboard: {:?}\nlocked: {:?}\nempty: ({},{})",
                self.board, self.locked, self.zr, self.zc
            )
        });
        let mut slides = Vec::new();
        while cur != start {
            let (p, cell) = prev[&cur];
            slides.push(cell);
            cur = p;
        }
        slides.reverse();
        for (r, c) in slides {
            self.slide(r, c);
        }
    }

    fn slide(&mut self, r: usize, c: usize) {
        let tile = self.board[r][c];
        debug_assert_ne!(tile, 0);
        debug_assert_eq!(r.abs_diff(self.zr) + c.abs_diff(self.zc), 1);
        self.board[self.zr][self.zc] = tile;
        self.board[r][c] = 0;
        self.zr = r;
        self.zc = c;
        self.moves.push(tile);
    }

    fn neighbors(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut out = Vec::with_capacity(4);
        if r > 0 {
            out.push((r - 1, c));
        }
        if r + 1 < self.n {
            out.push((r + 1, c));
        }
        if c > 0 {
            out.push((r, c - 1));
        }
        if c + 1 < self.n {
            out.push((r, c + 1));
        }
        out
    }
}

fn pack(vals: [u8; 4]) -> u32 {
    (vals[0] as u32) << 24 | (vals[1] as u32) << 16 | (vals[2] as u32) << 8 | vals[3] as u32
}

fn unpack(state: u32) -> [u8; 4] {
    [
        (state >> 24) as u8,
        (state >> 16) as u8,
        (state >> 8) as u8,
        state as u8,
    ]
}

#[cfg(test)]
mod preloaded;

#[cfg(test)]
mod sample_tests {
    use super::preloaded::assert_valid_solution;
    use super::slide_puzzle;

    const TESTS: [&[&[u8]]; 3] = [
        &[&[4, 1, 3], &[2, 8, 0], &[7, 6, 5]],
        &[
            &[10, 3, 6, 4],
            &[1, 5, 8, 0],
            &[2, 13, 7, 15],
            &[14, 9, 12, 11],
        ],
        &[
            &[3, 7, 14, 15, 10],
            &[1, 0, 5, 9, 4],
            &[16, 2, 11, 12, 8],
            &[17, 6, 13, 18, 20],
            &[21, 22, 23, 19, 24],
        ],
    ];

    macro_rules! test_puzzle {
        ($name:ident, $idx:literal) => {
            #[test]
            fn $name() {
                let puzzle: Vec<_> = TESTS[$idx].iter().map(|r| r.to_vec()).collect();
                assert_valid_solution(&puzzle, slide_puzzle(&puzzle));
            }
        };
    }

    test_puzzle!(_1_sample_puzzle_3x3, 0);
    test_puzzle!(_2_sample_puzzle_4x4, 1);
    test_puzzle!(_3_sample_puzzle_5x5, 2);
}

#[cfg(test)]
mod extra_tests {
    use super::preloaded::assert_valid_solution;
    use super::slide_puzzle;

    #[test]
    fn already_solved() {
        let puzzle = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
        assert_valid_solution(&puzzle, slide_puzzle(&puzzle));
    }

    #[test]
    fn documented_example() {
        let puzzle = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 6, 8],
            vec![9, 10, 7, 11],
            vec![13, 14, 15, 12],
        ];
        assert_valid_solution(&puzzle, slide_puzzle(&puzzle));
    }

    #[test]
    fn unsolvable_3x3() {
        let puzzle = vec![vec![1, 2, 3], vec![4, 5, 6], vec![8, 7, 0]];
        assert_eq!(slide_puzzle(&puzzle), None);
    }

    #[test]
    fn scrambled_sizes() {
        for n in 3..=10 {
            for seed in 1..8 {
                let puzzle = scramble(n, n * n * 40, seed * 17 + n as u64);
                assert_valid_solution(&puzzle, slide_puzzle(&puzzle));
            }
        }
    }

    fn scramble(n: usize, steps: usize, mut seed: u64) -> Vec<Vec<u8>> {
        let mut board = vec![vec![0u8; n]; n];
        for r in 0..n {
            for c in 0..n {
                board[r][c] = if r + 1 == n && c + 1 == n {
                    0
                } else {
                    (r * n + c + 1) as u8
                };
            }
        }
        let mut zr = n - 1;
        let mut zc = n - 1;
        for _ in 0..steps {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mut dirs = Vec::new();
            if zr > 0 {
                dirs.push((-1isize, 0isize));
            }
            if zr + 1 < n {
                dirs.push((1, 0));
            }
            if zc > 0 {
                dirs.push((0, -1));
            }
            if zc + 1 < n {
                dirs.push((0, 1));
            }
            let (dr, dc) = dirs[seed as usize % dirs.len()];
            let nr = (zr as isize + dr) as usize;
            let nc = (zc as isize + dc) as usize;
            board[zr][zc] = board[nr][nc];
            board[nr][nc] = 0;
            zr = nr;
            zc = nc;
        }
        board
    }
}