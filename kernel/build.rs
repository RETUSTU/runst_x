use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取当前时间
    let now = chrono::Local::now();
    let build_date = now.format("%Y-%m-%d").to_string();
    let build_time = now.format("%H:%M:%S").to_string();
    let build_datetime = now.format("%Y-%m-%d-%H:%M:%S").to_string();
    
    // 创建编译信息文件
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    
    let content = format!(
        r#"// 自动生成的编译信息
// 此文件由build.rs在编译时自动生成

/// 编译日期
pub const BUILD_DATE: &str = "{}";

/// 编译时间
pub const BUILD_TIME: &str = "{}";

/// 完整编译时间戳
pub const BUILD_DATETIME: &str = "{}";
"#,
        build_date, build_time, build_datetime
    );
    
    fs::write(&dest_path, content).unwrap();
    
    // 告诉Cargo在编译时重新运行build.rs
    println!("cargo:rerun-if-changed=build.rs");
}