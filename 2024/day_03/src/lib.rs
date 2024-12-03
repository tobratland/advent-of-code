use regex::{Regex, RegexSetBuilder};

pub fn part_1(input: &str) -> usize {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    
    // Collect all matches into a vector
    let sum: i32 = re.find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .map(|p| {
            let (n1, n2) = p.split_once("(").unwrap();
            let n2 = n2.replace(")","");
            let n2 = n2.split_once(",").unwrap();
            println!("{:?}", n2);
            let (n1, n2) = (n2.0.parse::<i32>().unwrap(), n2.1.parse::<i32>().unwrap());
            //n1*n2
            n1*n2
        }).sum();

    sum as usize
}

pub fn part_2(input: &str) -> usize {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let mut should_do = true;
    // Collect all matches into a vector
    let sum: i32 = re.find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .map(|p| {
            if p.contains("do()") {
                should_do = true;
                return 0
            }
            if p.contains("don't()") {
                should_do = false;
                return 0
            }
            let (n1, n2) = p.split_once("(").unwrap();
            let n2 = n2.replace(")","");
            let n2 = n2.split_once(",").unwrap();
            println!("{:?}", n2);
            let (n1, n2) = (n2.0.parse::<i32>().unwrap(), n2.1.parse::<i32>().unwrap());
            //n1*n2
            if should_do {
                n1*n2
            } else {
                0
            }
            
        }).sum();

    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn part_1_test_input() {
        let result = part_1(include_str!("../test-input.txt"));
        assert_eq!(result, 161);
    }
    #[ignore]
    #[test]
    fn part_1_real_input() {
        let result = part_1(include_str!("../input.txt"));
        assert_eq!(result, 163931492);
    }

    #[test]
    fn part_2_test_input() {
        let result = part_2(include_str!("../test-input2.txt"));
        assert_eq!(result, 48);
    }

    #[test]
    fn part_2_real_input() {
        let result = part_2(include_str!("../input.txt"));
        assert_eq!(result, 76911921);
    }
}
