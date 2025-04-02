// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。

use std::io::Write;

pub fn question4() {
    let mut args = std::env::args();
    let append_mode = args.any(|arg| arg == "--append");
    let file_path = std::env::args().nth(1).unwrap_or_else(|| "input.txt".to_string());
    let output_path = "output.txt";

    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            let (line_count, char_count) = count_lines_and_chars(&content);
            write_output(output_path, line_count, char_count, append_mode).expect("Failed to write output file");
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}

fn count_lines_and_chars(content: &str) -> (usize, usize) {
    let line_count = content.lines().count();
    let char_count = content.chars().filter(|&c| c != '\n').count(); // 不计算换行符
    (line_count, char_count)
}

fn write_output(path: &str, line_count: usize, char_count: usize, append: bool) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(append) // 追加模式
        .truncate(!append) // 覆盖模式
        .open(path)?;

    writeln!(file, "Line count: {}", line_count)?;
    writeln!(file, "Character count (excluding newlines): {}", char_count)?;
    Ok(())
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_and_chars() {
        let content = "Hello, world!\nThis is a test.\n";
        let (line_count, char_count) = count_lines_and_chars(content);
        assert_eq!(line_count, 2);
        assert_eq!(char_count, 28);
    }

    #[test]
    fn test_write_output() {
        let path = "output1.txt";
        let _ = std::fs::remove_file(path);

        write_output(path, 2, 27, false).expect("Failed to write output file");

        let content = std::fs::read_to_string(path).expect("Failed to read output file");
        assert!(content.contains("Line count: 2"));
        assert!(content.contains("Character count (excluding newlines): 27"));

        let _ = std::fs::remove_file(path); // Clean up the test file
    }

    #[test]
    fn test_question4() {
        let input_path = "input.txt";
        let output_path = "output.txt";

        // Create a test input file
        std::fs::write(input_path, "Hello\nWorld\n").expect("Failed to write test input file");

        // Run the function
        question4();

        // Verify the output
        let content = std::fs::read_to_string(output_path).expect("Failed to read output file");
        assert!(content.contains("Line count: 2"));
        assert!(content.contains("Character count (excluding newlines): 10"));

        // Clean up test files
        let _ = std::fs::remove_file(input_path);
        let _ = std::fs::remove_file(output_path);
    }
}