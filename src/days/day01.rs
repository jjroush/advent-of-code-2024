use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run() {
    let path = Path::new("src/lists.txt");

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file: {}", error);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();



    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(val1), Ok(val2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        list1.push(val1);
                        list2.push(val2);
                    } else {
                        eprintln!("Error parsing line: {}", line);
                    }
                } else {
                    eprintln!("Unexpected line format: {}", line);
                }
            },
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }

    list1.sort();
    list2.sort();

    let mut list1_counts = std::collections::HashMap::new();
    for &num in &list1 {
        *list1_counts.entry(num).or_insert(0) += 1;
    }

    let mut list2_counts = std::collections::HashMap::new();
    for &num in &list2 {
        *list2_counts.entry(num).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for (&num, &count1) in &list1_counts {
        if let Some(&count2) = list2_counts.get(&num) {
            similarity_score += num * count1 * count2;
        }
    }
    println!("Similarity score: {}", similarity_score);

    let mut total_difference = 0;

    while !list1.is_empty() {
        let val1 = list1.remove(0);
        let val2 = list2.remove(0);

        let difference = (val1 - val2).abs();
        total_difference += difference;
    }

    println!("Similarity score: {}", similarity_score);
    println!("Total difference: {}", total_difference);
}
