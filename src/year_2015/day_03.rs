use std::{collections::HashSet, iter::once};

enum Move {
    North,
    East,
    South,
    West,
}
type Coordinates = (isize, isize);

fn to_move(c: char) -> Option<Move> {
    match c {
        '^' => Some(Move::North),
        '>' => Some(Move::East),
        'v' => Some(Move::South),
        '<' => Some(Move::West),
        _ => None,
    }
}

fn on_move(m: Move, c: Coordinates) -> Coordinates {
    match m {
        Move::North => (c.0, c.1 + 1),
        Move::East => (c.0 + 1, c.1),
        Move::South => (c.0, c.1 - 1),
        Move::West => (c.0 - 1, c.1),
    }
}

fn solve(s: &str) -> HashSet<Coordinates> {
    let coordinates: Coordinates = (0, 0);

    let all_coordinates = s
        .chars()
        .scan(coordinates, |state, x| {
            *state = on_move(to_move(x)?, *state);

            Some(state.clone())
        })
        .chain(once(coordinates));

    HashSet::<_>::from_iter(all_coordinates)
}

fn solve_a(s: &str) -> usize {
    solve(s).len()
}

fn solve_b(s: &str) -> usize {
    let original_santa = s.chars().step_by(2).collect::<String>();
    let robo_santa = s.chars().skip(1).step_by(2).collect::<String>();

    solve(&original_santa).union(&solve(&robo_santa)).count()
}

#[cfg(test)]
mod test {
    use crate::year_2015::day_03::{solve_a, solve_b};

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(">"), 2);
        assert_eq!(solve_a("^>v<"), 4);
        assert_eq!(solve_a("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b("^v"), 3);
        assert_eq!(solve_b("^>v<"), 3);
        assert_eq!(solve_b("^v^v^v^v^v"), 11);
    }
}
