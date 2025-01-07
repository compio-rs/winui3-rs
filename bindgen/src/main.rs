use std::{env, fs, str};

fn main() -> Result<(), &'static str> {
    env::var("CARGO")
        .map_err(|_| r"please run this tool with `cargo run -p bindgen` from the workspace root")?;

    if !fs::exists("bindgen/winmd").expect("failed to check if winmd dir exists") {
        return Err("please make sure to put WinUI 3 metadata in the bindgen/winmd dir");
    }

    println!("Generating Windows.UI.Xaml.Interop bindings...");
    bindgen_xaml_interop();

    println!("Generating WinUI 3 bindings...");
    bindgen_winui3();

    println!("Patching features...");
    patch_winui3_features();

    println!("Patching fn IApplicationFactory...");
    patch_application_factory();

    println!("Done.");
    Ok(())
}

fn bindgen_xaml_interop() {
    let interop_args = [
        "--in",
        "default",
        "--out",
        "winui3",
        "--package",
        "--filter",
        "Windows.UI.Xaml.Interop.TypeKind",
        "Windows.UI.Xaml.Interop.TypeName",
    ];
    windows_bindgen::bindgen(interop_args);
}

fn bindgen_winui3() {
    let winui_args = [
        "--in",
        "default",
        "bindgen/winmd/Microsoft.Foundation.winmd",
        "bindgen/winmd/Microsoft.Graphics.winmd",
        "bindgen/winmd/Microsoft.UI.Text.winmd",
        "bindgen/winmd/Microsoft.UI.winmd",
        "bindgen/winmd/Microsoft.UI.Xaml.winmd",
        "bindgen/winmd/Microsoft.Web.WebView2.Core.winmd",
        "bindgen/winmd/Microsoft.Windows.ApplicationModel.Resources.winmd",
        "--reference",
        // NOTE: This is a workaround for a bug in `windows-bindgen` which might never be fixed
        // NOTE: Due to the bug, we have to generate Windows.UI.Xaml.Interop separately
        // NOTE: This relies on the fact that references are added in order of appearance
        "crate,full,Windows.UI.Xaml.Interop",
        "windows,skip-root,Windows",
        "--out",
        "winui3",
        "--implement",
        "--package",
        "--filter",
        "Microsoft.Graphics",
        "Microsoft.UI",
        "Microsoft.Windows.ApplicationModel.Resources",
        // WebView2 is a big part of the API surface
        "!Microsoft.UI.Xaml.Automation.Peers.IWebView2AutomationPeer",
        "!Microsoft.UI.Xaml.Automation.Peers.IWebView2AutomationPeerFactory",
        "!Microsoft.UI.Xaml.Automation.Peers.WebView2AutomationPeer",
        "!Microsoft.UI.Xaml.Controls.IWebView2",
        "!Microsoft.UI.Xaml.Controls.IWebView22",
        "!Microsoft.UI.Xaml.Controls.IWebView2Factory",
        "!Microsoft.UI.Xaml.Controls.IWebView2Statics",
        "!Microsoft.UI.Xaml.Controls.WebView2",
        "!Microsoft.Web.WebView2",
    ];
    windows_bindgen::bindgen(winui_args);
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
