/*
 * @Date: 2024-12-23 21:54:33
 * @LastEditors: SunsJay SunsJay0806@gmail.com
 * @LastEditTime: 2024-12-23 22:37:02
 * @FilePath: /exam-grading/rustlings/exercises/03_if/if3.rs
 * @Description:
 */
fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        4
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
