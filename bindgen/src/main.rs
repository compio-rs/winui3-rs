use std::{env, fs, str};

use toml_edit::{value, Array, DocumentMut};

fn main() -> Result<(), &'static str> {
    env::var("CARGO").map_err(|_| r"please run this tool with `cargo run -p bindgen` from the workspace root")?;

    unsafe {
        std::env::set_var("WINDOWS_BINDGEN_TIMINGS", "1");
    }

    windows_bindgen::bindgen(["--etc", "bindgen/etc/subset.txt"]);
    windows_bindgen::bindgen(["--etc", "bindgen/etc/webview2.txt"]);
    windows_bindgen::bindgen(["--etc", "bindgen/etc/winui3.txt"]);

    println!("Patching features...");
    patch_winui3_features();

    println!("Done.");
    Ok(())
}

// The full desired dependency list for each feature `windows-bindgen` can't
// emit on its own — both internal features and the `windows/*` pass-throughs
// that the generator doesn't infer.
#[rustfmt::skip]
const FEATURE_PATCHES: &[(&str, &[&str])] = &[
    ("Graphics_Display",               &["windows/Storage_Streams"]),
    ("UI",                             &["Foundation", "windows/UI"]),
    ("UI_Composition",                 &["UI", "windows/UI_Composition"]),
    ("UI_Composition_SystemBackdrops", &["UI_Composition", "windows/UI_Core"]),
    ("UI_Content",                     &["UI", "windows/Graphics"]),
    ("UI_Input",                       &["UI", "windows/Graphics", "windows/System", "windows/UI_Core"]),
    ("UI_Input_DragDrop",              &["UI_Input", "windows/ApplicationModel_DataTransfer", "windows/Graphics_Imaging"]),
    ("UI_Input_Interop",               &["UI_Input", "windows/Devices_Input"]),
    ("UI_Text",                        &["UI", "windows/Storage_Streams", "windows/UI_Text"]),
    ("UI_Windowing",                   &["UI", "windows/Graphics"]),
    ("UI_Xaml",                        &["UI", "UI_Xaml_Interop", "windows/ApplicationModel_Activation", "windows/ApplicationModel_DataTransfer_DragDrop", "windows/Graphics_Imaging", "windows/UI_Core", "windows/Win32_Foundation"]),
    ("UI_Xaml_Controls",               &["UI_Text", "UI_Xaml", "windows/Media_Playback"]),
    ("UI_Xaml_Documents",              &["UI_Text", "UI_Xaml"]),
    ("UI_Xaml_Input",                  &["UI_Input", "UI_Xaml"]),
    ("UI_Xaml_Interop",                &["windows/UI_Xaml_Interop"]),
    ("UI_Xaml_Markup",                 &["UI_Xaml", "windows/Storage_Streams"]),
    ("UI_Xaml_Media",                  &["UI_Xaml", "windows/Storage_Streams"]),
    ("UI_Xaml_Media_DxInterop",        &["UI_Xaml_Media", "windows/Win32_Graphics_Dxgi"]),
    ("UI_Xaml_Media_Imaging",          &["UI_Xaml_Media", "windows/ApplicationModel_Background"]),
    ("UI_Xaml_Printing",               &["UI_Xaml", "windows/Graphics_Printing"]),
    ("Web",                            &["Foundation"]),
    ("Web_WebView2_Core",              &["Web_WebView2", "windows/ApplicationModel_DataTransfer_DragDrop_Core", "windows/Security_Cryptography_Certificates", "windows/Storage_Streams"]),
];

fn patch_winui3_features() {
    const PATH: &str = "winui3/Cargo.toml";

    let source = fs::read_to_string(PATH).expect("failed to read winui3/Cargo.toml");
    let mut doc = source.parse::<DocumentMut>().expect("failed to parse winui3/Cargo.toml");
    let features = doc["features"].as_table_mut().expect("missing [features] table");

    for &(name, deps) in FEATURE_PATCHES {
        if features.contains_key(name) {
            let array: Array = deps.iter().copied().collect();
            features[name] = value(array);
        } else {
            eprintln!("feature `{name}` not emitted by windows-bindgen — stale FEATURE_PATCHES entry?")
        }
    }

    fs::write(PATH, doc.to_string()).expect("failed to write winui3/Cargo.toml");
}
