#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontStretch(pub i32);
impl FontStretch {
    pub const Undefined: Self = Self(0);
    pub const UltraCondensed: Self = Self(1);
    pub const ExtraCondensed: Self = Self(2);
    pub const Condensed: Self = Self(3);
    pub const SemiCondensed: Self = Self(4);
    pub const Normal: Self = Self(5);
    pub const SemiExpanded: Self = Self(6);
    pub const Expanded: Self = Self(7);
    pub const ExtraExpanded: Self = Self(8);
    pub const UltraExpanded: Self = Self(9);
}
impl windows_core::imp::TypeKind for FontStretch {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FontStretch {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStretch;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Text.FontStretch");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontStyle(pub i32);
impl FontStyle {
    pub const Normal: Self = Self(0);
    pub const Oblique: Self = Self(1);
    pub const Italic: Self = Self(2);
}
impl windows_core::imp::TypeKind for FontStyle {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FontStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStyle;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Text.FontStyle");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontWeight {
    pub Weight: u16,
}
impl windows_core::imp::TypeKind for FontWeight {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FontWeight {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Text.FontWeight;u2)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Text.FontWeight");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextDecorations(pub u32);
impl TextDecorations {
    pub const None: Self = Self(0);
    pub const Underline: Self = Self(1);
    pub const Strikethrough: Self = Self(2);
}
impl windows_core::imp::TypeKind for TextDecorations {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TextDecorations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextDecorations;u4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Text.TextDecorations");
}
impl TextDecorations {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextDecorations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextDecorations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextDecorations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for TextDecorations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for TextDecorations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
