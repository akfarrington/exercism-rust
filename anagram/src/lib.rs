use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // hashset for return
    let mut anagramlist: HashSet<&'a str> = HashSet::new();

    //lowercase word
    let lowercaseword = word.to_lowercase();

    // sort letters in word
    let mut sortedword = lowercaseword.chars().collect::<Vec<char>>();
    sortedword.sort();

    // check possible_anagrams
    for possible in possible_anagrams {
        // lowercase possible word
        let lowercasepossible = possible.to_lowercase();

        // sort possible anagram
        let mut sortedpossible = lowercasepossible.chars().collect::<Vec<char>>();
        sortedpossible.sort();

        // check if equal, and if so add to hashset unless it's equal to original word
        if sortedword == sortedpossible && lowercasepossible != lowercaseword {
            anagramlist.insert(possible);
        }
    }

    anagramlist
}
