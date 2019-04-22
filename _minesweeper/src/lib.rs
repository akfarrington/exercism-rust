#[derive(Debug, PartialEq, PartialOrd)]
enum Square {
    Bomb,
    Flag(u8),
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut vec_minefield: Vec<Vec<Square>> = Vec::new();
    let mut return_minefield: Vec<String> = Vec::new();

    if minefield.is_empty() {
        return return_minefield;
    }

    // change each character to the Square enum
    for row in minefield {
        vec_minefield.push(
            row.chars()
                .map(|item| match item {
                    '*' => Square::Bomb,
                    _ => Square::Flag(0),
                })
                .collect::<Vec<Square>>(),
        );
    }

    let mut x = 0;
    let mut y = 0;

    // should be in the format vec_minefield[y][x]
    while x < vec_minefield[0].len() {
        while y < vec_minefield.len() {
            if vec_minefield[y][x] == Square::Bomb {
                let canup: bool = y > 0;
                let candown: bool = y < vec_minefield.len() - 1;
                let canleft: bool = x > 0;
                let canright: bool = x < vec_minefield[0].len() - 1;

                if canup && canleft {
                    vec_minefield[y - 1][x - 1].add_one();
                }
                if canup {
                    vec_minefield[y - 1][x].add_one();
                }
                if canup && canright {
                    vec_minefield[y - 1][x + 1].add_one();
                }
                if canleft {
                    vec_minefield[y][x - 1].add_one();
                }
                if canright {
                    vec_minefield[y][x + 1].add_one();
                }
                if candown && canleft {
                    vec_minefield[y + 1][x - 1].add_one();
                }
                if candown {
                    vec_minefield[y + 1][x].add_one();
                }
                if candown && canright {
                    vec_minefield[y + 1][x + 1].add_one();
                }
            }
            y += 1;
        }
        y = 0;
        x += 1;
    }

    // change to returnable value
    for row in vec_minefield {
        return_minefield.push(
            row.iter()
                .map(|square| match square {
                    Square::Bomb => "*".to_string(),
                    Square::Flag(0) => " ".to_string(),
                    Square::Flag(i) => i.to_string(),
                })
                .collect::<String>(),
        );
    }

    return_minefield
}

impl Square {
    pub fn add_one(&mut self) {
        *self = match self {
            Square::Bomb => Square::Bomb,
            Square::Flag(i) => Square::Flag(*i + 1),
        };
    }
}
