use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct PresentParsingError(String);

impl Display for PresentParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Cannot parse '{}' to a Present", self.0))
    }
}

pub struct Present {
    dimensions: [usize; 3],
}

impl FromStr for Present {
    type Err = PresentParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('x')
            .flat_map(|c| c.parse::<usize>())
            .next_tuple()
            .map_or_else(
                || Err(PresentParsingError(s.to_string())),
                |(w, h, l)| {
                    let mut dimensions = [w, h, l];
                    dimensions.sort();
                    Ok(Present { dimensions })
                },
            )
    }
}

impl Present {
    fn areas(&self) -> [usize; 3] {
        self.dimensions
            .iter()
            .combinations(2)
            .map(|edges| edges.into_iter().product())
            .collect_vec()
            .try_into()
            .unwrap()
    }
}

pub fn slack_for_present(present: Present) -> usize {
    let areas = present.areas();

    2 * areas.iter().sum::<usize>() + areas[0]
}

pub fn ribbon_for_present(present: Present) -> usize {
    let dimensions = present.dimensions;
    let wrapper = 2 * (dimensions[0] + dimensions[1]);
    let bow = dimensions.iter().product::<usize>();

    wrapper + bow
}

fn solve(map_with: fn(Present) -> usize, s: &str) -> Result<usize, PresentParsingError> {
    let a = s
        .lines()
        .map(Present::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(a.into_iter().map(map_with).sum())
}

pub fn solve_a(s: &str) -> Result<usize, PresentParsingError> {
    solve(slack_for_present, s)
}

pub fn solve_b(s: &str) -> Result<usize, PresentParsingError> {
    solve(ribbon_for_present, s)
}

#[cfg(test)]
mod test {
    use crate::year_2015::day_02::{solve_a, solve_b};

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a("2x3x4"), Ok(58));
        assert_eq!(solve_a("1x1x10"), Ok(43));
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b("2x3x4"), Ok(34));
        assert_eq!(solve_b("1x1x10"), Ok(14));
    }
}
