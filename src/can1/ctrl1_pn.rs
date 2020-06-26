#[doc = "Reader of register CTRL1_PN"]
pub type R = crate::R<u32, super::CTRL1_PN>;
#[doc = "Writer for register CTRL1_PN"]
pub type W = crate::W<u32, super::CTRL1_PN>;
#[doc = "Register CTRL1_PN `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::CTRL1_PN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Filtering Combination Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCS_A {
    #[doc = "0: Message ID filtering only"]
    _00 = 0,
    #[doc = "1: Message ID filtering and payload filtering"]
    _01 = 1,
    #[doc = "2: Message ID filtering occurring a specified number of times."]
    _10 = 2,
    #[doc = "3: Message ID filtering and payload filtering a specified number of times"]
    _11 = 3,
}
impl From<FCS_A> for u8 {
    #[inline(always)]
    fn from(variant: FCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FCS`"]
pub type FCS_R = crate::R<u8, FCS_A>;
impl FCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCS_A {
        match self.bits {
            0 => FCS_A::_00,
            1 => FCS_A::_01,
            2 => FCS_A::_10,
            3 => FCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FCS_A::_11
    }
}
#[doc = "Write proxy for field `FCS`"]
pub struct FCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Message ID filtering only"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FCS_A::_00)
    }
    #[doc = "Message ID filtering and payload filtering"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FCS_A::_01)
    }
    #[doc = "Message ID filtering occurring a specified number of times."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FCS_A::_10)
    }
    #[doc = "Message ID filtering and payload filtering a specified number of times"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FCS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "ID Filtering Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDFS_A {
    #[doc = "0: Match upon a ID contents against an exact target value"]
    _00 = 0,
    #[doc = "1: Match upon a ID value greater than or equal to a specified target value"]
    _01 = 1,
    #[doc = "2: Match upon a ID value smaller than or equal to a specified target value"]
    _10 = 2,
    #[doc = "3: Match upon a ID value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    _11 = 3,
}
impl From<IDFS_A> for u8 {
    #[inline(always)]
    fn from(variant: IDFS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IDFS`"]
pub type IDFS_R = crate::R<u8, IDFS_A>;
impl IDFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDFS_A {
        match self.bits {
            0 => IDFS_A::_00,
            1 => IDFS_A::_01,
            2 => IDFS_A::_10,
            3 => IDFS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDFS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDFS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDFS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDFS_A::_11
    }
}
#[doc = "Write proxy for field `IDFS`"]
pub struct IDFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDFS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Match upon a ID contents against an exact target value"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IDFS_A::_00)
    }
    #[doc = "Match upon a ID value greater than or equal to a specified target value"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IDFS_A::_01)
    }
    #[doc = "Match upon a ID value smaller than or equal to a specified target value"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IDFS_A::_10)
    }
    #[doc = "Match upon a ID value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IDFS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Payload Filtering Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLFS_A {
    #[doc = "0: Match upon a payload contents against an exact target value"]
    _00 = 0,
    #[doc = "1: Match upon a payload value greater than or equal to a specified target value"]
    _01 = 1,
    #[doc = "2: Match upon a payload value smaller than or equal to a specified target value"]
    _10 = 2,
    #[doc = "3: Match upon a payload value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    _11 = 3,
}
impl From<PLFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PLFS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLFS`"]
pub type PLFS_R = crate::R<u8, PLFS_A>;
impl PLFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLFS_A {
        match self.bits {
            0 => PLFS_A::_00,
            1 => PLFS_A::_01,
            2 => PLFS_A::_10,
            3 => PLFS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLFS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLFS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PLFS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLFS_A::_11
    }
}
#[doc = "Write proxy for field `PLFS`"]
pub struct PLFS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLFS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Match upon a payload contents against an exact target value"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLFS_A::_00)
    }
    #[doc = "Match upon a payload value greater than or equal to a specified target value"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLFS_A::_01)
    }
    #[doc = "Match upon a payload value smaller than or equal to a specified target value"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PLFS_A::_10)
    }
    #[doc = "Match upon a payload value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLFS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Number of Messages Matching the Same Filtering Criteria\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NMATCH_A {
    #[doc = "1: Received message must match the predefined filtering criteria for ID and/or PL once before generating a wake up event."]
    _00000001 = 1,
    #[doc = "2: Received message must match the predefined filtering criteria for ID and/or PL twice before generating a wake up event."]
    _00000010 = 2,
    #[doc = "255: Received message must match the predefined filtering criteria for ID and/or PL 255 times before generating a wake up event."]
    _11111111 = 255,
}
impl From<NMATCH_A> for u8 {
    #[inline(always)]
    fn from(variant: NMATCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NMATCH`"]
pub type NMATCH_R = crate::R<u8, NMATCH_A>;
impl NMATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NMATCH_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(NMATCH_A::_00000001),
            2 => Val(NMATCH_A::_00000010),
            255 => Val(NMATCH_A::_11111111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline(always)]
    pub fn is_00000001(&self) -> bool {
        *self == NMATCH_A::_00000001
    }
    #[doc = "Checks if the value of the field is `_00000010`"]
    #[inline(always)]
    pub fn is_00000010(&self) -> bool {
        *self == NMATCH_A::_00000010
    }
    #[doc = "Checks if the value of the field is `_11111111`"]
    #[inline(always)]
    pub fn is_11111111(&self) -> bool {
        *self == NMATCH_A::_11111111
    }
}
#[doc = "Write proxy for field `NMATCH`"]
pub struct NMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> NMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMATCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL once before generating a wake up event."]
    #[inline(always)]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(NMATCH_A::_00000001)
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL twice before generating a wake up event."]
    #[inline(always)]
    pub fn _00000010(self) -> &'a mut W {
        self.variant(NMATCH_A::_00000010)
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL 255 times before generating a wake up event."]
    #[inline(always)]
    pub fn _11111111(self) -> &'a mut W {
        self.variant(NMATCH_A::_11111111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Wake Up by Match Flag Mask Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUMF_MSK_A {
    #[doc = "0: Wake up match event is disabled"]
    _0 = 0,
    #[doc = "1: Wake up match event is enabled"]
    _1 = 1,
}
impl From<WUMF_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: WUMF_MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WUMF_MSK`"]
pub type WUMF_MSK_R = crate::R<bool, WUMF_MSK_A>;
impl WUMF_MSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUMF_MSK_A {
        match self.bits {
            false => WUMF_MSK_A::_0,
            true => WUMF_MSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUMF_MSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUMF_MSK_A::_1
    }
}
#[doc = "Write proxy for field `WUMF_MSK`"]
pub struct WUMF_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WUMF_MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUMF_MSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake up match event is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUMF_MSK_A::_0)
    }
    #[doc = "Wake up match event is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUMF_MSK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Wake Up by Timeout Flag Mask Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTOF_MSK_A {
    #[doc = "0: Timeout wake up event is disabled"]
    _0 = 0,
    #[doc = "1: Timeout wake up event is enabled"]
    _1 = 1,
}
impl From<WTOF_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: WTOF_MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTOF_MSK`"]
pub type WTOF_MSK_R = crate::R<bool, WTOF_MSK_A>;
impl WTOF_MSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTOF_MSK_A {
        match self.bits {
            false => WTOF_MSK_A::_0,
            true => WTOF_MSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WTOF_MSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WTOF_MSK_A::_1
    }
}
#[doc = "Write proxy for field `WTOF_MSK`"]
pub struct WTOF_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WTOF_MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTOF_MSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timeout wake up event is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTOF_MSK_A::_0)
    }
    #[doc = "Timeout wake up event is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTOF_MSK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Filtering Combination Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ID Filtering Selection"]
    #[inline(always)]
    pub fn idfs(&self) -> IDFS_R {
        IDFS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Payload Filtering Selection"]
    #[inline(always)]
    pub fn plfs(&self) -> PLFS_R {
        PLFS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Number of Messages Matching the Same Filtering Criteria"]
    #[inline(always)]
    pub fn nmatch(&self) -> NMATCH_R {
        NMATCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Mask Bit"]
    #[inline(always)]
    pub fn wumf_msk(&self) -> WUMF_MSK_R {
        WUMF_MSK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Mask Bit"]
    #[inline(always)]
    pub fn wtof_msk(&self) -> WTOF_MSK_R {
        WTOF_MSK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filtering Combination Selection"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W {
        FCS_W { w: self }
    }
    #[doc = "Bits 2:3 - ID Filtering Selection"]
    #[inline(always)]
    pub fn idfs(&mut self) -> IDFS_W {
        IDFS_W { w: self }
    }
    #[doc = "Bits 4:5 - Payload Filtering Selection"]
    #[inline(always)]
    pub fn plfs(&mut self) -> PLFS_W {
        PLFS_W { w: self }
    }
    #[doc = "Bits 8:15 - Number of Messages Matching the Same Filtering Criteria"]
    #[inline(always)]
    pub fn nmatch(&mut self) -> NMATCH_W {
        NMATCH_W { w: self }
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Mask Bit"]
    #[inline(always)]
    pub fn wumf_msk(&mut self) -> WUMF_MSK_W {
        WUMF_MSK_W { w: self }
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Mask Bit"]
    #[inline(always)]
    pub fn wtof_msk(&mut self) -> WTOF_MSK_W {
        WTOF_MSK_W { w: self }
    }
}
