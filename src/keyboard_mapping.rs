use std::fmt::{Display, Formatter};

// Creat a const for the dvorak keyboard mapping
const QWERTY: &[u8] = r#"`1234567890-=qwertyuiop[]\asdfghjkl;'zxcvbnm,./ ~!@#$%^&*()_+QWERTYUIOP{}|ASDFGHJKL:"ZXCVBNM<>? "#.as_bytes();
const DVORAK: &[u8] = r#"`1234567890[]',.pyfgcrl/=\aoeuidhtns-;qjkxbmwvz ~!@#$%^&*(){}"<>PYFGCRL?+|AOEUIDHTNS_:QJKXBMWVZ "#.as_bytes();
const COLEMARK: &[u8] = r#"`1234567890-=qwfpbjluy;[]arstdhneio'zxcvmk,./ ~!@#$%^&*()_+QWFPBJLUY:{}ARSTDHNEIO"ZXCVMK<>? "#.as_bytes();
const AZERTY: &[u8] = r#"`1234567890°+azertyuiop^$qsdfghjklmù%<wxcvbn,;:! *µAZERTYUIOP¨£QSDFGHJKLM%>WXCVBN?./§ "#.as_bytes();

pub enum KeyboardType {
    Dvorak,
    Qwerty,
    Azerty,
    Colemark
}

impl Display for KeyboardType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardType::Dvorak => write!(f, "Dvorak"),
            KeyboardType::Qwerty => write!(f, "Qwerty"),
            KeyboardType::Azerty => write!(f, "Azerty"),
            KeyboardType::Colemark => write!(f, "Colemark"),
        }
    }
}

impl KeyboardType {
    pub fn map_key(&self, c: char) -> char {
        if let Some(index) = QWERTY.iter().position(|&x| x == c as u8) {
            match self {
                KeyboardType::Dvorak => DVORAK[index] as char,
                KeyboardType::Qwerty => c,
                KeyboardType::Azerty => AZERTY[index] as char,
                KeyboardType::Colemark => COLEMARK[index] as char
            }
        }
        else {
            c
        }
    }
}