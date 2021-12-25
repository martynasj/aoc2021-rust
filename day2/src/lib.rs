pub fn calculate_final_pos(input: &String) -> i32 {
    let mut initial_pos = Position { x: 0, y: 0 };

    input.lines().for_each(|l| {
        let cmd = create_command_from_str(l);
        if let Some(cmd) = cmd {
            initial_pos.apply_command(cmd)
        }
    });

    initial_pos.x * initial_pos.y
}

pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn apply_command(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(x) => self.x = self.x + x,
            Command::Up(x) => self.y = self.y - x,
            Command::Down(x) => self.y = self.y + x,
        }
    }
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn create_command_from_str(input: &str) -> Option<Command> {
    let (cmd, val) = input.split_once(" ")?;
    let val_nmb: i32 = val.parse().ok()?;

    let cmd = match cmd {
        "forward" => Some(Command::Forward(val_nmb)),
        "up" => Some(Command::Up(val_nmb)),
        "down" => Some(Command::Down(val_nmb)),
        _ => None,
    }?;

    Some(cmd)
}

#[test]
fn test_calc_final_pos() {
    let result = calculate_final_pos(&include_str!("../input.txt").to_string());
    assert_eq!(result, 1815044);
}
