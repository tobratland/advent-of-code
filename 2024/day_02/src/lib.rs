use core::num;
use std::vec;

pub fn part_1(input: &str) -> usize {
    let mut total = 0;
    let mut numbers = Vec::new();
    for line in input.lines() {
        let mut line_of_numbers = Vec::new();

        for num_str in line.split_ascii_whitespace().into_iter() {
            let num = num_str.parse::<i32>();
            if num.is_ok() {
                line_of_numbers.push(num.unwrap() as i32);
            }
        }
        numbers.push(line_of_numbers);
    }
    for line in numbers.into_iter() {
        let mut increasing = false;
        let mut decreasing = false;
        let mut safe_line = true;
        let mut previous = 0;

        for n in line {
            //println!("previous {} current {}", previous, n);
            if previous == 0 {
                previous = n;
                continue;
            } else if n == previous + 1 || n == previous + 2 || n == previous + 3 {
                increasing = true;
                previous = n
            } else if n == previous - 1 || n == previous - 2 || n == previous - 3 {
                decreasing = true;
                previous = n;
            } else {
                safe_line = false;
                break;
            }
        }
        if increasing && decreasing {
            safe_line = false
        }

        if safe_line {
            total += 1
        }
        //println!("is safe {}", safe_line)
    }

    total
}

pub fn part_2(input: &str) -> usize {
    let mut total = 0;
    let mut numbers = Vec::new();
    for line in input.lines() {
        let mut line_of_numbers = Vec::new();

        for num_str in line.split_ascii_whitespace().into_iter() {
            let num = num_str.parse::<i32>();
            if num.is_ok() {
                line_of_numbers.push(num.unwrap() as i32);
            }
        }
        numbers.push(line_of_numbers);
    }
    let mut unsafe_lines = Vec::new();
    for line in numbers.into_iter() {
        

        if safe_line(&line) {
            total += 1
        } else {
            unsafe_lines.push(line);
        }
        //println!("is safe {}", safe_line)
    }

    for unsafe_line in unsafe_lines {
        for i in 0..unsafe_line.len() {
            let mut usl = unsafe_line.clone();
            usl.remove(i);
            println!("potential line {:?}", usl);
            if safe_line(&usl) {
                println!("safe line: {:?}", usl);
                total +=1;
                break;
            }
        }
    }
    total
}

fn safe_line(line: &Vec<i32>) -> bool{
    let mut increasing = false;
    let mut decreasing = false;
    let mut previous = 0;
    for n in line {
        //println!("previous {} current {}", previous, n);
        if previous == 0 {
            previous = *n;
            continue;
        } else if *n == previous + 1 || *n == previous + 2 || *n == previous + 3 {
            increasing = true;
            previous = *n
        } else if *n == previous - 1 || *n == previous - 2 || *n == previous - 3 {
            decreasing = true;
            previous = *n;
        } else {
            return false;
        }
    }
    if increasing && decreasing {
        return false
    }
    return true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_input() {
        let input = include_str!("../test-input.txt");
        let result = part_1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_1_actual_input() {
        let input = include_str!("../input.txt");
        let result = part_1(input);
        assert_eq!(result, 332);
    }

    #[test]
    fn part_2_test_input() {
        let input = include_str!("../test-input.txt");
        let result = part_2(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_2_real_input() {
        let input = include_str!("../input.txt");
        let result = part_2(input);
        assert_eq!(result, 4);
    }

}
