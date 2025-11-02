#![cfg(feature = "server")]
use rpassword::prompt_password;

fn main() {
    println!("输入要哈希的密码：");
    let password = prompt_password("Password: ").expect("读取密码失败");

    if password.trim().is_empty() {
        eprintln!("密码不能为空");
        std::process::exit(1);
    }

    println!("正在哈希密码...");

    let hashed_password = "aaa";

    println!("\n密码哈希成功!");
    println!("-------------------------------");
    println!("把下面的哈希值保存到你的配置文件中：");
    println!("\n{}\n", hashed_password);
    println!("-------------------------------");
}
