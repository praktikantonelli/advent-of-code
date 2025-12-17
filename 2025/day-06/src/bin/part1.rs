fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-06.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> u64 {
    // parse &str into Vec<Vec<&str>> <=> each inner Vec<&str> is a row
    let rows = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&row| row.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    // rows[i] is a list of numeric strings EXCEPT if i = rows.len() - 1, in that case it's a list
    // of "+" and "*"
    dbg!(&rows);
    // To do math, trnapose so each row contains some numbers and a final operator
    let rows = transpose(rows);
    // Now, rows[i] has some numeric strings and the last element is either "+" or "*"
    dbg!(&rows);
    let mut result: u64 = 0;
    for row in rows {
        dbg!(&row);
        let numbers = row
            .clone()
            .into_iter()
            .filter_map(|e| e.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        dbg!(&numbers);
        let sign = row.last().unwrap();
        if *sign == "+" {
            println!("Plus sign");
            let intermediate = numbers.iter().sum::<u64>();
            dbg!(intermediate);
            result += intermediate;
        } else {
            println!("Multiply sign");
            let intermediate = numbers.iter().fold(1, |acc, x| acc * x);
            dbg!(intermediate);
            result += intermediate;
        }
    }

    result
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    // Convert a 2D vector of shape (n, m) to shape (m, n) by swapping elements [i,j] with [j,i]
    // https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let output = part1(input);
        assert_eq!(output, 4277556);
    }
}
