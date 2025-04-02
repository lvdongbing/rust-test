// 实现一个命令行工具，对指定目录下的所有文本文件进行搜索，将匹配结果汇总后输出。
// 为增强可玩性和综合性，该工具需要支持：
// - 命令行参数（接收搜索关键词、目录路径、是否忽略大小写等）。
// - 并发搜索。
// - 消息通信。
// - 数据结构。
// - 错误处理。
// - 文件操作。
// - 迭代器与泛型（文本行迭代、搜索函数可考虑使用泛型或 trait 做一定延伸）。
// - 可选扩展：忽略大小写、正则匹配、统计行数或文件数等。

use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

struct Config {
    keyword: String,
    directory: String,
    case_insensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: <keyword> <directory> [case_insensitive]");
        }
        let keyword = args[1].clone();
        let directory = args[2].clone();
        let case_insensitive = args.get(3).map_or(false, |arg| arg == "case_insensitive");
        Ok(Config {
            keyword,
            directory,
            case_insensitive,
        })
    }
}

fn search_file(file_path: &Path, keyword: &str, case_insensitive: bool) -> io::Result<Vec<String>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut results = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let matched = if case_insensitive {
            line.to_lowercase().contains(&keyword.to_lowercase())
        } else {
            line.contains(keyword)
        };
        if matched {
            results.push(format!(
                "{}:{}: {}",
                file_path.display(),
                line_number + 1,
                line
            ));
        }
    }
    Ok(results)
}

fn search_directory(config: Config) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    for entry in fs::read_dir(&config.directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
            let tx = tx.clone();
            let keyword = config.keyword.clone();
            let case_insensitive = config.case_insensitive;

            let handle = thread::spawn(move || {
                if let Ok(results) = search_file(&path, &keyword, case_insensitive) {
                    for result in results {
                        tx.send(result).unwrap();
                    }
                }
            });
            handles.push(handle);
        }
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    for result in rx {
        println!("{}", result);
    }

    Ok(())
}

pub fn question6() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = search_directory(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_search_file() {
        let path = Path::new("test.txt");
        let keyword = "test";
        let case_insensitive = true;

        // Create a temporary file for testing
        let mut file = File::create(&path).unwrap();
        writeln!(file, "This is a test line.").unwrap();
        writeln!(file, "Another line without the keyword.").unwrap();

        let results = search_file(&path, keyword, case_insensitive).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("This is a test line."));

        // Clean up the temporary file
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_search_directory() {
        let config = Config {
            keyword: "test".to_string(),
            directory: ".".to_string(),
            case_insensitive: true,
        };
        assert!(search_directory(config).is_ok());
    }

    #[test]
    fn test_search_file_case_insensitive() {
        let path = Path::new("test_case_insensitive.txt");
        let keyword = "TEST";
        let case_insensitive = true;

        // Create a temporary file for testing
        let mut file = File::create(&path).unwrap();
        writeln!(file, "This is a test line.").unwrap();
        writeln!(file, "Another line without the keyword.").unwrap();

        let results = search_file(&path, keyword, case_insensitive).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("This is a test line."));

        // Clean up the temporary file
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_search_file_case_sensitive() {
        let path = Path::new("test_case_sensitive.txt");
        let keyword = "TEST";
        let case_insensitive = false;

        // Create a temporary file for testing
        let mut file = File::create(&path).unwrap();
        writeln!(file, "This is a test line.").unwrap();
        writeln!(file, "Another line without the keyword.").unwrap();

        let results = search_file(&path, keyword, case_insensitive).unwrap();
        assert!(results.is_empty());

        // Clean up the temporary file
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_search_file_no_matches() {
        let path = Path::new("test_no_matches.txt");
        let keyword = "nonexistent";
        let case_insensitive = true;

        // Create a temporary file for testing
        let mut file = File::create(&path).unwrap();
        writeln!(file, "This is a test line.").unwrap();
        writeln!(file, "Another line without the keyword.").unwrap();

        let results = search_file(&path, keyword, case_insensitive).unwrap();
        assert!(results.is_empty());

        // Clean up the temporary file
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_search_file_empty_file() {
        let path = Path::new("test_empty_file.txt");
        let keyword = "test";
        let case_insensitive = true;

        // Create an empty temporary file for testing
        File::create(&path).unwrap();

        let results = search_file(&path, keyword, case_insensitive).unwrap();
        assert!(results.is_empty());

        // Clean up the temporary file
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_search_directory_with_no_txt_files() {
        let temp_dir = "test_no_txt_files";
        fs::create_dir(temp_dir).unwrap();

        let config = Config {
            keyword: "test".to_string(),
            directory: temp_dir.to_string(),
            case_insensitive: true,
        };

        let result = search_directory(config);
        assert!(result.is_ok());

        // Clean up the temporary directory
        fs::remove_dir(temp_dir).unwrap();
    }
}