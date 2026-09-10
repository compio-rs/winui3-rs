#[cfg(feature = "Win32_UI_Shell_Common")]
pub mod Common;
#[inline]
pub unsafe fn PathCchCombineEx<P2, P3>(pszpathout: windows_core::PWSTR, cchpathout: usize, pszpathin: P2, pszmore: P3, dwflags: PATHCCH_OPTIONS) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCombineEx(pszpathout : windows_core::PWSTR, cchpathout : usize, pszpathin : windows_core::PCWSTR, pszmore : windows_core::PCWSTR, dwflags : PATHCCH_OPTIONS) -> windows_core::HRESULT);
    unsafe { PathCchCombineEx(pszpathout, cchpathout, pszpathin.param().abi(), pszmore.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn SHGetSpecialFolderPathW(hwnd: Option<super::super::Foundation::HWND>, pszpath: windows_core::PWSTR, csidl: i32, fcreate: bool) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHGetSpecialFolderPathW(hwnd : super::super::Foundation::HWND, pszpath : windows_core::PWSTR, csidl : i32, fcreate : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SHGetSpecialFolderPathW(hwnd.unwrap_or(core::mem::zeroed()) as _, pszpath, csidl, fcreate.into()) }
}
pub const CSIDL_WINDOWS: u32 = 36;
pub type FILEOPENDIALOGOPTIONS = u32;
pub const FOS_ALLNONSTORAGEITEMS: FILEOPENDIALOGOPTIONS = 128;
pub const FOS_ALLOWMULTISELECT: FILEOPENDIALOGOPTIONS = 512;
pub const FOS_CREATEPROMPT: FILEOPENDIALOGOPTIONS = 8192;
pub const FOS_DEFAULTNOMINIMODE: FILEOPENDIALOGOPTIONS = 536870912;
pub const FOS_DONTADDTORECENT: FILEOPENDIALOGOPTIONS = 33554432;
pub const FOS_FILEMUSTEXIST: FILEOPENDIALOGOPTIONS = 4096;
pub const FOS_FORCEFILESYSTEM: FILEOPENDIALOGOPTIONS = 64;
pub const FOS_FORCEPREVIEWPANEON: FILEOPENDIALOGOPTIONS = 1073741824;
pub const FOS_FORCESHOWHIDDEN: FILEOPENDIALOGOPTIONS = 268435456;
pub const FOS_HIDEMRUPLACES: FILEOPENDIALOGOPTIONS = 131072;
pub const FOS_HIDEPINNEDPLACES: FILEOPENDIALOGOPTIONS = 262144;
pub const FOS_NOCHANGEDIR: FILEOPENDIALOGOPTIONS = 8;
pub const FOS_NODEREFERENCELINKS: FILEOPENDIALOGOPTIONS = 1048576;
pub const FOS_NOREADONLYRETURN: FILEOPENDIALOGOPTIONS = 32768;
pub const FOS_NOTESTFILECREATE: FILEOPENDIALOGOPTIONS = 65536;
pub const FOS_NOVALIDATE: FILEOPENDIALOGOPTIONS = 256;
pub const FOS_OKBUTTONNEEDSINTERACTION: FILEOPENDIALOGOPTIONS = 2097152;
pub const FOS_OVERWRITEPROMPT: FILEOPENDIALOGOPTIONS = 2;
pub const FOS_PATHMUSTEXIST: FILEOPENDIALOGOPTIONS = 2048;
pub const FOS_PICKFOLDERS: FILEOPENDIALOGOPTIONS = 32;
pub const FOS_SHAREAWARE: FILEOPENDIALOGOPTIONS = 16384;
pub const FOS_STRICTFILETYPES: FILEOPENDIALOGOPTIONS = 4;
pub const FOS_SUPPORTSTREAMABLEITEMS: FILEOPENDIALOGOPTIONS = 2147483648;
pub const FileOpenDialog: windows_core::GUID = windows_core::GUID::from_u128(0xdc1c5a9c_e88a_4dde_a5a1_60f82a20aef7);
pub const FileSaveDialog: windows_core::GUID = windows_core::GUID::from_u128(0xc0b4e2f3_ba21_4773_8dba_335ec946eb8b);
windows_core::imp::define_interface!(IFileDialog, IFileDialog_Vtbl, 0x42f85136_db7e_439c_85f1_e4075d135fc8);
impl core::ops::Deref for IFileDialog {
    type Target = IModalWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFileDialog, windows_core::IUnknown, IModalWindow);
impl IFileDialog {
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub unsafe fn SetFileTypes(&self, rgfilterspec: &[Common::COMDLG_FILTERSPEC]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileTypes)(windows_core::Interface::as_raw(self), rgfilterspec.len().try_into().unwrap(), rgfilterspec.as_ptr()).ok() }
    }
    pub unsafe fn SetFileTypeIndex(&self, ifiletype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileTypeIndex)(windows_core::Interface::as_raw(self), ifiletype).ok() }
    }
    pub unsafe fn GetFileTypeIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileTypeIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
    pub unsafe fn SetOptions(&self, fos: FILEOPENDIALOGOPTIONS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), fos).ok() }
    }
    pub unsafe fn GetOptions(&self) -> windows_core::Result<FILEOPENDIALOGOPTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultFolder<P0>(&self, psi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultFolder)(windows_core::Interface::as_raw(self), psi.param().abi()).ok() }
    }
    pub unsafe fn SetFolder<P0>(&self, psi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFolder)(windows_core::Interface::as_raw(self), psi.param().abi()).ok() }
    }
    pub unsafe fn GetFolder(&self) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentSelection(&self) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFileName<P0>(&self, pszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFileName)(windows_core::Interface::as_raw(self), pszname.param().abi()).ok() }
    }
    pub unsafe fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), psztitle.param().abi()).ok() }
    }
    pub unsafe fn SetOkButtonLabel<P0>(&self, psztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOkButtonLabel)(windows_core::Interface::as_raw(self), psztext.param().abi()).ok() }
    }
    pub unsafe fn SetFileNameLabel<P0>(&self, pszlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFileNameLabel)(windows_core::Interface::as_raw(self), pszlabel.param().abi()).ok() }
    }
    pub unsafe fn GetResult(&self) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDefaultExtension<P0>(&self, pszdefaultextension: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultExtension)(windows_core::Interface::as_raw(self), pszdefaultextension.param().abi()).ok() }
    }
    pub unsafe fn Close(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), hr).ok() }
    }
    pub unsafe fn SetClientGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientGuid)(windows_core::Interface::as_raw(self), guid).ok() }
    }
    pub unsafe fn ClearClientData(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearClientData)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileDialog_Vtbl {
    pub base__: IModalWindow_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub SetFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const Common::COMDLG_FILTERSPEC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_Common"))]
    SetFileTypes: usize,
    pub SetFileTypeIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFileTypeIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, FILEOPENDIALOGOPTIONS) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FILEOPENDIALOGOPTIONS) -> windows_core::HRESULT,
    pub SetDefaultFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetOkButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFileNameLabel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    AddPlace: usize,
    pub SetDefaultExtension: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub SetClientGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub ClearClientData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    SetFilter: usize,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::RuntimeName for IFileDialog {}
windows_core::imp::define_interface!(IFileOpenDialog, IFileOpenDialog_Vtbl, 0xd57c7288_d4ad_4768_be02_9d969532d960);
impl core::ops::Deref for IFileOpenDialog {
    type Target = IFileDialog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFileOpenDialog, windows_core::IUnknown, IModalWindow, IFileDialog);
impl IFileOpenDialog {
    pub unsafe fn GetResults(&self) -> windows_core::Result<IShellItemArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResults)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSelectedItems(&self) -> windows_core::Result<IShellItemArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenDialog_Vtbl {
    pub base__: IFileDialog_Vtbl,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelectedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_Common")]
impl windows_core::RuntimeName for IFileOpenDialog {}
windows_core::imp::define_interface!(IModalWindow, IModalWindow_Vtbl, 0xb4db1657_70d7_485e_8e3e_6fcb5a5c1802);
windows_core::imp::interface_hierarchy!(IModalWindow, windows_core::IUnknown);
impl IModalWindow {
    pub unsafe fn Show(&self, hwndowner: Option<super::super::Foundation::HWND>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), hwndowner.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IModalWindow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait IModalWindow_Impl: windows_core::IUnknownImpl {
    fn Show(&self, hwndowner: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl IModalWindow_Vtbl {
    pub const fn new<Identity: IModalWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: IModalWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModalWindow_Impl::Show(this, core::mem::transmute_copy(&hwndowner)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IModalWindow as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IModalWindow {}
windows_core::imp::define_interface!(IShellItem, IShellItem_Vtbl, 0x43826d1e_e718_42ee_bc55_a1e261c37bfe);
windows_core::imp::interface_hierarchy!(IShellItem, windows_core::IUnknown);
impl IShellItem {
    pub unsafe fn GetParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDisplayName(&self, sigdnname: SIGDN) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), sigdnname, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Compare<P0>(&self, psi: P0, hint: u32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), psi.param().abi(), hint, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    BindToHandler: usize,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, SIGDN, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    GetAttributes: usize,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut i32) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IShellItem {}
windows_core::imp::define_interface!(IShellItemArray, IShellItemArray_Vtbl, 0xb63ea76d_1f85_456f_a19c_48159efa858b);
windows_core::imp::interface_hierarchy!(IShellItemArray, windows_core::IUnknown);
impl IShellItemArray {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetItemAt(&self, dwindex: u32) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemAt)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellItemArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    BindToHandler: usize,
    GetPropertyStore: usize,
    GetPropertyDescriptionList: usize,
    GetAttributes: usize,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    EnumItems: usize,
}
impl windows_core::RuntimeName for IShellItemArray {}
pub const PATHCCH_NONE: PATHCCH_OPTIONS = 0;
pub type PATHCCH_OPTIONS = u32;
pub type SIGDN = i32;
pub const SIGDN_DESKTOPABSOLUTEEDITING: SIGDN = -2147172352;
pub const SIGDN_DESKTOPABSOLUTEPARSING: SIGDN = -2147319808;
pub const SIGDN_FILESYSPATH: SIGDN = -2147123200;
pub const SIGDN_NORMALDISPLAY: SIGDN = 0;
pub const SIGDN_PARENTRELATIVE: SIGDN = -2146959359;
pub const SIGDN_PARENTRELATIVEEDITING: SIGDN = -2147282943;
pub const SIGDN_PARENTRELATIVEFORADDRESSBAR: SIGDN = -2146975743;
pub const SIGDN_PARENTRELATIVEFORUI: SIGDN = -2146877439;
pub const SIGDN_PARENTRELATIVEPARSING: SIGDN = -2147385343;
pub const SIGDN_URL: SIGDN = -2147057664;
