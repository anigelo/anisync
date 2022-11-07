use super::super::Mega;

impl From<Mega> for String {
    fn from(command: Mega) -> Self {
        match command {
            Mega::LS => String::from("mega-ls"),
            Mega::GET => String::from("mega-get"),
            Mega::LOGIN => String::from("mega-login")
        }
    }
}