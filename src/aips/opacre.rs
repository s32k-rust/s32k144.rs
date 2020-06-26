#[doc = "Reader of register OPACRE"]
pub type R = crate::R<u32, super::OPACRE>;
#[doc = "Writer for register OPACRE"]
pub type W = crate::W<u32, super::OPACRE>;
#[doc = "Register OPACRE `reset()`'s with value 0x4000_0040"]
impl crate::ResetValue for super::OPACRE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0040
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP6_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP6_A> for bool {
    #[inline(always)]
    fn from(variant: TP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TP6`"]
pub type TP6_R = crate::R<bool, TP6_A>;
impl TP6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP6_A {
        match self.bits {
            false => TP6_A::_0,
            true => TP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP6_A::_1
    }
}
#[doc = "Write proxy for field `TP6`"]
pub struct TP6_W<'a> {
    w: &'a mut W,
}
impl<'a> TP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP6_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP6_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP6_A> for bool {
    #[inline(always)]
    fn from(variant: WP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WP6`"]
pub type WP6_R = crate::R<bool, WP6_A>;
impl WP6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP6_A {
        match self.bits {
            false => WP6_A::_0,
            true => WP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP6_A::_1
    }
}
#[doc = "Write proxy for field `WP6`"]
pub struct WP6_W<'a> {
    w: &'a mut W,
}
impl<'a> WP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP6_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP6_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP6_A> for bool {
    #[inline(always)]
    fn from(variant: SP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SP6`"]
pub type SP6_R = crate::R<bool, SP6_A>;
impl SP6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP6_A {
        match self.bits {
            false => SP6_A::_0,
            true => SP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP6_A::_1
    }
}
#[doc = "Write proxy for field `SP6`"]
pub struct SP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP6_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline(always)]
    pub fn tp6(&self) -> TP6_R {
        TP6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline(always)]
    pub fn wp6(&self) -> WP6_R {
        WP6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp6(&self) -> SP6_R {
        SP6_R::new(((self.bits >> 6) & 0x01) != 0)
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
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline(always)]
    pub fn tp6(&mut self) -> TP6_W {
        TP6_W { w: self }
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline(always)]
    pub fn wp6(&mut self) -> WP6_W {
        WP6_W { w: self }
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp6(&mut self) -> SP6_W {
        SP6_W { w: self }
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
