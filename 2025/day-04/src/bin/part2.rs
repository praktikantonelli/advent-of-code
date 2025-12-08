use std::fmt;
fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-04.txt");
    let output = part2(input);
    println!("{output}");
}

#[derive(Debug)]
struct Grid {
    characters: Vec<Vec<char>>,
    neighbor_count: Vec<Vec<u32>>,
    accessible_toiletpaper: Vec<Vec<u32>>,
    n_rows: usize,
    n_columns: usize,
    n_accessible_rolls: u32,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut characters = Vec::new();
        for line in input.lines() {
            characters.push(line.chars().collect::<Vec<char>>());
        }
        let n_rows = characters.len();
        let n_columns = characters[0].len();

        let neighbor_count = vec![vec![0; n_columns]; n_rows];
        let accessible_toiletpaper = vec![vec![0; n_columns]; n_rows];

        let n_accessible_rolls = 0;

        Self {
            characters,
            neighbor_count,
            accessible_toiletpaper,
            n_rows,
            n_columns,
            n_accessible_rolls,
        }
    }

    fn remove_accessible_rolls(&mut self) -> u32 {
        let mut n_removed_rolls = 0;
        for (row_id, row) in self.accessible_toiletpaper.iter().enumerate() {
            for (column_id, column) in row.iter().enumerate() {
                if self.accessible_toiletpaper[row_id][column_id] == 1 {
                    self.characters[row_id][column_id] = '.';
                    n_removed_rolls += 1;
                }
            }
        }
        self.n_accessible_rolls += n_removed_rolls;
        println!("Removed {n_removed_rolls} rolls");
        n_removed_rolls // once this reaches zero, stop calling this method
    }

    fn compute_neighbor_count(&mut self) {
        for row in 0..self.n_rows {
            for column in 0..self.n_columns {
                let character = self.characters[row][column];
                // if char at row i, column j is "@", increase count at [i-1..=i+1][j-1..=j+1] by 1
                // caveat: need to make sure 0 < {i, j} < {n_rows, n_columns} to avoid out of bounds error
                if character != '@' {
                    continue;
                }
                let row_min = row.saturating_sub(1);
                let row_max = (row + 1).min(self.n_rows - 1);
                let col_min = column.saturating_sub(1);
                let col_max = (column + 1).min(self.n_columns - 1);
                for i in row_min..=row_max {
                    for j in col_min..=col_max {
                        if (i != row || j != column) && self.characters[i][j] == '@' {
                            self.neighbor_count[row][column] += 1;
                        }
                    }
                }
            }
        }
    }

    fn find_accessible_toiletpaper(&mut self) {
        let mut count = 0;
        for (row_id, row) in self.neighbor_count.iter().enumerate() {
            for (col_id, column) in row.iter().enumerate() {
                if *column < 4 && self.characters[row_id][col_id] == '@' {
                    self.accessible_toiletpaper[row_id][col_id] = 1;
                    count += 1;
                }
            }
        }
        println!("Found {count} more accessible rolls");
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

        write!(f, "{grid}")
    }
}

fn part2(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    println!("{grid}");
    grid.compute_neighbor_count();
    grid.find_accessible_toiletpaper();
    let mut val = grid.remove_accessible_rolls();
    grid.compute_neighbor_count();
    grid.find_accessible_toiletpaper();
    while val != 0 {
        println!(
            "==========================================================================================================================================="
        );
        val = grid.remove_accessible_rolls();
        grid.compute_neighbor_count();
        grid.find_accessible_toiletpaper();
        println!("{grid}");
    }

    grid.n_accessible_rolls
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
        let output = part2(input);
        assert_eq!(output, 43);
    }
}
