use minesweeper::*;

fn main() {
    run_test(&["1*2*1"]);
    run_test(&[" 2*2 ", "25*52", "*****", "25*52", " 2*2 "]);
    run_test(&["1*22*1", "12*322", " 123*2", "112*4*", "1*22*2", "111111"]);
}

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

fn run_test(test_case: &[&str]) {
    for row in test_case {
        println!("{:?}", row);
    }
    println!("\n");
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    // annotate(&cleaned_strs);
    let answer = annotate(&cleaned_strs);
    for row in answer {
        println!("{:?}", row);
    }
    println!("\n\n\n");
}
