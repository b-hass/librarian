use std::fs;
use std::error::Error;

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
            search_kmp("Heaven", text)
        );
    }

    #[test]
    fn directory_listing() {
       // let path = env::args[0];
    }

}

pub struct Config {
    root_path: String,
    pattern: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let root_path = args[1].clone();
        let pattern = args[2].clone();
        
        Ok(Config { pattern, root_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let files = list_files(&config.root_path);
    for file in files {
        let content = read_file(&file);
        let matched_lines = search_kmp(&config.pattern, &content);
        println!("{:?}", file);
        println!("{:?}", matched_lines);
    }

    Ok(())
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

fn kmp<'a>(pattern: &str, pi_table: &Vec<usize>, text: &'a str) -> bool {
    let mut pi_table_index: usize = 0;
    let patterns_bytes = (*pattern).as_bytes();
    
    for &c in text.as_bytes() {
        if c == patterns_bytes[pi_table_index] as u8 {
            pi_table_index += 1;
            if pi_table_index >= pi_table.len() - 1 {
                return true;
            }
        } else {
            if pi_table_index == 0 {
                pi_table_index = pi_table[0];
            } else {
                pi_table_index = pi_table[pi_table_index - 1];
            }
        }
    }
    
    return false;
}

fn search_kmp<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    let pi_table = compute_pi_table(pattern);

    text.lines()
        .filter(|line| kmp(pattern, &pi_table, line))
        .collect()
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or(String::new())
}

fn list_files(root: &str) -> Vec<String> {
    let mut results = Vec::new();
    let root_entries = fs::read_dir(root).expect("Directory not found");
    let paths: Vec<std::path::PathBuf> = root_entries.filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .collect();

    for path in paths {
        if path.is_dir() {
            results.append(&mut list_files(path.to_str().unwrap()));
        } else {
            results.push(String::from(fs::canonicalize(path).unwrap().to_str().unwrap()));
        }
    }
    
    results    
}