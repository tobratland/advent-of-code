pub fn part_1(input: &str) -> usize {
    let mut res = 0;
    let mut vec = Vec::new();
    for line in input.lines() {
        let mut tmp_vec = Vec::new();
        line.chars().for_each(|c| tmp_vec.push(c.to_string()));
        vec.push(tmp_vec);
    }

    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            //println!("{}", get_num_of_words_with_start((i, j), vec.clone(), "xmas".to_string()))
            res += get_num_of_words_with_start((i, j), vec.clone(), "XMAS".to_string());
        }
    }

    res
}

pub fn part_2(input: &str) -> usize {
    let mut res = 0;
    let mut vec = Vec::new();
    for line in input.lines() {
        let mut tmp_vec = Vec::new();
        line.chars().for_each(|c| tmp_vec.push(c.to_string()));
        vec.push(tmp_vec);
    }

    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            //println!("{}", get_num_of_words_with_start((i, j), vec.clone(), "xmas".to_string()))
            if vec[i][j] == "A" {
                if is_mas((i, j), vec.clone()) {
                    res += 1;
                }
            }
        }
    }

    res
}

fn is_mas((s1, s2): (usize, usize), vec: Vec<Vec<String>>) -> bool {

    if s1 < 1 || s2 < 1 || s1 > vec.len() - 2 || s2 > vec[s1].len() - 2 {
        return false
    }
    //println!("{}, {}", vec[s1-1][s2+1].clone() + &vec[s1+1][s2-1], vec[s1-1][s2-1].clone() + &vec[s1+1][s2+1]);

    if ((vec[s1-1][s2-1] == "M" && vec[s1+1][s2+1] == "S") || (vec[s1-1][s2-1] == "S" && vec[s1+1][s2+1] == "M")) 
        && ((vec[s1+1][s2-1] == "M" && vec[s1-1][s2+1] == "S") || (vec[s1+1][s2-1] == "S" && vec[s1-1][s2+1] == "M")) {
            return true
        }

    false
}


fn get_num_of_words_with_start((s1, s2): (usize, usize), vec: Vec<Vec<String>>, word: String) -> usize {
    let mut total = 0;
    if s1 > 2
        && (vec[s1][s2].clone() + &vec[s1 - 1][s2] + &vec[s1 - 2][s2] + &vec[s1 - 3][s2] == word)
    {
        //println!("{}", vec[s1][s2].clone() + &vec[s1 - 1][s2] + &vec[s1 - 2][s2] + &vec[s1 - 3][s2]);
        total += 1;
    }
    if s1 < vec.len() - 3 && (vec[s1][s2].clone() + &vec[s1 + 1][s2] + &vec[s1 + 2][s2] + &vec[s1 + 3][s2]) == word
    {
        total += 1;
    }
    if s2 > 2 && (vec[s1][s2].clone() + &vec[s1][s2 - 1] + &vec[s1][s2 - 2] + &vec[s1][s2 - 3] == word) {
        total += 1
    }
    if s2 < vec[s1].len() - 3 && (vec[s1][s2].clone() + &vec[s1][s2 + 1] + &vec[s1][s2 + 2] + &vec[s1][s2 + 3]) == word
    {
        total += 1;
    }

    

    if s1 > 2 && s2 > 2 && (vec[s1][s2].clone() + &vec[s1 - 1][s2 - 1] + &vec[s1 - 2][s2 - 2] + &vec[s1 - 3][s2 - 3] == word) {
        total += 1;
    }

    if s1 > 2 && s2 < vec[s1].len() - 3 && (vec[s1][s2].clone() + &vec[s1 - 1][s2 + 1] + &vec[s1 - 2][s2 + 2] + &vec[s1 - 3][s2 + 3] == word) {
        total += 1;
    }

    if s1 < vec.len() - 3 && s2 < vec[s1].len() - 3 && (vec[s1][s2].clone() + &vec[s1 + 1][s2 + 1] + &vec[s1 + 2][s2 + 2] + &vec[s1 + 3][s2 + 3] == word) {
        total += 1;
    }

    if s1 < vec.len() - 3 && s2 > 2 && (vec[s1][s2].clone() + &vec[s1 + 1][s2 - 1] + &vec[s1 + 2][s2 - 2] + &vec[s1 + 3][s2 - 3] == word) {
        total += 1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_input() {
        let result = part_1(include_str!("../test-input.txt"));
        assert_eq!(result, 18);
    }
    #[ignore]
    #[test]
    fn part_1_real_input() {
        let result = part_1(include_str!("../input.txt"));
        assert_eq!(result, 2613);
    }

    #[test]
    fn part_2_test_input() {
        let result = part_2(include_str!("../test-input.txt"));
        assert_eq!(result, 9);
    }

    #[test]
    fn part_2_real_input() {
        let result = part_2(include_str!("../input.txt"));
        assert_eq!(result, 9);
    }
}
