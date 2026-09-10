#[inline]
pub unsafe fn GetProcessHeap() -> super::super::Foundation::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeap() -> super::super::Foundation::HANDLE);
    unsafe { GetProcessHeap() }
}
#[inline]
pub unsafe fn HeapFree(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapFree(hheap : super::super::Foundation::HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { HeapFree(hheap as _, dwflags, lpmem.unwrap_or(core::mem::zeroed()) as _) }
}
pub type HEAP_FLAGS = u32;
