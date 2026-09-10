//! https://github.com/jchv/OpenWebView2Loader
//!
//! A reimplementation of WebView2Loader in pure Rust.

#![allow(non_snake_case, clippy::missing_safety_doc)]

use crate::*;
use windows::Win32::{
    Foundation::{E_INVALIDARG, E_POINTER},
    System::Com::CoTaskMemAlloc,
};
use windows_core::{Error, HSTRING, PCWSTR, PWSTR, Param, Result};

mod load;
mod r#override;

#[inline]
pub unsafe fn CreateCoreWebView2Environment<P0>(handler: P0) -> Result<()>
where
    P0: Param<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>,
{
    unsafe { CreateCoreWebView2EnvironmentWithOptions(None, None, None, handler) }
}

#[inline]
pub unsafe fn CreateCoreWebView2EnvironmentWithOptions<P0, P1, P2, P3>(browser_executable_folder: P0, user_data_folder: P1, options: P2, handler: P3) -> Result<()>
where
    P0: Param<PCWSTR>,
    P1: Param<PCWSTR>,
    P2: Param<ICoreWebView2EnvironmentOptions>,
    P3: Param<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>,
{
    unsafe {
        let handler = handler.param();
        let handler = handler.borrow();
        let handler = handler.ok()?;

        let options = options.param();
        let options = options.borrow();
        let mut params = WebView2EnvironmentParams {
            embedded_edge_sub_folder: browser_executable_folder.param().abi().into(),
            user_data_dir: user_data_folder.param().abi().into(),
            environment_options: options.as_ref(),
            release_channel_preference: WebView2ReleaseChannelPreference::Stable,
        };
        r#override::update(&mut params);
        load::create_env_impl(params, handler)
    }
}

struct WebView2EnvironmentParams<'a> {
    embedded_edge_sub_folder: CowPCWSTR,
    user_data_dir: CowPCWSTR,
    environment_options: Option<&'a ICoreWebView2EnvironmentOptions>,
    release_channel_preference: WebView2ReleaseChannelPreference,
}

enum CowPCWSTR {
    Pointer(PCWSTR),
    Owned(HSTRING),
}

impl CowPCWSTR {
    pub fn as_ptr(&self) -> PCWSTR {
        match self {
            CowPCWSTR::Pointer(ptr) => *ptr,
            CowPCWSTR::Owned(s) => PCWSTR(s.as_ptr()),
        }
    }
}

impl From<PCWSTR> for CowPCWSTR {
    fn from(value: PCWSTR) -> Self {
        Self::Pointer(value)
    }
}

impl From<HSTRING> for CowPCWSTR {
    fn from(value: HSTRING) -> Self {
        Self::Owned(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WebView2ReleaseChannelPreference {
    Stable = 0,
    Canary = 1,
}

#[inline]
pub unsafe fn GetAvailableCoreWebView2BrowserVersionString<P0>(browser_executable_folder: P0, version_info: *mut PWSTR) -> Result<()>
where
    P0: Param<PCWSTR>,
{
    unsafe { GetAvailableCoreWebView2BrowserVersionStringWithOptions(browser_executable_folder, None, version_info) }
}

pub unsafe fn GetAvailableCoreWebView2BrowserVersionStringWithOptions<P0, P1>(browser_executable_folder: P0, options: P1, version_info: *mut windows_core::PWSTR) -> Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<ICoreWebView2EnvironmentOptions>,
{
    if version_info.is_null() || !version_info.is_aligned() {
        return Err(Error::from_hresult(E_POINTER));
    }

    unsafe {
        let options = options.param();
        let options = options.borrow();

        let mut params = WebView2EnvironmentParams {
            embedded_edge_sub_folder: browser_executable_folder.param().abi().into(),
            user_data_dir: CowPCWSTR::Pointer(PCWSTR::null()),
            environment_options: options.as_ref(),
            release_channel_preference: WebView2ReleaseChannelPreference::Stable,
        };
        r#override::update(&mut params);
        let s = load::get_version_string(params)?;
        let byte_len = s.len() * 2 + 2;
        let mem = CoTaskMemAlloc(byte_len).cast::<u16>();
        if !mem.is_null() {
            mem.copy_from_nonoverlapping(s.as_ptr(), s.len() + 1);
        }
        *version_info = PWSTR(mem);
        Ok(())
    }
}

#[inline]
pub unsafe fn CompareBrowserVersions<P0, P1>(version1: P0, version2: P1, result: *mut i32) -> Result<()>
where
    P0: Param<PCWSTR>,
    P1: Param<PCWSTR>,
{
    if result.is_null() || !result.is_aligned() {
        return Err(Error::from_hresult(E_POINTER));
    }

    unsafe {
        let version1 = version1.param().abi();
        let version2 = version2.param().abi();

        if version1.is_null() || version2.is_null() {
            return Err(Error::from_hresult(E_INVALIDARG));
        }

        let v1 = version1.to_string()?;
        let v1 = load::parse_version(&v1).ok_or_else(|| Error::from_hresult(E_INVALIDARG))?;
        let v2 = version2.to_string()?;
        let v2 = load::parse_version(&v2).ok_or_else(|| Error::from_hresult(E_INVALIDARG))?;

        *result = match v1.cmp(&v2) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
    }

    Ok(())
}
