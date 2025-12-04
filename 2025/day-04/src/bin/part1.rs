use std::fmt;
fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-04.txt");
    let output = part1(input);
    println!("{output}");
}

#[derive(Debug)]
struct Grid {
    characters: Vec<Vec<char>>,
    neighbor_count: Vec<Vec<u32>>,
    accessible_toiletpaper: Vec<Vec<u32>>,
    n_rows: usize,
    n_columns: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut characters = Vec::new();
        for line in input.lines() {
            characters.push(line.chars().collect::<Vec<char>>());
        }
        let n_rows = characters.len();
        let n_columns = characters[0].len();

        let mut neighbor_count = vec![vec![0; n_columns]; n_rows];
        let mut accessible_toiletpaper = vec![vec![0; n_columns]; n_rows];

        for row in 0..n_rows {
            for column in 0..n_columns {
                let character = characters[row][column];
                // if char at row i, column j is "@", increase count at [i-1..=i+1][j-1..=j+1] by 1
                // caveat: need to make sure 0 < {i, j} < {n_rows, n_columns} to avoid out of bounds error
                if character != '@' {
                    continue;
                }
                let row_min = row.saturating_sub(1);
                let row_max = (row + 1).min(n_rows - 1);
                let col_min = column.saturating_sub(1);
                let col_max = (column + 1).min(n_columns - 1);
                for i in row_min..=row_max {
                    for j in col_min..=col_max {
                        if i != row && j != column && characters[i][j] == '@' {
                            neighbor_count[i][j] += 1;
                        }
                    }
                }
            }
        }

        for (row_id, row) in neighbor_count.iter().enumerate() {
            for (col_id, column) in row.iter().enumerate() {
                if *column < 4 && characters[row_id][col_id] == '@' {
                    accessible_toiletpaper[row_id][col_id] = 1;
                }
            }
        }

        Self {
            characters,
            neighbor_count,
            accessible_toiletpaper,
            n_rows,
            n_columns,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows: Vec<String> = Vec::new();
        for row in &self.characters {
            let string = row.iter().collect::<String>();
            rows.push(string);
        }
        let grid = rows.join("\n");

        let mut rows: Vec<String> = Vec::new();
        for row in &self.neighbor_count {
            let string = row.iter().map(|x| format!("{x}")).collect();
            rows.push(string);
        }
        let neighbour_count = rows.join("\n");

        let mut rows: Vec<String> = Vec::new();
        for row in &self.accessible_toiletpaper {
            let string = row.iter().map(|x| format!("{x}")).collect();
            rows.push(string);
        }
        let accessible_toiletpaper = rows.join("\n");

        write!(
            f,
            "Grid: \n{grid}\n\nNeighbour Count: \n{neighbour_count}\n\nAccessible Toilet Paper: \n{accessible_toiletpaper}"
        )
    }
}

fn part1(input: &str) -> u32 {
    let grid = Grid::new(input);
    println!("{grid}");

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let output = part1(input);
        assert_eq!(output, 13);
    }
}
