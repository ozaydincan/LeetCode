use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn remove_subfolders(folders: Vec<String>) -> Vec<String> {
        let folder_set: HashSet<String> = folders.iter().cloned().collect();
        let mut result = Vec::new();

        for folder in folders {
            let parts: Vec<&str> = folder.split('/').filter(|s| !s.is_empty()).collect();
            let mut current_path = String::new();
            let mut is_subfolder = false;

            for i in 0..parts.len() - 1 {
                current_path.push('/');
                current_path.push_str(parts[i]);

                if folder_set.contains(&current_path) {
                    is_subfolder = true;
                    break;
                }
            }

            if !is_subfolder {
                result.push(folder);
            }
        }

        result
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let folder: Vec<String> = vec![
            "/a".to_string(),
            "/a/b".to_string(),
            "/c/d".to_string(),
            "/c/d/e".to_string(),
            "/c/f".to_string(),
        ];
        assert_eq!(
            Solution::remove_subfolders(folder),
            vec!["/a", "/c/d", "/c/f"]
        )
    }
}
