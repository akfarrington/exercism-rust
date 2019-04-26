const LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.to_lowercase()
        .chars()
        .filter_map(switch_letter)
        .collect::<Vec<_>>()
        .chunks(5)
        .collect::<Vec<_>>()
        .join(&' ')
        .iter()
        .collect::<String<>>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(switch_letter).collect::<String>()
}

fn switch_letter(letter: char) -> Option<char> {
    match letter {
        '0'...'9' => Some(letter),
        'a'...'z' => Some(LETTERS[25 - LETTERS.iter().position(|&r| r == letter).unwrap()]),
        _ => None,
    }
}
