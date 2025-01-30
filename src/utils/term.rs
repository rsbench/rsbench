use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};
use std::io::stdout;

pub fn clear_last_line() {
    execute!(
        stdout(),
        cursor::MoveUp(1),
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine)
    )
    .unwrap();
}

pub fn clear_screen() {
    execute!(
        stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}

pub fn process_decimal_point(input: &str) -> String {
    let mut input = input.to_string();
    if input.ends_with(".") {
        if let Some(_) = input.pop() {
            input.push(' ');
        }
    }
    input
}
