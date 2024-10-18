use std::io::{self, stdin, Write};
use std::time::Duration;
use crossterm::event::KeyCode;
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use engine::parser::Commands;

fn main() {
    println!("Savant shell");

    let mut stdout = io::BufWriter::new(io::stdout());
    enable_raw_mode().expect("Failed to enable raw mode");

    loop {
        println!(" ");
        print!("> ");
        io::stdout().flush().unwrap();


        let mut input = String::new();
        loop {
            // no duration
            if crossterm::event::poll(Duration::from_secs(0)).unwrap() {
                let event = crossterm::event::read().unwrap();
                match event {
                    crossterm::event::Event::Key(key_event) => {
                        if key_event.kind == crossterm::event::KeyEventKind::Release {
                            continue;
                        }
                        match key_event.code {
                            KeyCode::Enter => {
                                writeln!(stdout, "").unwrap();
                                stdout.flush().unwrap();
                                let command = engine::parser::parse_command(&input);
                                if let Some(cli) = command {
                                    engine::handler::handle_command(cli);
                                } else {
                                    println!("Failed to parse command");
                                }
                                break;
                            }
                            KeyCode::Backspace => {
                                if !input.is_empty() {
                                    input.pop();
                                    stdout.execute(crossterm::cursor::MoveLeft(1)).unwrap();
                                    write!(stdout, " ").unwrap();
                                    stdout.execute(crossterm::cursor::MoveLeft(1)).unwrap();
                                    stdout.flush().unwrap();
                                }
                            }
                            KeyCode::Tab => {
                                // TODO: suggestions
                            }
                            KeyCode::Char(c) => {
                                input.push(c);
                                write!(stdout, "{}", c).unwrap();
                                stdout.flush().unwrap();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}