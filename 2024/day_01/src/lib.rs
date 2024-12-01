use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    let mut distance = 0;
    let (mut list1, mut list2) = (Vec::new(), Vec::new());
    //println!("complete input {}", input);
    let split_lines: Vec<&str> = input.split('\n').collect();
    for line in split_lines {
        let (f, s) = line.split_once(' ').unwrap();

        let (f, s) = (no_space(f), no_space(s));
        //println!("total distance before: {}", distance);
        let (n1, n2) = (f.parse::<usize>().unwrap(), s.parse::<usize>().unwrap());
        list1.push(n1);
        list2.push(n2);
        /* let distance_this_round = f.parse::<usize>().unwrap().abs_diff(s.parse::<usize>().unwrap());
        println!("distance between 1:{} and 2:{} is {}", n1, n2, distance_this_round);
        distance = distance + distance_this_round;
        println!("total distance after: {}", distance); */
    }
    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        let c_dist = list1[i].abs_diff(list2[i]);
        //println!("distance between {} and {} is {}", list1[i], list2[i], c_dist);
        distance = distance + c_dist;
    }

    distance
}

pub fn part_2(input: &str) -> usize {
    let mut res = 0;
    let (mut list1, mut list2) = (Vec::new(), Vec::new());
    //println!("complete input {}", input);
    let split_lines: Vec<&str> = input.split('\n').collect();
    for line in split_lines {
        let (f, s) = line.split_once(' ').unwrap();

        let (f, s) = (no_space(f), no_space(s));
        //println!("total distance before: {}", distance);
        let (n1, n2) = (f.parse::<usize>().unwrap(), s.parse::<usize>().unwrap());
        list1.push(n1);
        list2.push(n2);
    }

    let mut count_map = HashMap::new();
    for n2 in list2.iter() {
        *count_map.entry(n2).or_insert(0) += 1;
    }

    for n1 in list1.iter() {
        if let Some(&same) = count_map.get(n1) {
            println!("number of {} in list 2 is {}", n1, same);
            res += n1 * same;
        }
    }

    res
}

fn no_space(x: &str) -> String {
    x.replace(" ", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_input() {
        let input = include_str!("../test-input.txt");
        let result = part_1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn part_1_actual_input() {
        let input = include_str!("../input.txt");
        let result = part_1(input);
        assert_eq!(result, 2164381);
    }

    #[test]
    fn part_2_test_input() {
        let input = include_str!("../test-input.txt");
        let result = part_2(input);
        assert_eq!(result, 31);
    }

    #[test]
    fn part_2_real_input() {
        let input = include_str!("../input.txt");
        let result = part_2(input);
        assert_eq!(result, 20719933);
    }
}
