use std::{env, fs, str};

fn main() -> Result<(), &'static str> {
    env::var("CARGO")
        .map_err(|_| r"please run this tool with `cargo run -p bindgen` from the workspace root")?;

    if !fs::exists("bindgen/winmd").expect("failed to check if winmd dir exists") {
        return Err("please make sure to put WinUI 3 metadata in the bindgen/winmd dir");
    }

    println!("Generating Windows.UI.Xaml.Interop bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/src/xaml_interop.txt"]);

    println!("Generating WinUI 3 bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/src/winui3.txt"]);

    println!("Patching features...");
    patch_winui3_features();

    println!("Patching fn IApplicationFactory...");
    patch_application_factory();

    println!("Done.");
    Ok(())
}

fn patch_winui3_features() {
    let manifest =
        fs::read_to_string("winui3/Cargo.toml").expect("failed to read winui3/Cargo.toml");
    let manifest = manifest
        .replace(
            r#"UI = ["Foundation"]"#,
            r#"UI = ["Foundation", "windows/UI"]"#,
        )
        .replace(
            r#"UI_Composition = ["UI"]"#,
            r#"UI_Composition = ["UI", "windows/UI_Composition"]"#,
        )
        .replace(
            r#"UI_Text = ["UI"]"#,
            r#"UI_Text = ["UI", "windows/UI_Text"]"#,
        );
    fs::write("winui3/Cargo.toml", &manifest).expect("failed to write winui3/Cargo.toml");
}

fn patch_application_factory() {
    const MICROSOFT_UI_XAML_MOD: &str = "winui3/src/Microsoft/UI/Xaml/mod.rs";

    let contents = fs::read_to_string(MICROSOFT_UI_XAML_MOD).expect("failed to read mod.rs");
    let contents = contents.replace(
        r#"fn IApplicationFactory"#,
        r#"pub(crate) fn IApplicationFactory"#,
    );
    fs::write(MICROSOFT_UI_XAML_MOD, &contents).expect("failed to write mod.rs");
}
