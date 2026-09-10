#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: Self = Self(0);
    pub const Blocked: Self = Self(1);
    pub const ConsentRequired: Self = Self(2);
}
impl windows_core::imp::TypeKind for ProtectionPolicyEvaluationResult {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ProtectionPolicyEvaluationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult");
}
