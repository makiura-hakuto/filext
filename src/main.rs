use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn count_extensions(path: &str) -> io::Result<HashMap<String, (usize, u64)>> {
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
            let size = fs::metadata(&path)?.len();

            let entry = counts.entry(ext).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += size;
        }
    }

    Ok(counts)
}

fn print_counts(counts: &HashMap<String, (usize, u64)>) {
    for (ext, (count, size)) in counts {
        println!("{}: {} files, {} bytes", ext, count, size);
    }
}

fn print_help() {
    println!("filext");
    println!();
    println!("Usage:");
    println!("  filext [PATH]");
    println!();
    println!("Arguments:");
    println!("  PATH    集計対象のディレクトリ（省略時はカレントディレクトリ）");
    println!();
    println!("Options:");
    println!("  -h, --help       ヘルプを表示する");
    println!("  -V, --version    バージョン情報を表示する");
}

fn print_version() {
    println!("filext {}", env!("CARGO_PKG_VERSION"));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-V" | "--version" => {
                print_version();
                return;
            }
            _ => {}
        }
    }

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