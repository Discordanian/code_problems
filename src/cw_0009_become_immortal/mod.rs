/*
In the nation of CodeWars, there lives an Elder who has lived for a long time. Some people call him the Grandpatriarch, but most people just refer to him as the Elder.

There is a secret to his longetivity: he has a lot of young worshippers, who regularly perform a ritual to ensure that the Elder stays immortal:

The worshippers line up in a magic rectangle, of dimensions m and n.
They channel their will to wish for the Elder. In this magic rectangle, any worshipper can donate time equal to the xor of the column and the row (zero-indexed) he's on, in seconds, to the Elder.
However, not every ritual goes perfectly. The donation of time from the worshippers to the Elder will experience a transmission loss l (in seconds). Also, if a specific worshipper cannot channel more than l seconds, the Elder will not be able to receive this worshipper's donation.
The estimated age of the Elder is so old it's probably bigger than the total number of atoms in the universe. However, the lazy programmers (who made a big news by inventing the Y2K bug and other related things) apparently didn't think thoroughly enough about this, and so their simple date-time system can only record time from 0 to t-1 seconds. If the elder received the total amount of time (in seconds) more than the system can store, it will be wrapped around so that the time would be between the range 0 to t-1.

Given m, n, l and t, please find the number of seconds the Elder has received, represented in the poor programmer's date-time system.

(Note: t will never be bigger than 2^32 - 1, and in JS, 2^26 - 1.)

Example:

m=8, n=5, l=1, t=100

Let's draw out the whole magic rectangle:
0 1 2 3 4 5 6 7
1 0 3 2 5 4 7 6
2 3 0 1 6 7 4 5
3 2 1 0 7 6 5 4
4 5 6 7 0 1 2 3

Applying a transmission loss of 1:
0 0 1 2 3 4 5 6
0 0 2 1 4 3 6 5
1 2 0 0 5 6 3 4
2 1 0 0 6 5 4 3
3 4 5 6 0 0 1 2

Adding up all the time gives 105 seconds.

Because the system can only store time between 0 to 99 seconds, the first 100 seconds of time will be lost, giving the answer of 5.
This is no ordinary magic (the Elder's life is at stake), so you need to care about performance. All test cases (900 tests) can be passed within 1 second, but naive solutions will time out easily. Good luck, and do not displease the Elder.
*/

pub fn elder_age(m: u64, n: u64, l: u64, t: u64) -> u64 {
    if t == 0 {
        return 0;
    }
    rect_sum(m, n, l, t)
}

fn next_pow2(x: u64) -> u128 {
    let mut p = 1u128;
    let x = x as u128;
    while p < x {
        p <<= 1;
    }
    p
}

fn add_mod(a: u64, b: u64, t: u64) -> u64 {
    ((a as u128 + b as u128) % t as u128) as u64
}

fn sub_mod(a: u64, b: u64, t: u64) -> u64 {
    ((a as u128 + t as u128 - b as u128) % t as u128) as u64
}

fn mul_mod(a: u128, b: u128, t: u64) -> u64 {
    let t = t as u128;
    ((a % t) * (b % t) % t) as u64
}

/// Arithmetic series (lo + ... + hi) modulo t. Divides by 2 before reducing
/// so this stays correct when t is even.
fn range_sum_mod(lo: i128, hi: i128, t: u64) -> u64 {
    if hi < lo {
        return 0;
    }
    let mut a = (lo + hi) as u128;
    let mut b = (hi - lo + 1) as u128;
    if a % 2 == 0 {
        a /= 2;
    } else {
        b /= 2;
    }
    mul_mod(a, b, t)
}

/// Sum of max(0, (i ^ j) - l) over i in 0..m, j in 0..n, modulo t.
fn rect_sum(mut m: u64, mut n: u64, l: u64, t: u64) -> u64 {
    if m == 0 || n == 0 {
        return 0;
    }
    if m > n {
        std::mem::swap(&mut m, &mut n);
    }

    let ln = next_pow2(n);
    let mut lm = next_pow2(m);
    let l128 = l as u128;
    if l128 > ln {
        return 0;
    }

    if lm == ln {
        let series = range_sum_mod(1, ln as i128 - l as i128 - 1, t);
        let count = m as u128 + n as u128 - ln;
        return add_mod(
            mul_mod(series as u128, count, t),
            rect_sum((ln - n as u128) as u64, (lm - m as u128) as u64, l, t),
            t,
        );
    }

    lm = ln / 2;
    let mut total = mul_mod(
        range_sum_mod(1, ln as i128 - l as i128 - 1, t) as u128,
        m as u128,
        t,
    );
    total = sub_mod(
        total,
        mul_mod(
            ln - n as u128,
            range_sum_mod((lm as i128 - l as i128).max(0), ln as i128 - l as i128 - 1, t)
                as u128,
            t,
        ),
        t,
    );

    if l128 <= lm {
        total = add_mod(
            total,
            mul_mod(
                mul_mod(lm - l128, lm - m as u128, t) as u128,
                ln - n as u128,
                t,
            ),
            t,
        );
        add_mod(total, rect_sum((lm - m as u128) as u64, (ln - n as u128) as u64, 0, t), t)
    } else {
        add_mod(
            total,
            rect_sum((lm - m as u128) as u64, (ln - n as u128) as u64, l - lm as u64, t),
            t,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(elder_age(8, 5, 1, 100), 5);
        assert_eq!(elder_age(8, 8, 0, 100007), 224);
        assert_eq!(elder_age(25, 31, 0, 100007), 11925);
        assert_eq!(elder_age(5, 45, 3, 1000007), 4323);
        assert_eq!(elder_age(31, 39, 7, 2345), 1586);
        assert_eq!(elder_age(545, 435, 342, 1000007), 808451);

        // You need to run this test very quickly before attempting the actual tests :)
        assert_eq!(
            elder_age(28827050410, 35165045587, 7109602, 13719506),
            5456283
        );
        assert_eq!(
            elder_age(1630029867390898003, 2001333086544497128, 533501, 978505085),
            446863989
        );
    }

    fn naive(m: u64, n: u64, l: u64, t: u64) -> u64 {
        let mut sum = 0u128;
        for i in 0..m {
            for j in 0..n {
                let xor = i ^ j;
                if xor > l {
                    sum += (xor - l) as u128;
                }
            }
        }
        (sum % t as u128) as u64
    }

    #[test]
    fn matches_naive_on_small_rectangles() {
        let t = 1000007;
        for m in 0..24 {
            for n in 0..24 {
                for l in [0, 1, 3, 7, 15, 31] {
                    assert_eq!(
                        elder_age(m, n, l, t),
                        naive(m, n, l, t),
                        "m={m} n={n} l={l}"
                    );
                }
            }
        }
    }
}
