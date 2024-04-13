use std::process::Command;

/**
 * 在编译 cargo 项目时，做额外的编译处理
 */
fn main() {
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }

    // 编译到 src/pb 目录下
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();

    Command::new("cargo")
        .args(&["fmt", "--", "src/*.rs"])
        .status()
        .expect("cargo fmt failed");
}
