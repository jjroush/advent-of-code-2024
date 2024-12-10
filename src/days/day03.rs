use regex::Regex;

pub fn run() {
    let input = include_str!("../../input/day03-test.txt");

    solve_part2(input);
}


fn solve_part1(input: &str) -> i32 {

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let result: i32 = re.captures_iter(input).map(|cap| {
        let nums: Vec<i32> = cap[0]
            .trim_start_matches("mul(")
            .trim_end_matches(")")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        nums[0] * nums[1]
    }).sum();

    result
}

fn solve_part2(input: &str) -> i32 {

    let re_remove = Regex::new(r"don't\([^\)]*\).*?do\([^\)]*\)").unwrap();

    let mut modified_input = re_remove.replace_all(input, "").to_string();

    // Iteratively remove all instances of the pattern
    while re_remove.is_match(&modified_input) {
        modified_input = re_remove.replace_all(&modified_input, "").to_string();
    }

    println!("{}", modified_input);

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let result: i32 = re.captures_iter(&*modified_input).map(|cap| {
        let nums: Vec<i32> = cap[0]
            .trim_start_matches("mul(")
            .trim_end_matches(")")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        nums[0] * nums[1]
    }).sum();

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../../input/day03-test.txt");
        assert_eq!(solve_part1(input), 161);
    }

    #[test]
    fn test_part2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve_part2(input), 48);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day03-test.txt");
        println!("{}", (solve_part1(input)))
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day03-test.txt");
        println!("{}", (solve_part2(input)))
    }



    // #[test]
    // fn test_part2_example() {
    //     let input = "example input here";
    //     assert_eq!(solve_part2(input), 84);
    // }
}

