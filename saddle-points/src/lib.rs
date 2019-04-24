pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();
    let mut max_coords: Vec<(usize, usize)> = Vec::new();
    let mut min_coords: Vec<(usize, usize)> = Vec::new();

    if input.is_empty() || input[0].is_empty() {
        return saddle_points;
    };

    // find the coordinates for the maxs in each row
    input.iter().enumerate().for_each(|(y, row)| {
        // find the max value
        let (max, _) = row
            .iter()
            .enumerate()
            .map(|(index, number)| (number, index))
            .max()
            .unwrap();

        // find the coordinates that match the max (in case of duplicates)
        row.iter()
            .enumerate()
            .filter(|(_, number)| number == &max)
            .for_each(|(x, _)| max_coords.push((y, x)));
    });

    // find the coordinates for the mins in each column
    (0..input[0].len()).for_each(|x| {
        // find the min value
        let (min, _) = (0..input.len())
            .map(|index_y| (&input[index_y][x], index_y))
            .min()
            .unwrap();

        // find the coordinates that match the min (in case of duplicates)
        (0..input.len())
            .filter(|number| input[*number][x].eq(min))
            .for_each(|number| min_coords.push((number, x)));
    });

    for item in max_coords {
        if min_coords.contains(&item) {
            saddle_points.push(item);
        }
    }
    saddle_points
}
