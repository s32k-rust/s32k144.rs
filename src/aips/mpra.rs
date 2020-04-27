#[doc = "Reader of register MPRA"]
pub type R = crate::R<u32, super::MPRA>;
#[doc = "Writer for register MPRA"]
pub type W = crate::W<u32, super::MPRA>;
#[doc = "Register MPRA `reset()`'s with value 0x7770_0000"]
impl crate::ResetValue for super::MPRA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7770_0000
    }
}
#[doc = "Master 2 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL2_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL2_A> for bool {
    #[inline(always)]
    fn from(variant: MPL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPL2`"]
pub type MPL2_R = crate::R<bool, MPL2_A>;
impl MPL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL2_A {
        match self.bits {
            false => MPL2_A::_0,
            true => MPL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL2_A::_1
    }
}
#[doc = "Write proxy for field `MPL2`"]
pub struct MPL2_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL2_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Master 2 Trusted For Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW2_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW2_A> for bool {
    #[inline(always)]
    fn from(variant: MTW2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTW2`"]
pub type MTW2_R = crate::R<bool, MTW2_A>;
impl MTW2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW2_A {
        match self.bits {
            false => MTW2_A::_0,
            true => MTW2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW2_A::_1
    }
}
#[doc = "Write proxy for field `MTW2`"]
pub struct MTW2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW2_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Master 2 Trusted For Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR2_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR2_A> for bool {
    #[inline(always)]
    fn from(variant: MTR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTR2`"]
pub type MTR2_R = crate::R<bool, MTR2_A>;
impl MTR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR2_A {
        match self.bits {
            false => MTR2_A::_0,
            true => MTR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR2_A::_1
    }
}
#[doc = "Write proxy for field `MTR2`"]
pub struct MTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR2_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Master 1 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL1_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL1_A> for bool {
    #[inline(always)]
    fn from(variant: MPL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPL1`"]
pub type MPL1_R = crate::R<bool, MPL1_A>;
impl MPL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL1_A {
        match self.bits {
            false => MPL1_A::_0,
            true => MPL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL1_A::_1
    }
}
#[doc = "Write proxy for field `MPL1`"]
pub struct MPL1_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL1_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL1_A::_1)
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
#[doc = "Master 1 Trusted for Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW1_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW1_A> for bool {
    #[inline(always)]
    fn from(variant: MTW1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTW1`"]
pub type MTW1_R = crate::R<bool, MTW1_A>;
impl MTW1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW1_A {
        match self.bits {
            false => MTW1_A::_0,
            true => MTW1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW1_A::_1
    }
}
#[doc = "Write proxy for field `MTW1`"]
pub struct MTW1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW1_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW1_A::_1)
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
#[doc = "Master 1 Trusted for Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR1_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR1_A> for bool {
    #[inline(always)]
    fn from(variant: MTR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTR1`"]
pub type MTR1_R = crate::R<bool, MTR1_A>;
impl MTR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR1_A {
        match self.bits {
            false => MTR1_A::_0,
            true => MTR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR1_A::_1
    }
}
#[doc = "Write proxy for field `MTR1`"]
pub struct MTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR1_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR1_A::_1)
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
#[doc = "Master 0 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL0_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL0_A> for bool {
    #[inline(always)]
    fn from(variant: MPL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPL0`"]
pub type MPL0_R = crate::R<bool, MPL0_A>;
impl MPL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL0_A {
        match self.bits {
            false => MPL0_A::_0,
            true => MPL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL0_A::_1
    }
}
#[doc = "Write proxy for field `MPL0`"]
pub struct MPL0_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL0_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL0_A::_1)
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
#[doc = "Master 0 Trusted For Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW0_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW0_A> for bool {
    #[inline(always)]
    fn from(variant: MTW0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTW0`"]
pub type MTW0_R = crate::R<bool, MTW0_A>;
impl MTW0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW0_A {
        match self.bits {
            false => MTW0_A::_0,
            true => MTW0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW0_A::_1
    }
}
#[doc = "Write proxy for field `MTW0`"]
pub struct MTW0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW0_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW0_A::_1)
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
#[doc = "Master 0 Trusted For Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR0_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR0_A> for bool {
    #[inline(always)]
    fn from(variant: MTR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTR0`"]
pub type MTR0_R = crate::R<bool, MTR0_A>;
impl MTR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR0_A {
        match self.bits {
            false => MTR0_A::_0,
            true => MTR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR0_A::_1
    }
}
#[doc = "Write proxy for field `MTR0`"]
pub struct MTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR0_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR0_A::_1)
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
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline(always)]
    pub fn mpl2(&self) -> MPL2_R {
        MPL2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw2(&self) -> MTW2_R {
        MTW2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline(always)]
    pub fn mtr2(&self) -> MTR2_R {
        MTR2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline(always)]
    pub fn mpl1(&self) -> MPL1_R {
        MPL1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw1(&self) -> MTW1_R {
        MTW1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline(always)]
    pub fn mtr1(&self) -> MTR1_R {
        MTR1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline(always)]
    pub fn mpl0(&self) -> MPL0_R {
        MPL0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw0(&self) -> MTW0_R {
        MTW0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline(always)]
    pub fn mtr0(&self) -> MTR0_R {
        MTR0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline(always)]
    pub fn mpl2(&mut self) -> MPL2_W {
        MPL2_W { w: self }
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw2(&mut self) -> MTW2_W {
        MTW2_W { w: self }
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline(always)]
    pub fn mtr2(&mut self) -> MTR2_W {
        MTR2_W { w: self }
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline(always)]
    pub fn mpl1(&mut self) -> MPL1_W {
        MPL1_W { w: self }
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw1(&mut self) -> MTW1_W {
        MTW1_W { w: self }
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline(always)]
    pub fn mtr1(&mut self) -> MTR1_W {
        MTR1_W { w: self }
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline(always)]
    pub fn mpl0(&mut self) -> MPL0_W {
        MPL0_W { w: self }
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw0(&mut self) -> MTW0_W {
        MTW0_W { w: self }
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline(always)]
    pub fn mtr0(&mut self) -> MTR0_W {
        MTR0_W { w: self }
    }
}
