pub fn verse(n: i32) -> String {
    match n {
        0 => {
            return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
        }
        1 => {
            return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
        }
        2 => {
            return "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string();
        }
        _ => {
            return n.to_string() + " bottles of beer on the wall, " + &n.to_string() + " bottles of beer.\nTake one down and pass it around, " + &(n-1).to_string() + " bottles of beer on the wall.\n";
        }
    }
}


// thanks https://exercism.io/tracks/rust/exercises/beer-song/solutions/69e3124c1c67479285b2045c64d4450f
pub fn sing(start: i32, end: i32) -> String {
    (end..=start).rev().map(|i| verse(i)).collect::<Vec<_>>().join("\n")
}

