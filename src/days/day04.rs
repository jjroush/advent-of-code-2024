use regex::Regex;
use std::io::{Write};
use std::thread::sleep;
use std::time::Duration;
use termion::{cursor};

pub fn run() {
    let input = include_str!("../../input/day04-test.txt");

    solve_part1(input);
}


fn solve_part1(input: &str) -> i32 {
    
    
    let multidimensional_array: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let rendered_md_array = multidimensional_array.clone();

    let rows = multidimensional_array.len();
    let cols = multidimensional_array[0].len();

    let terms = ["XMAS"];
    let search_len = terms[0].len();


    let directions = [
        ( 1,  0),  // Right
        ( 0,  1),  // Down
        ( 1,  1),  // Diagonal down-right
        ( 1, -1),  // Diagonal down-left
        (-1,  0),  // Left (backwards)
        ( 0, -1),  // Up (backwards)
        (-1, -1),  // Diagonal up-left (backwards)
        (-1,  1),  // Diagonal up-right (backwards)
    ];
    
    let mut christmas_count = 0;

    // print!("{}{}", cursor::Hide, cursor::Goto(1, 1));
    
    // ai was here
    for i in 0..rows {
        for j in 0..cols {
            for &term in &terms {
                if multidimensional_array[i][j] == term.chars().next().unwrap() {
                    'search: for (di, dj) in directions.iter() {
                        let mut found = String::new();
                        for k in 0..search_len {
                            let ni = i as isize + di * k as isize;
                            let nj = j as isize + dj * k as isize;
                            if ni < 0 || ni >= rows as isize || nj < 0 || nj >= cols as isize {
                                continue 'search;
                            }

                            // [i][j]
                            
                            found.push(multidimensional_array[ni as usize][nj as usize]);
                        }
                        if found == term {
                            // print!("\x1B[2J\x1B[1;1H");


                            print!("\x1b[3J");
                            println!("Found '{}' at: ({}, {}), direction: ({}, {}){}", term, i, j, di, dj, "\x1b[K");

                            sleep(Duration::from_millis(100));

                            christmas_count += 1;
                        }
                    }
                }
            }
        }
    }

    print!("{}", cursor::Show);
    christmas_count
}

fn solve_part2(input: &str) -> i32 {

    let relevant_chars: Vec<char> = vec!['M', 'A', 'S'];

    // Process the input to replace irrelevant characters with '.'
    let processed_input: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if relevant_chars.contains(&c) { c } else { '.' })
                .collect()
        })
        .collect();

    // Print the processed multidimensional array to verify
    for row in &processed_input {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
    
    1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../../input/day04-test.txt");
        assert_eq!(solve_part1(input), 18);
    }

    #[test]
    fn test_part2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve_part2(input), 48);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day04.txt");
        println!("{}", (solve_part1(input)))
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day04.txt");

        println!("{}", (solve_part2(input)))
    }



    // #[test]
    // fn test_part2_example() {
    //     let input = "example input here";
    //     assert_eq!(solve_part2(input), 84);
    // }
}

