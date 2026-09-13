pub fn triangle(row: &str) -> char {
    let mut colors: Vec<u8> = row
        .bytes()
        .map(|c| match c {
            b'R' => 0,
            b'G' => 1,
            _ => 2,
        })
        .collect();

    let mut n = colors.len();
    while n > 1 {
        let mut step = 1;
        while step * 3 <= n - 1 {
            step *= 3;
        }
        let m = n - step;
        for i in 0..m {
            colors[i] = (2 * (colors[i] + colors[i + step])) % 3;
        }
        n = m;
    }

    [b'R', b'G', b'B'][colors[0] as usize] as char
}
#[cfg(test)]
mod tests {
    use super::triangle;

    fn dotest(s: &str, expected: char) {
        let actual = triangle(s);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right) for row = {s:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        for (row, expected) in [
            ("B", 'B'),
            ("GB", 'R'),
            ("RRR", 'R'),
            ("RGBG", 'B'),
            ("RBRGBRB", 'G'),
            ("RBRGBRBGGRRRBGBBBGG", 'G'),
        ] {
            dotest(row, expected);
        }
    }

    fn naive(row: &str) -> char {
        let mut cur: Vec<u8> = row
            .bytes()
            .map(|c| match c {
                b'R' => 0,
                b'G' => 1,
                _ => 2,
            })
            .collect();
        while cur.len() > 1 {
            cur = cur.windows(2).map(|w| (2 * (w[0] + w[1])) % 3).collect();
        }
        [b'R', b'G', b'B'][cur[0] as usize] as char
    }

    #[test]
    fn matches_naive_for_small_rows() {
        const COLORS: &[u8] = b"RGB";
        for len in 1..=8 {
            let mut row = vec![b'R'; len];
            let total = 3usize.pow(len as u32);
            for mask in 0..total {
                let mut m = mask;
                for slot in row.iter_mut() {
                    *slot = COLORS[m % 3];
                    m /= 3;
                }
                let s = std::str::from_utf8(&row).unwrap();
                assert_eq!(triangle(s), naive(s), "row = {s}");
            }
        }
    }
}
