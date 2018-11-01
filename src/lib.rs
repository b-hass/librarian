use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pi_table_computation() {
        let mut pattern = "abcdabca";
        assert_eq!(
            vec!(0, 0, 0, 0, 1, 2, 3, 1),
            compute_pi_table(&pattern)
        );

        pattern = "ababd";
        assert_eq!(
            vec!(0, 0, 1, 2, 0),
            compute_pi_table(&pattern)
        )
    }
    
    #[test]
    fn kmp_computation() {
        let text = 
"To see a World in a Grain of Sand. 
And a Heaven in a Wild Flower. 
Hold Infinity in the palm of your hand. And Eternity in an hour. ";
        assert_eq!(
            vec!("And a Heaven in a Wild Flower."),
            kmp("Heaven", text)
        );
    }

}

fn compute_pi_table(pattern: &str) -> Vec<usize> {
    let mut pi_table: Vec<usize> = Vec::new();
    let pattern_arr = pattern.as_bytes();
   
    pi_table.push(0);
    
    let mut j: usize = 0;
    for &c in pattern_arr[1..].iter() {
        
        if c == pattern_arr[j] {
            j += 1;
            pi_table.push(j);
        } else {
            if j == 0 {
                j = pi_table[0] as usize;
            } else {
                j = pi_table[j - 1] as usize;
            }
            
            if c == pattern_arr[j] {
                j += 1;
            }
            pi_table.push(j);
            
        }
    }
    return pi_table;
}

pub fn kmp<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    let pi_table = compute_pi_table(pattern);
    let mut pi_table_index: usize = 0;
    let pattern_bytes = (*pattern).as_bytes();

    let mut result = Vec::new();
    
    for line in text.lines() {
        for &c in line.as_bytes() {
            if c == pattern_bytes[pi_table_index] as u8 {
                pi_table_index += 1;
                if pi_table_index == pi_table.len() {
                    result.push(line);
                }
            } else {
                if pi_table_index == 0 {
                    pi_table_index = pi_table[0];
                } else {
                    pi_table_index = pi_table[pi_table_index - 1];
                }
            }
        }
    }

    return result;
}

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("FILE_NOT_FOUND")
}