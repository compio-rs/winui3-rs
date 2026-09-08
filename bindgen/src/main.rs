use std::{
    collections::HashMap,
    env::{self, args},
    str,
};

fn main() {
    env::var("CARGO")
        .expect(r"please run this tool with `cargo run -p bindgen` from the workspace root");

    let mut api_set = args().skip(1).collect::<Vec<_>>();

    let api_set_map: HashMap<&'static str, &'static str> = HashMap::from_iter([
        ("winui3", "WinUI 3"),
        ("internal", "internal"),
        ("subset", "Win32 (subset)"),
        ("subset_winrt", "WinRT (subset)"),
    ]);

    if api_set.is_empty() {
        api_set = api_set_map.keys().map(|s| s.to_string()).collect();
    }

    for key in api_set {
        if let Some(api_set_name) = api_set_map.get(key.as_str()) {
            println!("Generating {} bindings...", api_set_name);
            windows_bindgen::bindgen(["--etc", &format!("bindgen/etc/{}.txt", key)]);
        } else {
            println!("Unknown API set: {}", key);
        }
    }

    println!("Done.");
}
