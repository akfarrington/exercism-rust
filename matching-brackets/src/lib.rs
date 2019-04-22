const VALID: [char; 6] = ['(', ')', '{', '}', '[', ']'];

#[derive(Debug)]
struct Sent {
    text: String,
    sanitized: String,
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let test = Sent::from(string);

    test.is_b()
}

impl Sent {
    pub fn is_b(&self) -> bool {
        let mut brackets: Vec<char> = Vec::new();

        for c in self.sanitized.chars() {
            match c {
                ')' | '}' | ']' => {
                    if brackets.pop() != Some(c) {
                        return false;
                    }
                },
                '(' => brackets.push(')'),
                '{' => brackets.push('}'),
                '[' => brackets.push(']'),
                _ => (),
            }
        }
        
        brackets.is_empty()
    }
}

impl<'a> From<&'a str> for Sent {
    fn from(s: &str) -> Sent {
        let sanitized = s.chars().filter(|c| VALID.contains(c)).collect::<String<>>();
        Sent{
            text: s.to_string(),
            sanitized: sanitized,
        }
    }
}