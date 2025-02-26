#[allow(dead_code)]
pub fn pillars(num_of_pillars: u32, distance: u32, width: u32) -> u32 {
    100 * (num_of_pillars - 1) * distance
        + match num_of_pillars {
            0..2 => 0,
            x => (x - 2) * width,
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(pillars(1, 10, 10), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(pillars(2, 20, 25), 2000);
    }

    #[test]
    fn test3() {
        assert_eq!(pillars(11, 15, 30), 15270);
    }
}
