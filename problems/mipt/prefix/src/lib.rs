#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    
    let mut min_count = std::usize::MAX;
    
    for s in &strs {
        if s.is_empty() {
            return String::new();
        }
        
        let c = s.chars().count();
        if c < min_count {
            min_count = c;
        }
    }
    
    loop {
        let set = strs
            .iter()
            .map(|s| s.chars().take(min_count).collect::<String>())
            .collect::<std::collections::HashSet<_>>();
        
        if set.len() == 1 {
            return set.into_iter().next().unwrap();
        } else {
            min_count -= 1;
            if min_count == 0 {
                return String::new();
            }
        }
    }
}