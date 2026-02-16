// https://www.codewars.com/kata/659af96994b858db10e1675f/train/rust
/*
You're given a string of digits representing a sequence of consecutive natural numbers concatenated together. Your task is to find the smallest possible first number in the sequence. The sequence starts with a single or multi-digit number and continues with numbers each incremented by 1. If multiple sequences can be formed, choose the one that starts with the smallest number.


 */
#[allow(dead_code)]
pub fn find(s: &str) -> u32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    let mut best = u32::MAX;

    // Try all possible lengths for the first number
    for len in 1..=n {
        if len > 1 && chars[0] == '0' {
            continue; // leading zero not allowed for natural numbers
        }

        let first_str: String = chars[0..len].iter().collect();
        let first: u32 = match first_str.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Check if the rest of the string matches the sequence starting from first+1
        let mut pos = len;
        let mut current = first + 1;
        let mut valid = true;

        while pos < n && valid {
            let next_str = current.to_string();
            let next_len = next_str.len();

            if pos + next_len > n {
                valid = false;
                break;
            }

            for (i, c) in next_str.chars().enumerate() {
                if chars[pos + i] != c {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }

            pos += next_len;
            current += 1;
        }

        if valid && pos == n {
            best = best.min(first);
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(find("123456789101112131415"), 1);
        assert_eq!(find("17181920"), 17);
        assert_eq!(find("72637236"), 72637236);
        assert_eq!(find("1112"), 11);
        assert_eq!(find("91011"), 9);
        assert_eq!(find("99100"), 99);
        assert_eq!(find("431243"), 431243);
        assert_eq!(find("577495"), 577495);
    }
}
