fn compute_pi_table(pattern: &String) -> Vec<u32> {
    let mut pi_table = Vec::new();
    let pattern_arr = pattern.chars();
   
    pi_table.push(0);
    
    let mut j = 0;
    for c in pattern_arr[1..] {
        if c == pattern_arr[j] {
            j += 1;
            pi_table.push(j);
        } else {
            j = pi_table.get(j - 1).unwrap_or(0);
            pi_table.push(0);
        }
    }

    return pi_table;
}

fn kmp(pattern: &String, text: &String) {

}
