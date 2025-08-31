use std::fmt;
use std::env;

struct Task {
    id: u32,
    title: String,
}

// 出力をフォーマットする
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.id, self.title)
    }
}

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    let mut tasks = vec![
        Task { id: 1, title: "A".into() },
        Task { id: 2, title: "B".into() },
        Task { id: 3, title: "C".into() },
    ];
        
    if args.len() > 1 {
        match args[1].as_str() {
            "ls" => {
                println!("=== TODO List ===");
                for task in &tasks {
                    println!("{}", task);
                }
            }
            "add" => {
                if args.len() > 2 {
                    let next_id = tasks.len() as u32 + 1;
                    let title = args[2].clone();
                    let new_task = Task { id: next_id, title };
                    tasks.push(new_task);

                    println!("追加しました:");
                    for task in &tasks {
                        println!("{}", task);
                    }
                } else {
                    println!("使い方: cargo run -- add <todo>");
                }
            }
            "rm" => {
                if args.len() > 2 {
                    let id = args[2].parse().unwrap_or(0);
                    let before = tasks.len();
                    tasks.retain(|t| t.id != id);
                    if tasks.len() < before {
                        println!("削除しました: [{}]", id);
                    } else {
                        println!("ID {} のタスクは見つかりませんでした", id);
                    }
                    println!("=== TODO List ===");
                    for task in &tasks {
                        println!("{}", task);
                    }
                } else {
                    println!("使い方: cargo run -- rm <ID>");
                }
            }
            _ => {
                println!("コマンドが不明です。");
                println!("使い方:");
                println!("  cargo run -- ls");
                println!("  cargo run -- add <todo>");
                println!("  cargo run -- rm <ID>");
            }
        }
    } else {
        println!("コマンドを指定してください。 (ls/add/rm)");
    }
}
