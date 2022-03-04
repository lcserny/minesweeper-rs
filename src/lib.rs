pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(minefield.len());

    for (row_i, row_val) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for (col_i, col_val) in row_val.as_bytes().iter().enumerate() {
            if *col_val == b' ' {
                new_row.push(' ');
            }

            if *col_val == b'*' {
                new_row.push('*');

                // TODO: update around mine
                    // if cell around has number, increse it
                    // if cell is empty add number 1
                    // if cell is mine, leave it be

                if row_i >= 1 {
                    // we can check above
                }
                if row_i == minefield.len() - 1 {
                    // we can check below
                }
                if col_i >= 1 {
                    // we can check left
                }
                if col_i == row_val.len() - 1 {
                    // we can check right
                }
                if row_i >= 1 && col_i >= 1 {
                    // we can check up-left
                }
                if row_i >= 1 && col_i == row_val.len() - 1 {
                    // we can check up-right
                }
                if row_i == minefield.len() - 1 && col_i >= 1 {
                    // we can check below-left
                }
                if row_i == minefield.len() - 1 && col_i == row_val.len() - 1 {
                    // we can check below-right
                }
            }
        }
        result.push(new_row);
    }

    result
}
