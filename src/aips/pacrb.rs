#[doc = "Reader of register PACRB"]
pub type R = crate::R<u32, super::PACRB>;
#[doc = "Writer for register PACRB"]
pub type W = crate::W<u32, super::PACRB>;
#[doc = "Register PACRB `reset()`'s with value 0x4400_0400"]
impl crate::ResetValue for super::PACRB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4400_0400
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP5_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP5_A> for bool {
    #[inline(always)]
    fn from(variant: TP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TP5`"]
pub type TP5_R = crate::R<bool, TP5_A>;
impl TP5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP5_A {
        match self.bits {
            false => TP5_A::_0,
            true => TP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP5_A::_1
    }
}
#[doc = "Write proxy for field `TP5`"]
pub struct TP5_W<'a> {
    w: &'a mut W,
}
impl<'a> TP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP5_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP5_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP5_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP5_A> for bool {
    #[inline(always)]
    fn from(variant: WP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WP5`"]
pub type WP5_R = crate::R<bool, WP5_A>;
impl WP5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP5_A {
        match self.bits {
            false => WP5_A::_0,
            true => WP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP5_A::_1
    }
}
#[doc = "Write proxy for field `WP5`"]
pub struct WP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP5_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP5_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP5_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP5_A> for bool {
    #[inline(always)]
    fn from(variant: SP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SP5`"]
pub type SP5_R = crate::R<bool, SP5_A>;
impl SP5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP5_A {
        match self.bits {
            false => SP5_A::_0,
            true => SP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP5_A::_1
    }
}
#[doc = "Write proxy for field `SP5`"]
pub struct SP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP5_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP5_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP1_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP1_A> for bool {
    #[inline(always)]
    fn from(variant: TP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TP1`"]
pub type TP1_R = crate::R<bool, TP1_A>;
impl TP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP1_A {
        match self.bits {
            false => TP1_A::_0,
            true => TP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP1_A::_1
    }
}
#[doc = "Write proxy for field `TP1`"]
pub struct TP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP1_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP1_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP1_A> for bool {
    #[inline(always)]
    fn from(variant: WP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WP1`"]
pub type WP1_R = crate::R<bool, WP1_A>;
impl WP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP1_A {
        match self.bits {
            false => WP1_A::_0,
            true => WP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP1_A::_1
    }
}
#[doc = "Write proxy for field `WP1`"]
pub struct WP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP1_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP1_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP1_A> for bool {
    #[inline(always)]
    fn from(variant: SP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SP1`"]
pub type SP1_R = crate::R<bool, SP1_A>;
impl SP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP1_A {
        match self.bits {
            false => SP1_A::_0,
            true => SP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP1_A::_1
    }
}
#[doc = "Write proxy for field `SP1`"]
pub struct SP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP1_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP0_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP0_A> for bool {
    #[inline(always)]
    fn from(variant: TP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TP0`"]
pub type TP0_R = crate::R<bool, TP0_A>;
impl TP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP0_A {
        match self.bits {
            false => TP0_A::_0,
            true => TP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP0_A::_1
    }
}
#[doc = "Write proxy for field `TP0`"]
pub struct TP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP0_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP0_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP0_A> for bool {
    #[inline(always)]
    fn from(variant: WP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WP0`"]
pub type WP0_R = crate::R<bool, WP0_A>;
impl WP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP0_A {
        match self.bits {
            false => WP0_A::_0,
            true => WP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP0_A::_1
    }
}
#[doc = "Write proxy for field `WP0`"]
pub struct WP0_W<'a> {
    w: &'a mut W,
}
impl<'a> WP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP0_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP0_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP0_A> for bool {
    #[inline(always)]
    fn from(variant: SP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SP0`"]
pub type SP0_R = crate::R<bool, SP0_A>;
impl SP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP0_A {
        match self.bits {
            false => SP0_A::_0,
            true => SP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP0_A::_1
    }
}
#[doc = "Write proxy for field `SP0`"]
pub struct SP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP0_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline(always)]
    pub fn tp5(&self) -> TP5_R {
        TP5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp5(&self) -> SP5_R {
        SP5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline(always)]
    pub fn tp1(&self) -> TP1_R {
        TP1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp1(&self) -> SP1_R {
        SP1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline(always)]
    pub fn tp0(&self) -> TP0_R {
        TP0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline(always)]
    pub fn wp0(&self) -> WP0_R {
        WP0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp0(&self) -> SP0_R {
        SP0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline(always)]
    pub fn tp5(&mut self) -> TP5_W {
        TP5_W { w: self }
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline(always)]
    pub fn wp5(&mut self) -> WP5_W {
        WP5_W { w: self }
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp5(&mut self) -> SP5_W {
        SP5_W { w: self }
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline(always)]
    pub fn tp1(&mut self) -> TP1_W {
        TP1_W { w: self }
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline(always)]
    pub fn wp1(&mut self) -> WP1_W {
        WP1_W { w: self }
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp1(&mut self) -> SP1_W {
        SP1_W { w: self }
    }
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline(always)]
    pub fn tp0(&mut self) -> TP0_W {
        TP0_W { w: self }
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline(always)]
    pub fn wp0(&mut self) -> WP0_W {
        WP0_W { w: self }
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp0(&mut self) -> SP0_W {
        SP0_W { w: self }
    }
}
