fn compute_pi_table(pattern: &String) -> Vec<u32> {
    let mut pi_table: Vec<u32> = Vec::new();
    let pattern_arr = pattern.as_bytes();
   
    pi_table.push(0);
    
    let mut j: usize = 0;
    for &c in pattern_arr[1..].iter() {
        
        if c == pattern_arr[j] {
            j += 1;
            pi_table.push(j as u32);
        } else {
            if j == 0 {
                j = pi_table[0] as usize;
            } else {
                j = pi_table[j - 1] as usize;
            }
            
            if c == pattern_arr[j] {
                j += 1;
            }
            pi_table.push(j as u32);
            
        }
    }

    return pi_table;
}

fn kmp(pattern: &String, text: &String) {

}
