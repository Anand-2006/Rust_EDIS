use crate::command::Command;

pub fn parse(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["SET", key, value] => Command::Set(key.to_string(), value.to_string()),
        ["GET", key] => Command::Get(key.to_string()),
        ["DEL", key] => Command::Del(key.to_string()),
        ["EXIT"] => Command::Exit,
        _ => Command::Invalid,
    }
}