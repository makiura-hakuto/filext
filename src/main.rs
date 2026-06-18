use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn count_extensions(path: &str) -> io::Result<HashMap<String, usize>> {
    let mut counts = HashMap::new();

    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }
        
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_string();
        
            *counts.entry(ext).or_insert(0) += 1;
        }
    }

    Ok(counts)
}

fn print_counts(counts: &HashMap<String, usize>) {
    for (ext, count) in counts {
        println!("{}: {} files", ext, count);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_dir = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    match count_extensions(target_dir) {
        Ok(counts) => print_counts(&counts),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_extensions() {
        let result = count_extensions(".");

        assert!(result.is_ok());
    }
}