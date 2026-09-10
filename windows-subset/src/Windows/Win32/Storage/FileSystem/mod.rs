#[inline]
pub unsafe fn GetFileVersionInfoSizeW<P0>(lptstrfilename: P0, lpdwhandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoSizeW(lptstrfilename : windows_core::PCWSTR, lpdwhandle : *mut u32) -> u32);
    unsafe { GetFileVersionInfoSizeW(lptstrfilename.param().abi(), lpdwhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetFileVersionInfoW<P0>(lptstrfilename: P0, dwhandle: Option<u32>, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoW(lptstrfilename : windows_core::PCWSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetFileVersionInfoW(lptstrfilename.param().abi(), dwhandle.unwrap_or(core::mem::zeroed()) as _, dwlen, lpdata as _) }
}
#[inline]
pub unsafe fn VerQueryValueW<P1>(pblock: *const core::ffi::c_void, lpsubblock: P1, lplpbuffer: *mut *mut core::ffi::c_void, pulen: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn VerQueryValueW(pblock : *const core::ffi::c_void, lpsubblock : windows_core::PCWSTR, lplpbuffer : *mut *mut core::ffi::c_void, pulen : *mut u32) -> windows_core::BOOL);
    unsafe { VerQueryValueW(pblock, lpsubblock.param().abi(), lplpbuffer as _, pulen as _) }
}
