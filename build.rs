//! 构建脚本:Windows 下把预生成的应用图标(ICO)嵌入 exe,
//! 作为窗口类图标(gpui 以资源 ID=1 加载,任务栏/Alt-Tab 亦使用)。
//! 其他平台:无操作(winresource 也经 target 门控,不会被编译)。
//!
//! 图标源文件:`src/assets/app-icon.ico`(标题栏 SVG 的 256×256 转换产物)。

fn main() {
    // 注意:必须按"目标平台"而非"宿主平台"判断(交叉编译时二者不同),
    // 因此用 CARGO_CFG_TARGET_OS 环境变量而非 #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    println!("cargo:rerun-if-changed=src/assets/app-icon.ico");

    let mut compiler = winresource::WindowsResource::new();
    compiler.set_icon("src/assets/app-icon.ico");
    compiler
        .compile()
        .expect("failed to compile Windows resources");
}
