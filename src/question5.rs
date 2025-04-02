// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn question5() {
    let result = _question5();
    for res in result {
        println!("{}", res);
    }
}

fn _question5() -> Vec<String> {
    let urls = vec!["http://example.com/1", "http://example.com/2", "http://example.com/3"];
    let (tx, rx) = mpsc::channel();

    for url in urls {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            // 模拟下载
            thread::sleep(Duration::from_secs(2));
            let result = format!("{} + 下载完成", url);
            tx_clone.send(result).expect("Failed to send message");
        });
    }

    drop(tx);

    let mut result = vec![];
    for received in rx {
        result.push(received);
    }
    result.sort();
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question5() {
        let result = _question5();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "http://example.com/1 + 下载完成");
        assert_eq!(result[1], "http://example.com/2 + 下载完成");
        assert_eq!(result[2], "http://example.com/3 + 下载完成");
    }

    #[test]
    fn test_message_order() {
        let result = _question5();
        let expected = vec![
            "http://example.com/1 + 下载完成",
            "http://example.com/2 + 下载完成",
            "http://example.com/3 + 下载完成",
        ];
        assert_eq!(result, expected, "Messages should be sorted alphabetically");
    }

    #[test]
    fn test_thread_behavior() {
        let urls = vec!["http://example.com/1", "http://example.com/2", "http://example.com/3"];
        let (tx, rx) = mpsc::channel();

        for url in urls {
            let tx_clone = tx.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                let result = format!("{} + 下载完成", url);
                tx_clone.send(result).expect("Failed to send message");
            });
        }

        drop(tx);

        let mut received_messages = vec![];
        for received in rx {
            received_messages.push(received);
        }
    }
}