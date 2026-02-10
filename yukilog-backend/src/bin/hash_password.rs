use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use std::env;

/// 密码哈希工具
/// 
/// 使用 Argon2id 算法生成密码哈希
/// 
/// # 用法
/// 
/// ```bash
/// cargo run --bin hash_password -- your_password
/// ```
/// 
/// # 输出
/// 
/// 生成的哈希可直接用于 .env 文件的 ADMIN_PASSWORD_HASH 字段
fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("用法: cargo run --bin hash_password -- <password>");
        eprintln!();
        eprintln!("示例:");
        eprintln!("  cargo run --bin hash_password -- mySecurePassword123");
        eprintln!();
        eprintln!("生成的哈希可用于 .env 文件:");
        eprintln!("  ADMIN_PASSWORD_HASH=<生成的哈希>");
        std::process::exit(1);
    }
    
    let password = &args[1];
    
    // 验证密码强度（可选）
    if password.len() < 8 {
        eprintln!("⚠️  警告: 密码长度少于 8 字符，建议使用更强的密码");
    }
    
    // 生成随机盐值
    let salt = SaltString::generate(&mut OsRng);
    
    // 使用 Argon2 默认配置
    let argon2 = Argon2::default();
    
    // 生成密码哈希
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(password_hash) => {
            println!();
            println!("✅ 密码哈希生成成功!");
            println!();
            println!("将以下内容添加到 .env 文件:");
            println!("─────────────────────────────────────────────────────");
            println!("ADMIN_PASSWORD_HASH={}", password_hash);
            println!("─────────────────────────────────────────────────────");
            println!();
            println!("💡 提示:");
            println!("  • 请妥善保管此哈希值");
            println!("  • 不要将 .env 文件提交到版本控制系统");
            println!("  • 默认使用 Argon2id 算法（推荐）");
            println!();
        }
        Err(e) => {
            eprintln!("❌ 生成密码哈希失败: {}", e);
            std::process::exit(1);
        }
    }
}
