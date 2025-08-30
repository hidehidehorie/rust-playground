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

    // ひとまず引数が "ls" のときだけ処理
    if args.len() > 1 && args[1] == "ls" {
        let tasks = vec![
        Task { id: 1, title: "A".into() },
        Task { id: 2, title: "B".into() },
        Task { id: 3, title: "C".into() },
        ];

        println!("=== TODO List ===");
        for task in tasks {
            println!("{}", task);
        }
    } else {
        println!("一覧表示: cargo run -- ls");
    }
}
