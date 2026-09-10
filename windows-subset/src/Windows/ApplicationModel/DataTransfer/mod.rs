#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
pub mod DragDrop;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataPackage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataPackage, windows_core::IUnknown, windows_core::IInspectable);
impl DataPackage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataPackage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetView(&self) -> windows_core::Result<DataPackageView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn RequestedOperation(&self) -> windows_core::Result<DataPackageOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequestedOperation(&self, value: DataPackageOperation) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRequestedOperation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn OperationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<OperationCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, OperationCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).OperationCompleted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveOperationCompleted))
        }
    }
    pub fn Destroyed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Destroyed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDestroyed))
        }
    }
    pub fn SetData<P1>(&self, formatid: &windows_core::HSTRING, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(formatid), value.param().abi()).ok() }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUri)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn SetHtmlFormat(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHtmlFormat)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ResourceMap(&self) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceMap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetRtf(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRtf)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBitmap<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::RandomAccessStreamReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBitmap)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn SetStorageItemsReadOnly<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<super::super::Storage::IStorageItem>>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStorageItemsReadOnly)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn SetStorageItems<P0>(&self, value: P0, readonly: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<super::super::Storage::IStorageItem>>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStorageItems)(windows_core::Interface::as_raw(self), value.param().abi(), readonly).ok() }
    }
    pub fn SetApplicationLink<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IDataPackage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetApplicationLink)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetWebLink<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IDataPackage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWebLink)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ShareCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ShareCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IDataPackage3>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, ShareCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ShareCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveShareCompleted))
        }
    }
    pub fn ShareCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IDataPackage4>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ShareCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveShareCanceled))
        }
    }
}
impl windows_core::RuntimeType for DataPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataPackage>();
}
unsafe impl windows_core::Interface for DataPackage {
    type Vtable = <IDataPackage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataPackage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataPackage {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackage";
}
unsafe impl Send for DataPackage {}
unsafe impl Sync for DataPackage {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataPackageOperation(pub u32);
impl DataPackageOperation {
    pub const None: Self = Self(0);
    pub const Copy: Self = Self(1);
    pub const Move: Self = Self(2);
    pub const Link: Self = Self(4);
    pub const NewTarget: Self = Self(1073741824);
    pub const BackgroundTarget: Self = Self(536870912);
}
impl windows_core::imp::TypeKind for DataPackageOperation {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DataPackageOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DataPackageOperation;u4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.DataPackageOperation");
}
impl DataPackageOperation {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DataPackageOperation {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DataPackageOperation {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DataPackageOperation {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for DataPackageOperation {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for DataPackageOperation {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataPackageView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataPackageView, windows_core::IUnknown, windows_core::IInspectable);
impl DataPackageView {
    pub fn RequestedOperation(&self) -> windows_core::Result<DataPackageOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReportOperationCompleted(&self, value: DataPackageOperation) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportOperationCompleted)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn AvailableFormats(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AvailableFormats)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Contains(&self, formatid: &windows_core::HSTRING) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Contains)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(formatid), &mut result__).map(|| result__)
        }
    }
    pub fn GetDataAsync(&self, formatid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(formatid), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetTextAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetCustomTextAsync(&self, formatid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomTextAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(formatid), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetUriAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Foundation::Uri>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUriAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetHtmlFormatAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHtmlFormatAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResourceMapAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IMapView<windows_core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceMapAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetRtfAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRtfAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetBitmapAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmapAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn GetStorageItemsAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::super::Storage::IStorageItem>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorageItemsAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetApplicationLinkAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Foundation::Uri>> {
        let this = &windows_core::Interface::cast::<IDataPackageView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetApplicationLinkAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetWebLinkAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Foundation::Uri>> {
        let this = &windows_core::Interface::cast::<IDataPackageView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetWebLinkAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_EnterpriseData")]
    pub fn RequestAccessAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>> {
        let this = &windows_core::Interface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_EnterpriseData")]
    pub fn RequestAccessWithEnterpriseIdAsync(&self, enterpriseid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>> {
        let this = &windows_core::Interface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessWithEnterpriseIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(enterpriseid), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_EnterpriseData")]
    pub fn UnlockAndAssumeEnterpriseIdentity(&self) -> windows_core::Result<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult> {
        let this = &windows_core::Interface::cast::<IDataPackageView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnlockAndAssumeEnterpriseIdentity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAcceptedFormatId(&self, formatid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDataPackageView4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAcceptedFormatId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(formatid)).ok() }
    }
}
impl windows_core::RuntimeType for DataPackageView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataPackageView>();
}
unsafe impl windows_core::Interface for DataPackageView {
    type Vtable = <IDataPackageView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataPackageView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataPackageView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DataPackageView";
}
unsafe impl Send for DataPackageView {}
unsafe impl Sync for DataPackageView {}
windows_core::imp::define_interface!(IDataPackage, IDataPackage_Vtbl, 0x61ebf5c7_efea_4346_9554_981d7e198ffe);
impl windows_core::RuntimeType for IDataPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IDataPackage");
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IDataPackage {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage";
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    Properties: usize,
    pub RequestedOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataPackageOperation) -> windows_core::HRESULT,
    pub SetRequestedOperation: unsafe extern "system" fn(*mut core::ffi::c_void, DataPackageOperation) -> windows_core::HRESULT,
    pub OperationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveOperationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Destroyed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    SetDataProvider: usize,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHtmlFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ResourceMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResourceMap: usize,
    pub SetRtf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBitmap: usize,
    #[cfg(feature = "Storage")]
    pub SetStorageItemsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetStorageItemsReadOnly: usize,
    #[cfg(feature = "Storage")]
    pub SetStorageItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetStorageItems: usize,
}
windows_core::imp::define_interface!(IDataPackage2, IDataPackage2_Vtbl, 0x041c1fe9_2409_45e1_a538_4c53eeee04a7);
impl windows_core::RuntimeType for IDataPackage2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IDataPackage2");
}
impl windows_core::RuntimeName for IDataPackage2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage2";
}
pub trait IDataPackage2_Impl: windows_core::IUnknownImpl {
    fn SetApplicationLink(&self, value: windows_core::Ref<super::super::Foundation::Uri>) -> windows_core::Result<()>;
    fn SetWebLink(&self, value: windows_core::Ref<super::super::Foundation::Uri>) -> windows_core::Result<()>;
}
impl IDataPackage2_Vtbl {
    pub const fn new<Identity: IDataPackage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetApplicationLink<Identity: IDataPackage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataPackage2_Impl::SetApplicationLink(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetWebLink<Identity: IDataPackage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataPackage2_Impl::SetWebLink(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataPackage2, OFFSET>(),
            SetApplicationLink: SetApplicationLink::<Identity, OFFSET>,
            SetWebLink: SetWebLink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataPackage2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetApplicationLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWebLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataPackage3, IDataPackage3_Vtbl, 0x88f31f5d_787b_4d32_965a_a9838105a056);
impl windows_core::RuntimeType for IDataPackage3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IDataPackage3");
}
impl windows_core::RuntimeName for IDataPackage3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage3";
}
pub trait IDataPackage3_Impl: windows_core::IUnknownImpl {
    fn ShareCompleted(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<DataPackage, ShareCompletedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveShareCompleted(&self, token: i64) -> windows_core::Result<()>;
}
impl IDataPackage3_Vtbl {
    pub const fn new<Identity: IDataPackage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShareCompleted<Identity: IDataPackage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataPackage3_Impl::ShareCompleted(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveShareCompleted<Identity: IDataPackage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataPackage3_Impl::RemoveShareCompleted(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataPackage3, OFFSET>(),
            ShareCompleted: ShareCompleted::<Identity, OFFSET>,
            RemoveShareCompleted: RemoveShareCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataPackage3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShareCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveShareCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataPackage4, IDataPackage4_Vtbl, 0x13a24ec8_9382_536f_852a_3045e1b29a3b);
impl windows_core::RuntimeType for IDataPackage4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IDataPackage4");
}
impl windows_core::RuntimeName for IDataPackage4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage4";
}
pub trait IDataPackage4_Impl: windows_core::IUnknownImpl {
    fn ShareCanceled(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<DataPackage, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveShareCanceled(&self, token: i64) -> windows_core::Result<()>;
}
impl IDataPackage4_Vtbl {
    pub const fn new<Identity: IDataPackage4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShareCanceled<Identity: IDataPackage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataPackage4_Impl::ShareCanceled(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveShareCanceled<Identity: IDataPackage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataPackage4_Impl::RemoveShareCanceled(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataPackage4, OFFSET>(),
            ShareCanceled: ShareCanceled::<Identity, OFFSET>,
            RemoveShareCanceled: RemoveShareCanceled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataPackage4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackage4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShareCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveShareCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataPackageView, IDataPackageView_Vtbl, 0x7b840471_5900_4d85_a90b_10cb85fe3552);
impl windows_core::RuntimeType for IDataPackageView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IDataPackageView");
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IDataPackageView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView";
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Properties: usize,
    pub RequestedOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataPackageOperation) -> windows_core::HRESULT,
    pub ReportOperationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, DataPackageOperation) -> windows_core::HRESULT,
    pub AvailableFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Contains: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCustomTextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHtmlFormatAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetResourceMapAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetResourceMapAsync: usize,
    pub GetRtfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetBitmapAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetBitmapAsync: usize,
    #[cfg(feature = "Storage")]
    pub GetStorageItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GetStorageItemsAsync: usize,
}
windows_core::imp::define_interface!(IDataPackageView2, IDataPackageView2_Vtbl, 0x40ecba95_2450_4c1d_b6b4_ed45463dee9c);
impl windows_core::RuntimeType for IDataPackageView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IDataPackageView2");
}
impl windows_core::RuntimeName for IDataPackageView2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView2";
}
pub trait IDataPackageView2_Impl: windows_core::IUnknownImpl {
    fn GetApplicationLinkAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetWebLinkAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Foundation::Uri>>;
}
impl IDataPackageView2_Vtbl {
    pub const fn new<Identity: IDataPackageView2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetApplicationLinkAsync<Identity: IDataPackageView2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataPackageView2_Impl::GetApplicationLinkAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWebLinkAsync<Identity: IDataPackageView2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataPackageView2_Impl::GetWebLinkAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataPackageView2, OFFSET>(),
            GetApplicationLinkAsync: GetApplicationLinkAsync::<Identity, OFFSET>,
            GetWebLinkAsync: GetWebLinkAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataPackageView2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetApplicationLinkAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWebLinkAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataPackageView3, IDataPackageView3_Vtbl, 0xd37771a8_ddad_4288_8428_d1cae394128b);
impl windows_core::RuntimeType for IDataPackageView3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IDataPackageView3");
}
#[cfg(feature = "Security_EnterpriseData")]
impl windows_core::RuntimeName for IDataPackageView3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView3";
}
#[cfg(feature = "Security_EnterpriseData")]
pub trait IDataPackageView3_Impl: windows_core::IUnknownImpl {
    fn RequestAccessAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>;
    fn RequestAccessWithEnterpriseIdAsync(&self, enterpriseId: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>;
    fn UnlockAndAssumeEnterpriseIdentity(&self) -> windows_core::Result<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>;
}
#[cfg(feature = "Security_EnterpriseData")]
impl IDataPackageView3_Vtbl {
    pub const fn new<Identity: IDataPackageView3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestAccessAsync<Identity: IDataPackageView3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataPackageView3_Impl::RequestAccessAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestAccessWithEnterpriseIdAsync<Identity: IDataPackageView3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enterpriseid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataPackageView3_Impl::RequestAccessWithEnterpriseIdAsync(this, core::mem::transmute(&enterpriseid)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnlockAndAssumeEnterpriseIdentity<Identity: IDataPackageView3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataPackageView3_Impl::UnlockAndAssumeEnterpriseIdentity(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataPackageView3, OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Identity, OFFSET>,
            RequestAccessWithEnterpriseIdAsync: RequestAccessWithEnterpriseIdAsync::<Identity, OFFSET>,
            UnlockAndAssumeEnterpriseIdentity: UnlockAndAssumeEnterpriseIdentity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataPackageView3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_EnterpriseData")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_EnterpriseData"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Security_EnterpriseData")]
    pub RequestAccessWithEnterpriseIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_EnterpriseData"))]
    RequestAccessWithEnterpriseIdAsync: usize,
    #[cfg(feature = "Security_EnterpriseData")]
    pub UnlockAndAssumeEnterpriseIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_EnterpriseData"))]
    UnlockAndAssumeEnterpriseIdentity: usize,
}
windows_core::imp::define_interface!(IDataPackageView4, IDataPackageView4_Vtbl, 0xdfe96f1f_e042_4433_a09f_26d6ffda8b85);
impl windows_core::RuntimeType for IDataPackageView4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IDataPackageView4");
}
impl windows_core::RuntimeName for IDataPackageView4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView4";
}
pub trait IDataPackageView4_Impl: windows_core::IUnknownImpl {
    fn SetAcceptedFormatId(&self, formatId: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IDataPackageView4_Vtbl {
    pub const fn new<Identity: IDataPackageView4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAcceptedFormatId<Identity: IDataPackageView4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formatid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataPackageView4_Impl::SetAcceptedFormatId(this, core::mem::transmute(&formatid)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataPackageView4, OFFSET>(),
            SetAcceptedFormatId: SetAcceptedFormatId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataPackageView4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPackageView4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAcceptedFormatId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOperationCompletedEventArgs, IOperationCompletedEventArgs_Vtbl, 0xe7af329d_051d_4fab_b1a9_47fd77f70a41);
impl windows_core::RuntimeType for IOperationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs");
}
impl windows_core::RuntimeName for IOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs";
}
pub trait IOperationCompletedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Operation(&self) -> windows_core::Result<DataPackageOperation>;
}
impl IOperationCompletedEventArgs_Vtbl {
    pub const fn new<Identity: IOperationCompletedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Operation<Identity: IOperationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut DataPackageOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOperationCompletedEventArgs_Impl::Operation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IOperationCompletedEventArgs, OFFSET>(), Operation: Operation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOperationCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOperationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataPackageOperation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOperationCompletedEventArgs2, IOperationCompletedEventArgs2_Vtbl, 0x858fa073_1e19_4105_b2f7_c8478808d562);
impl windows_core::RuntimeType for IOperationCompletedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2");
}
impl windows_core::RuntimeName for IOperationCompletedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2";
}
pub trait IOperationCompletedEventArgs2_Impl: windows_core::IUnknownImpl {
    fn AcceptedFormatId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IOperationCompletedEventArgs2_Vtbl {
    pub const fn new<Identity: IOperationCompletedEventArgs2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcceptedFormatId<Identity: IOperationCompletedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOperationCompletedEventArgs2_Impl::AcceptedFormatId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IOperationCompletedEventArgs2, OFFSET>(),
            AcceptedFormatId: AcceptedFormatId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOperationCompletedEventArgs2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOperationCompletedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AcceptedFormatId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShareCompletedEventArgs, IShareCompletedEventArgs_Vtbl, 0x4574c442_f913_4f60_9df7_cc4060ab1916);
impl windows_core::RuntimeType for IShareCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs");
}
impl windows_core::RuntimeName for IShareCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OperationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl OperationCompletedEventArgs {
    pub fn Operation(&self) -> windows_core::Result<DataPackageOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Operation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn AcceptedFormatId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IOperationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptedFormatId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for OperationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOperationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for OperationCompletedEventArgs {
    type Vtable = <IOperationCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOperationCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OperationCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs";
}
unsafe impl Send for OperationCompletedEventArgs {}
unsafe impl Sync for OperationCompletedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ShareCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for ShareCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IShareCompletedEventArgs>();
}
unsafe impl windows_core::Interface for ShareCompletedEventArgs {
    type Vtable = <IShareCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IShareCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ShareCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs";
}
unsafe impl Send for ShareCompletedEventArgs {}
unsafe impl Sync for ShareCompletedEventArgs {}
