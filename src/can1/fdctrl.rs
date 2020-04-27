#[doc = "Reader of register FDCTRL"]
pub type R = crate::R<u32, super::FDCTRL>;
#[doc = "Writer for register FDCTRL"]
pub type W = crate::W<u32, super::FDCTRL>;
#[doc = "Register FDCTRL `reset()`'s with value 0x8000_0100"]
impl crate::ResetValue for super::FDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0100
    }
}
#[doc = "Reader of field `TDCVAL`"]
pub type TDCVAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TDCOFF`"]
pub type TDCOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCOFF`"]
pub struct TDCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Transceiver Delay Compensation Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDCFAIL_A {
    #[doc = "0: Measured loop delay is in range."]
    _0 = 0,
    #[doc = "1: Measured loop delay is out of range."]
    _1 = 1,
}
impl From<TDCFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: TDCFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDCFAIL`"]
pub type TDCFAIL_R = crate::R<bool, TDCFAIL_A>;
impl TDCFAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCFAIL_A {
        match self.bits {
            false => TDCFAIL_A::_0,
            true => TDCFAIL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDCFAIL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDCFAIL_A::_1
    }
}
#[doc = "Write proxy for field `TDCFAIL`"]
pub struct TDCFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCFAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCFAIL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Measured loop delay is in range."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDCFAIL_A::_0)
    }
    #[doc = "Measured loop delay is out of range."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDCFAIL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Transceiver Delay Compensation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDCEN_A {
    #[doc = "0: TDC is disabled"]
    _0 = 0,
    #[doc = "1: TDC is enabled"]
    _1 = 1,
}
impl From<TDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: TDCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDCEN`"]
pub type TDCEN_R = crate::R<bool, TDCEN_A>;
impl TDCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCEN_A {
        match self.bits {
            false => TDCEN_A::_0,
            true => TDCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDCEN_A::_1
    }
}
#[doc = "Write proxy for field `TDCEN`"]
pub struct TDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TDC is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDCEN_A::_0)
    }
    #[doc = "TDC is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDCEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Message Buffer Data Size for Region 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MBDSR0_A {
    #[doc = "0: Selects 8 bytes per Message Buffer."]
    _00 = 0,
    #[doc = "1: Selects 16 bytes per Message Buffer."]
    _01 = 1,
    #[doc = "2: Selects 32 bytes per Message Buffer."]
    _10 = 2,
    #[doc = "3: Selects 64 bytes per Message Buffer."]
    _11 = 3,
}
impl From<MBDSR0_A> for u8 {
    #[inline(always)]
    fn from(variant: MBDSR0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MBDSR0`"]
pub type MBDSR0_R = crate::R<u8, MBDSR0_A>;
impl MBDSR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBDSR0_A {
        match self.bits {
            0 => MBDSR0_A::_00,
            1 => MBDSR0_A::_01,
            2 => MBDSR0_A::_10,
            3 => MBDSR0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MBDSR0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MBDSR0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MBDSR0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MBDSR0_A::_11
    }
}
#[doc = "Write proxy for field `MBDSR0`"]
pub struct MBDSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MBDSR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBDSR0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects 8 bytes per Message Buffer."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MBDSR0_A::_00)
    }
    #[doc = "Selects 16 bytes per Message Buffer."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MBDSR0_A::_01)
    }
    #[doc = "Selects 32 bytes per Message Buffer."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MBDSR0_A::_10)
    }
    #[doc = "Selects 64 bytes per Message Buffer."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MBDSR0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Bit Rate Switch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDRATE_A {
    #[doc = "0: Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    _0 = 0,
    #[doc = "1: Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    _1 = 1,
}
impl From<FDRATE_A> for bool {
    #[inline(always)]
    fn from(variant: FDRATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDRATE`"]
pub type FDRATE_R = crate::R<bool, FDRATE_A>;
impl FDRATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDRATE_A {
        match self.bits {
            false => FDRATE_A::_0,
            true => FDRATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDRATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDRATE_A::_1
    }
}
#[doc = "Write proxy for field `FDRATE`"]
pub struct FDRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDRATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDRATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDRATE_A::_0)
    }
    #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDRATE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Transceiver Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcval(&self) -> TDCVAL_R {
        TDCVAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdcoff(&self) -> TDCOFF_R {
        TDCOFF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Transceiver Delay Compensation Fail"]
    #[inline(always)]
    pub fn tdcfail(&self) -> TDCFAIL_R {
        TDCFAIL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Message Buffer Data Size for Region 0"]
    #[inline(always)]
    pub fn mbdsr0(&self) -> MBDSR0_R {
        MBDSR0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn fdrate(&self) -> FDRATE_R {
        FDRATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdcoff(&mut self) -> TDCOFF_W {
        TDCOFF_W { w: self }
    }
    #[doc = "Bit 14 - Transceiver Delay Compensation Fail"]
    #[inline(always)]
    pub fn tdcfail(&mut self) -> TDCFAIL_W {
        TDCFAIL_W { w: self }
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub fn tdcen(&mut self) -> TDCEN_W {
        TDCEN_W { w: self }
    }
    #[doc = "Bits 16:17 - Message Buffer Data Size for Region 0"]
    #[inline(always)]
    pub fn mbdsr0(&mut self) -> MBDSR0_W {
        MBDSR0_W { w: self }
    }
    #[doc = "Bit 31 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn fdrate(&mut self) -> FDRATE_W {
        FDRATE_W { w: self }
    }
}
