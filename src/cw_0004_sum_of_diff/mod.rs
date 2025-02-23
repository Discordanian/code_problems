/*
 *
* https://www.codewars.com/kata/5b73fe9fb3d9776fbf00009e/train/rust
*/

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.len() < 2 {
        return None;
    }
    let vec: Vec<i8> = arr.iter().copied().rev().collect();
    let mut prev = vec[0];
    let mut retval = 0;
    for i in vec {
        retval += (i - prev).abs();
        prev = i;
    }
    Some(retval)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERR_MSG: &str = "\nYour results (left) did not match the expected output (right)";
    #[test]
    fn returns_expected() {
        assert_eq!(sum_of_differences(&[]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[0]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1, 2, 10]), Some(9), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-3, -2, -1]), Some(2), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1, 1, 1, 1]), Some(0), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[17, -17]), Some(34), "{}", ERR_MSG);
    }
}
