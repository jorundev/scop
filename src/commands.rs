#[derive(Debug, Clone)]
pub enum Command {
    LoadModel(String),
}

pub struct CommandInterpreter {}

impl CommandInterpreter {
    pub fn listen() -> Command {
        Command::LoadModel("yep".to_string())
    }
}
