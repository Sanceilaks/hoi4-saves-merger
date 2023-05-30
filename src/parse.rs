pub enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    pub fn from_char(c: char) -> Option<Bracket> {
        match c {
            '{' => Some(Bracket::Open(c)),
            '}' => Some(Bracket::Close('{')),
            _ => None,
        }
    }
}

pub fn find_closing(str: &str, start_pos: Option<usize>) -> Option<usize> {
    let start_pos = start_pos.unwrap_or(0);
    let mut brackets: Vec<char> = vec![];

    let slice = &str[start_pos..];
    let mut is_first_char = true;

    for (i, c) in slice.chars().enumerate() {
        match Bracket::from_char(c) {
            Some(Bracket::Open(char_bracket)) => {
                brackets.push(char_bracket);
                is_first_char = false;
            }
            Some(Bracket::Close(char_close_bracket)) => {
                if brackets.pop() != Some(char_close_bracket) {
                    return None;
                }
            }
            _ => {}
        }

        if brackets.len() == 0 && !is_first_char {
            return Some(start_pos + i + 1);
        }
    }
    None
}
