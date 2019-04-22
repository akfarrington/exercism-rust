const STATEMENT: &str = "Whatever.";
const SILENCE: &str = "Fine. Be that way!";
const SHOUTING: &str = "Whoa, chill out!";
const QUESTION: &str = "Sure.";
const QUESTION_SHOUT: &str = "Calm down, I know what I'm doing!";

pub fn reply(message: &str) -> &str {
    let trim_message: &str = message.trim();

    match trim_message.is_empty() {
        true => SILENCE,
        false => {
            let is_question: bool = trim_message.ends_with('?');
            let is_shout: bool = trim_message == trim_message.to_uppercase()
                && trim_message.chars().any(|letter| letter.is_alphabetic());

            match (is_question, is_shout) {
                (true, true) => QUESTION_SHOUT,
                (false, true) => SHOUTING,
                (true, false) => QUESTION,
                _ => STATEMENT,
            }
        }
    }
}
