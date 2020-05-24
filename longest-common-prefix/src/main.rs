pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::from("");
    }
    if strs.len() == 1 {
        return strs[0].clone();
    }

    let matrix: Vec<Vec<char>> = strs.iter().map(|a| a.chars().collect()).collect();
    let len = matrix.iter().map(|a| a.len()).min().unwrap();
    for i in 0..len {
        let val = matrix[0][i];
        for j in 1..matrix.len() {
            if matrix[j][i] != val {
                return matrix[0][0..i].into_iter().collect();
            }
        }
        if i == len - 1 {
            return matrix[0][0..(i + 1)].into_iter().collect();
        }
    }

    String::from("")
}

fn main() {
    // assert_eq!(
    //     longest_common_prefix(vec![
    //         String::from("flower"),
    //         String::from("flow"),
    //         String::from("flight")
    //     ]),
    //     String::from("fl")
    // );
    assert_eq!(
        longest_common_prefix(vec![String::from("c",), String::from("c")]),
        String::from("c")
    );
}
