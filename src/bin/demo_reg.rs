use rovkit::regkit::*;

#[allow(dead_code)]
fn main() {
    let text = "My email is abc@example.com and 123@qq.com";
    // Rust字符串中反斜杠用单个 \
    let pattern = r"\b(\w+?)@(\w+?)\.com\b";

    println!("匹配邮箱: {}", is_match(pattern, text));
    println!("所有邮箱: {:?}", find_all(pattern, text));
    println!("首个邮箱: {:?}", find_first(pattern, text));
    println!("首个邮箱分组: {:?}", find_groups(pattern, text));
    println!("所有邮箱分组: {:?}", find_all_groups(pattern, text));

    println!("替换所有邮箱: {}", replace_all(pattern, text, "[email]"));
}
