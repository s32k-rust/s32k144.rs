#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `FRAMESZ`"]
pub type FRAMESZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRAMESZ`"]
pub struct FRAMESZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Transfer Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WIDTH_A {
    #[doc = "0: Single bit transfer."]
    _00 = 0,
    #[doc = "1: Two bit transfer."]
    _01 = 1,
    #[doc = "2: Four bit transfer."]
    _10 = 2,
}
impl From<WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WIDTH`"]
pub type WIDTH_R = crate::R<u8, WIDTH_A>;
impl WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WIDTH_A::_00),
            1 => Val(WIDTH_A::_01),
            2 => Val(WIDTH_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WIDTH_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WIDTH_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WIDTH_A::_10
    }
}
#[doc = "Write proxy for field `WIDTH`"]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single bit transfer."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WIDTH_A::_00)
    }
    #[doc = "Two bit transfer."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WIDTH_A::_01)
    }
    #[doc = "Four bit transfer."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WIDTH_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Transmit Data Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSK_A {
    #[doc = "0: Normal transfer."]
    _0 = 0,
    #[doc = "1: Mask transmit data."]
    _1 = 1,
}
impl From<TXMSK_A> for bool {
    #[inline(always)]
    fn from(variant: TXMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXMSK`"]
pub type TXMSK_R = crate::R<bool, TXMSK_A>;
impl TXMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXMSK_A {
        match self.bits {
            false => TXMSK_A::_0,
            true => TXMSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXMSK_A::_1
    }
}
#[doc = "Write proxy for field `TXMSK`"]
pub struct TXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXMSK_A::_0)
    }
    #[doc = "Mask transmit data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXMSK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Receive Data Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMSK_A {
    #[doc = "0: Normal transfer."]
    _0 = 0,
    #[doc = "1: Receive data is masked."]
    _1 = 1,
}
impl From<RXMSK_A> for bool {
    #[inline(always)]
    fn from(variant: RXMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXMSK`"]
pub type RXMSK_R = crate::R<bool, RXMSK_A>;
impl RXMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXMSK_A {
        match self.bits {
            false => RXMSK_A::_0,
            true => RXMSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXMSK_A::_1
    }
}
#[doc = "Write proxy for field `RXMSK`"]
pub struct RXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXMSK_A::_0)
    }
    #[doc = "Receive data is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXMSK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Continuing Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTC_A {
    #[doc = "0: Command word for start of new transfer."]
    _0 = 0,
    #[doc = "1: Command word for continuing transfer."]
    _1 = 1,
}
impl From<CONTC_A> for bool {
    #[inline(always)]
    fn from(variant: CONTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONTC`"]
pub type CONTC_R = crate::R<bool, CONTC_A>;
impl CONTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTC_A {
        match self.bits {
            false => CONTC_A::_0,
            true => CONTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CONTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CONTC_A::_1
    }
}
#[doc = "Write proxy for field `CONTC`"]
pub struct CONTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Command word for start of new transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONTC_A::_0)
    }
    #[doc = "Command word for continuing transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONTC_A::_1)
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
#[doc = "Continuous Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: Continuous transfer disabled."]
    _0 = 0,
    #[doc = "1: Continuous transfer enabled."]
    _1 = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, CONT_A>;
impl CONT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::_0,
            true => CONT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CONT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CONT_A::_1
    }
}
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Continuous transfer disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_A::_0)
    }
    #[doc = "Continuous transfer enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_A::_1)
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
#[doc = "Byte Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYSW_A {
    #[doc = "0: Byte swap disabled."]
    _0 = 0,
    #[doc = "1: Byte swap enabled."]
    _1 = 1,
}
impl From<BYSW_A> for bool {
    #[inline(always)]
    fn from(variant: BYSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYSW`"]
pub type BYSW_R = crate::R<bool, BYSW_A>;
impl BYSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYSW_A {
        match self.bits {
            false => BYSW_A::_0,
            true => BYSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BYSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BYSW_A::_1
    }
}
#[doc = "Write proxy for field `BYSW`"]
pub struct BYSW_W<'a> {
    w: &'a mut W,
}
impl<'a> BYSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Byte swap disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYSW_A::_0)
    }
    #[doc = "Byte swap enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYSW_A::_1)
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
#[doc = "LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBF_A {
    #[doc = "0: Data is transferred MSB first."]
    _0 = 0,
    #[doc = "1: Data is transferred LSB first."]
    _1 = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSBF`"]
pub type LSBF_R = crate::R<bool, LSBF_A>;
impl LSBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::_0,
            true => LSBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LSBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LSBF_A::_1
    }
}
#[doc = "Write proxy for field `LSBF`"]
pub struct LSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is transferred MSB first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBF_A::_0)
    }
    #[doc = "Data is transferred LSB first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Peripheral Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Transfer using LPSPI_PCS\\[0\\]"]
    _00 = 0,
    #[doc = "1: Transfer using LPSPI_PCS\\[1\\]"]
    _01 = 1,
    #[doc = "2: Transfer using LPSPI_PCS\\[2\\]"]
    _10 = 2,
    #[doc = "3: Transfer using LPSPI_PCS\\[3\\]"]
    _11 = 3,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCS`"]
pub type PCS_R = crate::R<u8, PCS_A>;
impl PCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS_A {
        match self.bits {
            0 => PCS_A::_00,
            1 => PCS_A::_01,
            2 => PCS_A::_10,
            3 => PCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PCS_A::_11
    }
}
#[doc = "Write proxy for field `PCS`"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Transfer using LPSPI_PCS\\[0\\]"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCS_A::_00)
    }
    #[doc = "Transfer using LPSPI_PCS\\[1\\]"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCS_A::_01)
    }
    #[doc = "Transfer using LPSPI_PCS\\[2\\]"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCS_A::_10)
    }
    #[doc = "Transfer using LPSPI_PCS\\[3\\]"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PCS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: Divide by 1."]
    _000 = 0,
    #[doc = "1: Divide by 2."]
    _001 = 1,
    #[doc = "2: Divide by 4."]
    _010 = 2,
    #[doc = "3: Divide by 8."]
    _011 = 3,
    #[doc = "4: Divide by 16."]
    _100 = 4,
    #[doc = "5: Divide by 32."]
    _101 = 5,
    #[doc = "6: Divide by 64."]
    _110 = 6,
    #[doc = "7: Divide by 128."]
    _111 = 7,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, PRESCALE_A>;
impl PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALE_A {
        match self.bits {
            0 => PRESCALE_A::_000,
            1 => PRESCALE_A::_001,
            2 => PRESCALE_A::_010,
            3 => PRESCALE_A::_011,
            4 => PRESCALE_A::_100,
            5 => PRESCALE_A::_101,
            6 => PRESCALE_A::_110,
            7 => PRESCALE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PRESCALE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PRESCALE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PRESCALE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PRESCALE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PRESCALE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PRESCALE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PRESCALE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PRESCALE_A::_111
    }
}
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PRESCALE_A::_000)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PRESCALE_A::_001)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PRESCALE_A::_010)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PRESCALE_A::_011)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PRESCALE_A::_100)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PRESCALE_A::_101)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PRESCALE_A::_110)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PRESCALE_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data is captured on the leading edge of SCK and changed on the following edge."]
    _0 = 0,
    #[doc = "1: Data is changed on the leading edge of SCK and captured on the following edge."]
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHA_A::_1
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
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
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: The inactive state value of SCK is low."]
    _0 = 0,
    #[doc = "1: The inactive state value of SCK is high."]
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOL_A::_1
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The inactive state value of SCK is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "The inactive state value of SCK is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
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
    #[doc = "Bits 0:11 - Frame Size"]
    #[inline(always)]
    pub fn framesz(&self) -> FRAMESZ_R {
        FRAMESZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:17 - Transfer Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Transmit Data Mask"]
    #[inline(always)]
    pub fn txmsk(&self) -> TXMSK_R {
        TXMSK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive Data Mask"]
    #[inline(always)]
    pub fn rxmsk(&self) -> RXMSK_R {
        RXMSK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Continuing Command"]
    #[inline(always)]
    pub fn contc(&self) -> CONTC_R {
        CONTC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Continuous Transfer"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Byte Swap"]
    #[inline(always)]
    pub fn bysw(&self) -> BYSW_R {
        BYSW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LSB First"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 27:29 - Prescaler Value"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Frame Size"]
    #[inline(always)]
    pub fn framesz(&mut self) -> FRAMESZ_W {
        FRAMESZ_W { w: self }
    }
    #[doc = "Bits 16:17 - Transfer Width"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
    #[doc = "Bit 18 - Transmit Data Mask"]
    #[inline(always)]
    pub fn txmsk(&mut self) -> TXMSK_W {
        TXMSK_W { w: self }
    }
    #[doc = "Bit 19 - Receive Data Mask"]
    #[inline(always)]
    pub fn rxmsk(&mut self) -> RXMSK_W {
        RXMSK_W { w: self }
    }
    #[doc = "Bit 20 - Continuing Command"]
    #[inline(always)]
    pub fn contc(&mut self) -> CONTC_W {
        CONTC_W { w: self }
    }
    #[doc = "Bit 21 - Continuous Transfer"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bit 22 - Byte Swap"]
    #[inline(always)]
    pub fn bysw(&mut self) -> BYSW_W {
        BYSW_W { w: self }
    }
    #[doc = "Bit 23 - LSB First"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LSBF_W {
        LSBF_W { w: self }
    }
    #[doc = "Bits 24:25 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bits 27:29 - Prescaler Value"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 30 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 31 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
}
