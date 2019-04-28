pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows_vec: Vec<Vec<u32>> = Vec::new();

        if row_count == 0 {
            return PascalsTriangle { triangle: rows_vec };
        }

        (1..=row_count).for_each(|row| {
            let complete_row = (1..=row)
                .map(|column| {
                    if column == 1 || column == row {
                        1
                    } else {
                        rows_vec[row as usize - 2 as usize][column as usize - 1 as usize]
                            + rows_vec[row as usize - 2 as usize][column as usize - 2 as usize]
                    }
                })
                .collect::<Vec<u32>>();
            rows_vec.push(complete_row);
        });
        Self { triangle: rows_vec }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.to_vec()
    }
}
