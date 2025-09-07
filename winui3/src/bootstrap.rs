use std::fmt;
use windows::Win32::{
    Storage::Packaging::Appx::{
        AddPackageDependency, AddPackageDependencyOptions_None,
        CreatePackageDependencyOptions_None, PackageDependencyLifetimeKind_Process,
        PackageDependencyProcessorArchitectures_None, RemovePackageDependency,
        TryCreatePackageDependency, PACKAGEDEPENDENCY_CONTEXT, PACKAGE_VERSION, PACKAGE_VERSION_0,
    },
    System::Memory::{GetProcessHeap, HeapFree, HEAP_FLAGS},
};
use windows_core::{h, Result, HSTRING, PWSTR};

const PACKAGEFAMILYNAME_V1_1: &HSTRING = h!("Microsoft.WindowsAppRuntime.1.1_8wekyb3d8bbwe");

const PACKAGEFAMILYNAME_V1_2: &HSTRING = h!("Microsoft.WindowsAppRuntime.1.2_8wekyb3d8bbwe");

const PACKAGEFAMILYNAME_V1_3: &HSTRING = h!("Microsoft.WindowsAppRuntime.1.3_8wekyb3d8bbwe");

const PACKAGEFAMILYNAME_V1_4: &HSTRING = h!("Microsoft.WindowsAppRuntime.1.4_8wekyb3d8bbwe");

const PACKAGEFAMILYNAME_V1_5: &HSTRING = h!("Microsoft.WindowsAppRuntime.1.5_8wekyb3d8bbwe");

const PACKAGEFAMILYNAME_CBS: &HSTRING = h!("Microsoft.WindowsAppRuntime.CBS_8wekyb3d8bbwe");

const PACKAGEFAMILYNAME_V1_6: &HSTRING = h!("Microsoft.WindowsAppRuntime.1.6_8wekyb3d8bbwe");

const PACKAGEFAMILYNAME_VNEXT_CBS: &HSTRING =
    h!("Microsoft.WindowsAppRuntime.vNext.CBS_8wekyb3d8bbwe");

const PACKAGEFAMILYNAME_V1_7: &HSTRING = h!("Microsoft.WindowsAppRuntime.1.7_8wekyb3d8bbwe");

pub enum WindowsAppSDKVersion {
    V1_1,
    V1_2,
    V1_3,
    V1_4,
    V1_5,
    V1_6,
    V1_7,
    CBS,
    VNextCbs,
}

impl WindowsAppSDKVersion {
    const fn get_package_family_name(&self) -> &'static HSTRING {
        match self {
            Self::V1_1 => PACKAGEFAMILYNAME_V1_1,
            Self::V1_2 => PACKAGEFAMILYNAME_V1_2,
            Self::V1_3 => PACKAGEFAMILYNAME_V1_3,
            Self::V1_4 => PACKAGEFAMILYNAME_V1_4,
            Self::V1_5 => PACKAGEFAMILYNAME_V1_5,
            Self::V1_6 => PACKAGEFAMILYNAME_V1_6,
            Self::V1_7 => PACKAGEFAMILYNAME_V1_7,
            Self::CBS => PACKAGEFAMILYNAME_CBS,
            Self::VNextCbs => PACKAGEFAMILYNAME_VNEXT_CBS,
        }
    }
}

#[derive(Debug)]
struct PackageDependencyID(PWSTR);

unsafe impl Sync for PackageDependencyID {}
unsafe impl Send for PackageDependencyID {}

impl Drop for PackageDependencyID {
    fn drop(&mut self) {
        unsafe {
            if let Ok(heap) = GetProcessHeap() {
                HeapFree(heap, HEAP_FLAGS(0), Some(self.0 .0.cast())).ok();
            }
        }
    }
}

pub struct PackageDependency {
    ctx: PACKAGEDEPENDENCY_CONTEXT,
    package_full_name: HSTRING,
}

impl PackageDependency {
    pub fn initialize() -> Result<Self> {
        Self::initialize_version(WindowsAppSDKVersion::V1_7)
    }

    pub fn initialize_version(version: WindowsAppSDKVersion) -> Result<Self> {
        let min_version = PACKAGE_VERSION {
            Anonymous: PACKAGE_VERSION_0 { Version: 0 },
        };
        let dependency_id = unsafe {
            TryCreatePackageDependency(
                windows::Win32::Security::PSID::default(),
                version.get_package_family_name(),
                min_version,
                PackageDependencyProcessorArchitectures_None,
                PackageDependencyLifetimeKind_Process,
                None,
                CreatePackageDependencyOptions_None,
            )
        }?;
        let dependency_id = PackageDependencyID(dependency_id);

        let mut ctx = PACKAGEDEPENDENCY_CONTEXT::default();
        let mut package_full_name = PWSTR::null();

        unsafe {
            AddPackageDependency(
                dependency_id.0,
                0,
                AddPackageDependencyOptions_None,
                &mut ctx,
                Some(&mut package_full_name),
            )
        }?;

        Ok(Self {
            ctx,
            package_full_name: unsafe { package_full_name.to_hstring() },
        })
    }

    fn uninitialize(&self) -> Result<()> {
        unsafe { RemovePackageDependency(self.ctx) }
    }
}

impl fmt::Debug for PackageDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackageDependency")
            .field("package_full_name", &self.package_full_name)
            .finish_non_exhaustive()
    }
}

impl Drop for PackageDependency {
    fn drop(&mut self) {
        self.uninitialize()
            .expect("failed to remove package dependency")
    }
}
