use super::super::Mega;

impl From<Mega> for String {
    fn from(command: Mega) -> Self {
        match command {
            Mega::LS => String::from("mega-ls.bat"),
            Mega::GET => String::from("mega-get.bat"),
            Mega::LOGIN => String::from("mega-login.bat")
        }
    }
}