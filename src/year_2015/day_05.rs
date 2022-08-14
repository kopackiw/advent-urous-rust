fn is_nice_a(s: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let doubled_letter_regex = fancy_regex::Regex::new(r"(\w)\1+").unwrap();

    let has_no_blocked_pattern = ["ab", "cd", "pq", "xy"]
        .into_iter()
        .all(|blocked| !s.contains(blocked));
    let has_at_least_three_vowels = s.matches(|c| vowels.contains(&c)).count() >= 3;
    let has_doubled_letter = doubled_letter_regex.is_match(s).unwrap();

    has_no_blocked_pattern && has_at_least_three_vowels && has_doubled_letter
}

fn is_nice_b(s: &str) -> bool {
    let non_overlapping_pairs_regex = fancy_regex::Regex::new(r"(\w\w).*\1+").unwrap();
    let repeating_letter_with_intersperse_regex = fancy_regex::Regex::new(r"(\w)\w\1").unwrap();

    non_overlapping_pairs_regex.is_match(s).unwrap()
        && repeating_letter_with_intersperse_regex.is_match(s).unwrap()
}

fn solve(f: fn(&str) -> bool, s: &str) -> usize {
    s.lines().map(f).filter(|x| *x).count()
}

fn solve_a(s: &str) -> usize {
    solve(is_nice_a, s)
}

fn solve_b(s: &str) -> usize {
    solve(is_nice_b, s)
}

#[cfg(test)]
mod test {

    use super::{is_nice_a, is_nice_b};

    #[test]
    fn test_is_nice_a() {
        assert_eq!(is_nice_a("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_a("aaa"), true);
        assert_eq!(is_nice_a("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_a("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_a("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_is_nice_b() {
        assert_eq!(is_nice_b("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_b("xxyxx"), true);
        assert_eq!(is_nice_b("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_b("ieodomkazucvgmuy"), false);
    }
}
