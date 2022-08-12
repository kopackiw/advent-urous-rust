enum Direction {
    Up,
    Down,
}

fn char_to_direction(c: char) -> Option<Direction> {
    match c {
        '(' => Some(Direction::Up),
        ')' => Some(Direction::Down),
        _ => None,
    }
}

fn next_floor(floor: isize, direction: Direction) -> isize {
    match direction {
        Direction::Up => floor + 1,
        Direction::Down => floor - 1,
    }
}

pub fn solve_a(text: &str) -> isize {
    text.chars()
        .filter_map(char_to_direction)
        .fold(0, next_floor)
}

pub fn solve_b(text: &str) -> Option<usize> {
    let mut current_floor = 0;

    text.chars()
        .filter_map(char_to_direction)
        .position(|direction| {
            current_floor = next_floor(current_floor, direction);

            current_floor == -1
        })
        .map(|p| p + 1)
}

#[cfg(test)]
mod tests {
    use crate::year_2015::day_01::{solve_a, solve_b};

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a("(())"), 0);
        assert_eq!(solve_a("()()"), 0);
        assert_eq!(solve_a("((("), 3);
        assert_eq!(solve_a("(()(()("), 3);
        assert_eq!(solve_a("))((((("), 3);
        assert_eq!(solve_a("())"), -1);
        assert_eq!(solve_a("))("), -1);
        assert_eq!(solve_a(")))"), -3);
        assert_eq!(solve_a(")())())"), -3);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(")"), Some(1));
        assert_eq!(solve_b("()())"), Some(5));
        assert_eq!(solve_b("((("), None);
    }
}
