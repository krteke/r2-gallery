#[cfg(feature = "build-tools")]
fn main() {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    use rpassword::prompt_password;
    println!("输入要哈希的密码：");
    let password = prompt_password("Password: ").expect("读取密码失败");

    if password.trim().is_empty() {
        eprintln!("密码不能为空");
        std::process::exit(1);
    }

    println!("正在哈希密码...");

    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("哈希密码失败")
        .to_string();

    println!("\n密码哈希成功!");
    println!("-------------------------------");
    println!("把下面的哈希值保存到你的配置文件中：");
    println!("\n{}\n", hashed_password);
    println!("-------------------------------");
}
