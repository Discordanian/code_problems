// https://www.codewars.com/kata/659af96994b858db10e1675f/train/rust
#[allow(dead_code)]
pub fn find(s: &str) -> u32 {
    let string_length = s.len();

    let x: Vec<char> = s.chars().collect();

    x.len().try_into().expect("Can cast u32 to usize")
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
