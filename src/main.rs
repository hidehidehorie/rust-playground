use std::fmt;

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
    // 追加・削除することを見据えてベクタにしている
    let tasks = vec![
        // 勉強のためString型を３つ別々で書いてみる
        Task { id: 1, title: String::from("A") },
        Task { id: 2, title: "B".to_string() },
        Task { id: 3, title: "C".into() },
    ];

    println!("=== TODO List ===");
    for task in tasks {
        println!("{}", task);
    }
}
