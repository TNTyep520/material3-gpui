fn main() {
    // 注意:必须按"目标平台"而非"宿主平台"判断(交叉编译时二者不同),
    // 因此用 CARGO_CFG_TARGET_OS 环境变量而非 #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    println!("cargo:rerun-if-changed=assets/app-icon.ico");
    let mut compiler = winresource::WindowsResource::new();
    compiler.set_icon("assets/app-icon.ico");
    compiler
        .compile()
        .expect("failed to compile Windows resources");
}
