fn solve_a(key: &str) -> Option<usize> {
    (0..std::usize::MAX).position(|num| {
        let hash = md5::compute(key.to_owned() + &num.to_string());
        // I didn't now this low-level bits manipulation.
        // Learned this through this nice discussion:
        // https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
        hash[..2] == [0, 0] && hash[2] <= 0x0F
    })
}

fn solve_b(key: &str) -> Option<usize> {
    (0..std::usize::MAX).position(|num| {
        let hash = md5::compute(key.to_owned() + &num.to_string());
        hash[..3] == [0, 0, 0]
    })
}

#[cfg(test)]
mod test {
    use super::{solve_a, solve_b};

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a("abcdef"), Some(609043));
        assert_eq!(solve_a("pqrstuv"), Some(1048970));
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b("iwrupvqb"), Some(9958218));
    }
}
