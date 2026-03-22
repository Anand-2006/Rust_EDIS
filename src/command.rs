pub enum Command {
    Set(String, String),
    Get(String),
    Del(String),
    Exit,
    Invalid,
}