#[allow(dead_code)]
pub fn warn_the_sheep(queue: &[&str]) -> String {
    let mut my_queue = vec![];
    for a in queue {
        my_queue.push(a);
    }
    my_queue.reverse();
    let mut i = 0;
    for animal in my_queue {
        match animal {
            &"wolf" => break,
            _ => i += 1,
        }
    }
    match i {
        0 => String::from("Pls go away and stop eating my sheep"),
        _n => format!(
            "Oi! Sheep number {}! You are about to be eaten by a wolf!",
            i
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            warn_the_sheep(&[
                "sheep", "sheep", "sheep", "sheep", "sheep", "wolf", "sheep", "sheep"
            ]),
            "Oi! Sheep number 2! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 5! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["wolf", "sheep", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 6! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep"]),
            "Oi! Sheep number 1! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "sheep", "wolf"]),
            "Pls go away and stop eating my sheep",
        );
    }
}
