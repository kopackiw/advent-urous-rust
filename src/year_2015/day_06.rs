use itertools::Itertools;

type Board = Vec<Vec<usize>>;
type Coordinates = (usize, usize);

struct Command {
    from: Coordinates,
    to: Coordinates,
    command_type: CommandType,
}

enum CommandType {
    TurnOn,
    TurnOff,
    Toggle,
}

fn parse_coordinates(s: &str) -> Coordinates {
    s.split(',')
        .flat_map(|x| x.parse())
        .next_tuple::<Coordinates>()
        .unwrap()
}

fn parse_command(command_type: &str, from: &str, to: &str) -> Option<Command> {
    let from = parse_coordinates(from);
    let to = parse_coordinates(to);

    match command_type {
        "toggle" => Some(Command {
            command_type: CommandType::Toggle,
            from,
            to,
        }),
        "turn on" => Some(Command {
            command_type: CommandType::TurnOn,
            from,
            to,
        }),
        "turn off" => Some(Command {
            command_type: CommandType::TurnOff,
            from,
            to,
        }),
        _ => None,
    }
}

fn parse(s: &str) -> Option<Command> {
    let command = s
        .chars()
        .take_while(|c| !char::is_ascii_digit(&c))
        .collect::<String>();

    let coords = s
        .chars()
        .skip_while(|c| !char::is_ascii_digit(c))
        .collect::<String>();

    let mut a = coords.split(' ');
    let from = a.next().unwrap();
    let to = a.skip(1).next().unwrap();

    parse_command(command.trim(), from, to)
}

fn set_values(
    board: &mut Board,
    command: &Command,
    command_mapping_strategy: fn(&CommandType) -> fn(usize) -> usize,
) -> Board {
    (command.from.0..=command.to.0).for_each(|line| {
        (command.from.1..=command.to.1).for_each(|column| {
            board[line][column] =
                command_mapping_strategy(&command.command_type)(board[line][column])
        });
    });

    board.to_vec()
}

fn solve(command_mapping_strategy: fn(&CommandType) -> fn(usize) -> usize, s: &str) -> usize {
    let init = vec![vec![0; 1_000]; 1_000];
    s.lines()
        .flat_map(parse)
        .fold(init, |mut acc, command| {
            set_values(&mut acc, &command, command_mapping_strategy)
        })
        .into_iter()
        .map(|line| line.into_iter().sum::<usize>())
        .sum()
}

fn command_mapping_strategy_a(command_type: &CommandType) -> fn(usize) -> usize {
    match command_type {
        CommandType::Toggle => |x| if x == 0 { 1 } else { 0 },
        CommandType::TurnOn => |_| 1,
        CommandType::TurnOff => |_| 0,
    }
}

fn solve_a(s: &str) -> usize {
    solve(command_mapping_strategy_a, s)
}

fn command_mapping_strategy_b(command_type: &CommandType) -> fn(usize) -> usize {
    match command_type {
        CommandType::Toggle => |x| x + 2,
        CommandType::TurnOn => |x| x + 1,
        CommandType::TurnOff => |x| std::cmp::max(0, x - 1),
    }
}

fn solve_b(s: &str) -> usize {
    solve(command_mapping_strategy_b, s)
}

#[cfg(test)]
mod test {

    use super::{solve_a, solve_b};

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a("turn on 0,0 through 999,999"), 1_000_000);
        assert_eq!(
            solve_a("turn on 0,0 through 999,999\ntoggle 100,0 through 199,100"),
            989_900
        );
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b("toggle 0,0 through 999,999"), 2_000_000);
    }
}
