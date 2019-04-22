// allow dead code because it thinks I never use this struct
// probably is useless, but it works.
#[allow(dead_code)]
struct Raindrop {
    number: u32,
    pling: bool,// 3
    plang: bool,// 5
    plong: bool,// 7
    printed: String,
}

pub fn raindrops(n: u32) -> String {
    make_raindrop(n).printed
}

fn make_raindrop(n: u32) -> Raindrop {
    let pling: bool = n % 3 == 0;
    let plang: bool = n % 5 == 0;
    let plong: bool = n % 7 == 0;

    let mut printed: String = "".to_string();

    if pling == true {
        printed += &"Pling".to_string();
    }
    if plang == true {
        printed += &"Plang".to_string();
    }
    if plong == true {
        printed += &"Plong".to_string();
    }

    if pling == false && plang == false && plong == false {
        printed = n.to_string();
    }

    Raindrop {
        number: n,
        pling: pling,
        plang: plang,
        plong: plong,
        printed: printed,
    }
}