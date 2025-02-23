pub fn quarter_of(month: u8) -> u8 {
    1 + (month - 1) / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_3() {
        assert_eq!(quarter_of(3), 1, "Month 3 = quarter 1");
    }

    #[test]
    fn month_8() {
        assert_eq!(quarter_of(8), 3, "Month 8 = quarter 3");
    }

    #[test]
    fn month_11() {
        assert_eq!(quarter_of(11), 4, "Month 11 = quarter 4");
    }
}
