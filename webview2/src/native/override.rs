use std::env::current_exe;

use windows::Win32::{Foundation::ERROR_INSUFFICIENT_BUFFER, Storage::Packaging::Appx::GetCurrentApplicationUserModelId, System::Registry::KEY_QUERY_VALUE};
use windows_core::{HStringBuilder, PWSTR};
use windows_registry::{CURRENT_USER, Key, LOCAL_MACHINE, Value};

use super::*;

const REDIST_OVERRIDE_KEY: &str = "Software\\Policies\\Microsoft\\Edge\\WebView2\\";

const EMBEDDED_OVERRIDE_KEY: &str = "Software\\Policies\\Microsoft\\EmbeddedBrowserWebView\\LoaderOverride\\";

enum RegistryValue {
    String(String),
    Value(Value),
}

impl RegistryValue {
    fn into_hstring(self) -> Option<HSTRING> {
        match self {
            Self::String(s) => Some(s.into()),
            Self::Value(v) => v.try_into().ok(),
        }
    }
}

pub fn update(params: &mut WebView2EnvironmentParams) {
    if let Some(sub_folder) = get_param("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER", "browserExecutableFolder").and_then(|v| v.into_hstring()) {
        params.embedded_edge_sub_folder = CowPCWSTR::Owned(sub_folder);
    }
    if let Some(user_data_dir) = get_param("WEBVIEW2_USER_DATA_FOLDER", "userDataFolder").and_then(|v| v.into_hstring()) {
        params.user_data_dir = CowPCWSTR::Owned(user_data_dir);
    }
    if let Some(channel) = get_param("WEBVIEW2_RELEASE_CHANNEL_PREFERENCE", "releaseChannelPreference") {
        let canary = match channel {
            RegistryValue::Value(v) => v[0] == 1 || trim(v.as_wide()) == [b'1' as u16],
            RegistryValue::String(s) => s == "1",
        };
        params.release_channel_preference = if canary { WebView2ReleaseChannelPreference::Canary } else { WebView2ReleaseChannelPreference::Stable };
    }
}

fn get_param(env: &str, key: &str) -> Option<RegistryValue> {
    if let Ok(env_var) = std::env::var(env) {
        return Some(RegistryValue::String(env_var));
    }

    let key_override = policy_exists(CURRENT_USER) || policy_exists(LOCAL_MACHINE);
    let current = current_exe().unwrap_or_default();
    let exe_name = current.file_name().unwrap_or_default().to_string_lossy();
    let id = app_user_model_id().unwrap_or_default().to_string_lossy();
    get_param_reg(key, LOCAL_MACHINE, true, &exe_name, &id, key_override).or_else(|| get_param_reg(key, CURRENT_USER, true, &exe_name, &id, key_override)).or_else(|| get_param_reg(key, LOCAL_MACHINE, false, &exe_name, &id, key_override)).or_else(|| get_param_reg(key, CURRENT_USER, false, &exe_name, &id, key_override)).map(RegistryValue::Value)
}

fn policy_exists(key: &Key) -> bool {
    key.options().read().open(REDIST_OVERRIDE_KEY).is_ok()
}

fn get_param_reg(key: &str, root: &Key, redist: bool, exe_name: &str, id: &str, key_override: bool) -> Option<Value> {
    if key_override
        && redist
        && let Some(v) = read_override(key, root, id, redist).or_else(|| read_override(key, root, exe_name, redist)).or_else(|| read_override(key, root, "*", redist))
    {
        return Some(v);
    }

    read_override(id, root, key, redist).or_else(|| read_override(exe_name, root, key, redist)).or_else(|| read_override("*", root, key, redist))
}

fn read_override(key: &str, root: &Key, value: &str, redist: bool) -> Option<Value> {
    if key.is_empty() {
        return None;
    }

    let sub_key = if redist { REDIST_OVERRIDE_KEY } else { EMBEDDED_OVERRIDE_KEY };

    let phk = root.options().access(KEY_QUERY_VALUE).open(sub_key).ok()?;

    phk.get_value(value).ok()
}

fn trim(mut wide: &[u16]) -> &[u16] {
    while wide.last() == Some(&0) {
        wide = &wide[..wide.len() - 1];
    }

    wide
}

fn app_user_model_id() -> Option<HSTRING> {
    let mut len = 0;
    let res = unsafe { GetCurrentApplicationUserModelId(&mut len, None) };
    if res == ERROR_INSUFFICIENT_BUFFER {
        let mut buffer = HStringBuilder::new(len as usize);
        unsafe { GetCurrentApplicationUserModelId(&mut len, Some(PWSTR(buffer.as_mut_ptr()))) }.ok().ok()?;
        buffer.trim_end();
        return Some(buffer.into());
    }
    None
}
