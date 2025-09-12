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
use windows_core::{Result, HSTRING, PCWSTR, PWSTR};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WindowsAppSDKVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    V1_4,
    V1_5,
    V1_6,
    V1_7,
    V1_8,
    Cbs,
    VNextCbs,
}

impl WindowsAppSDKVersion {
    const fn get_version(&self) -> &'static str {
        match self {
            Self::V1_0 => "1.0",
            Self::V1_1 => "1.1",
            Self::V1_2 => "1.2",
            Self::V1_3 => "1.3",
            Self::V1_4 => "1.4",
            Self::V1_5 => "1.5",
            Self::V1_6 => "1.6",
            Self::V1_7 => "1.7",
            Self::V1_8 => "1.8",
            Self::Cbs => "CBS",
            Self::VNextCbs => "vNext.CBS",
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
        let package_family_name = HSTRING::from(format!(
            "Microsoft.WindowsAppRuntime.{}_8wekyb3d8bbwe",
            version.get_version()
        ));
        let dependency_id = unsafe {
            TryCreatePackageDependency(
                windows::Win32::Security::PSID::default(),
                PCWSTR(package_family_name.as_ptr()),
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
        let package_full_name = PackageDependencyID(package_full_name);

        Ok(Self {
            ctx,
            package_full_name: unsafe { package_full_name.0.to_hstring() },
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
