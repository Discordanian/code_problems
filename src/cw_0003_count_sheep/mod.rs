/*
* https://www.codewars.com/kata/5b077ebdaf15be5c7f000077/train/rust
*/

#[allow(dead_code)]
fn count_sheep(n: u32) -> String {
    let mut retval = String::new();
    for i in 1..=n {
        retval.push_str(&format!("{i} sheep..."));
    }
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(count_sheep(0), "");
        assert_eq!(count_sheep(1), "1 sheep...");
        assert_eq!(count_sheep(2), "1 sheep...2 sheep...");
        assert_eq!(count_sheep(3), "1 sheep...2 sheep...3 sheep...");
    }
}
