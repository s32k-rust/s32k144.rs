#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0xd890_000f"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xd890_000f
    }
}
#[doc = "Reader of field `MAXMB`"]
pub type MAXMB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXMB`"]
pub struct MAXMB_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "ID Acceptance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDAM_A {
    #[doc = "0: Format A: One full ID (standard and extended) per ID Filter Table element."]
    _00 = 0,
    #[doc = "1: Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    _01 = 1,
    #[doc = "2: Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    _10 = 2,
    #[doc = "3: Format D: All frames rejected."]
    _11 = 3,
}
impl From<IDAM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IDAM`"]
pub type IDAM_R = crate::R<u8, IDAM_A>;
impl IDAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDAM_A {
        match self.bits {
            0 => IDAM_A::_00,
            1 => IDAM_A::_01,
            2 => IDAM_A::_10,
            3 => IDAM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDAM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDAM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDAM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDAM_A::_11
    }
}
#[doc = "Write proxy for field `IDAM`"]
pub struct IDAM_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDAM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IDAM_A::_00)
    }
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IDAM_A::_01)
    }
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IDAM_A::_10)
    }
    #[doc = "Format D: All frames rejected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IDAM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "CAN FD operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDEN_A {
    #[doc = "1: CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
    _1 = 1,
    #[doc = "0: CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
    _0 = 0,
}
impl From<FDEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDEN`"]
pub type FDEN_R = crate::R<bool, FDEN_A>;
impl FDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDEN_A {
        match self.bits {
            true => FDEN_A::_1,
            false => FDEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDEN_A::_0
    }
}
#[doc = "Write proxy for field `FDEN`"]
pub struct FDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDEN_A::_1)
    }
    #[doc = "CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDEN_A::_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Abort Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AEN_A {
    #[doc = "0: Abort disabled."]
    _0 = 0,
    #[doc = "1: Abort enabled."]
    _1 = 1,
}
impl From<AEN_A> for bool {
    #[inline(always)]
    fn from(variant: AEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AEN`"]
pub type AEN_R = crate::R<bool, AEN_A>;
impl AEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEN_A {
        match self.bits {
            false => AEN_A::_0,
            true => AEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AEN_A::_1
    }
}
#[doc = "Write proxy for field `AEN`"]
pub struct AEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Abort disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AEN_A::_0)
    }
    #[doc = "Abort enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Local Priority Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPRIOEN_A {
    #[doc = "0: Local Priority disabled."]
    _0 = 0,
    #[doc = "1: Local Priority enabled."]
    _1 = 1,
}
impl From<LPRIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRIOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPRIOEN`"]
pub type LPRIOEN_R = crate::R<bool, LPRIOEN_A>;
impl LPRIOEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPRIOEN_A {
        match self.bits {
            false => LPRIOEN_A::_0,
            true => LPRIOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPRIOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPRIOEN_A::_1
    }
}
#[doc = "Write proxy for field `LPRIOEN`"]
pub struct LPRIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPRIOEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Local Priority disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPRIOEN_A::_0)
    }
    #[doc = "Local Priority enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPRIOEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Pretended Networking Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PNET_EN_A {
    #[doc = "0: Pretended Networking mode is disabled."]
    _0 = 0,
    #[doc = "1: Pretended Networking mode is enabled."]
    _1 = 1,
}
impl From<PNET_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PNET_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PNET_EN`"]
pub type PNET_EN_R = crate::R<bool, PNET_EN_A>;
impl PNET_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PNET_EN_A {
        match self.bits {
            false => PNET_EN_A::_0,
            true => PNET_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PNET_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PNET_EN_A::_1
    }
}
#[doc = "Write proxy for field `PNET_EN`"]
pub struct PNET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PNET_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PNET_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pretended Networking mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PNET_EN_A::_0)
    }
    #[doc = "Pretended Networking mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PNET_EN_A::_1)
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
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: DMA feature for RX FIFO disabled."]
    _0 = 0,
    #[doc = "1: DMA feature for RX FIFO enabled."]
    _1 = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::_0,
            true => DMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMA_A::_1
    }
}
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA feature for RX FIFO disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "DMA feature for RX FIFO enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
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
#[doc = "Individual Rx Masking And Queue Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRMQ_A {
    #[doc = "0: Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    _0 = 0,
    #[doc = "1: Individual Rx masking and queue feature are enabled."]
    _1 = 1,
}
impl From<IRMQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRMQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRMQ`"]
pub type IRMQ_R = crate::R<bool, IRMQ_A>;
impl IRMQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRMQ_A {
        match self.bits {
            false => IRMQ_A::_0,
            true => IRMQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRMQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRMQ_A::_1
    }
}
#[doc = "Write proxy for field `IRMQ`"]
pub struct IRMQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRMQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRMQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRMQ_A::_0)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRMQ_A::_1)
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
#[doc = "Self Reception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRXDIS_A {
    #[doc = "0: Self reception enabled."]
    _0 = 0,
    #[doc = "1: Self reception disabled."]
    _1 = 1,
}
impl From<SRXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SRXDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRXDIS`"]
pub type SRXDIS_R = crate::R<bool, SRXDIS_A>;
impl SRXDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRXDIS_A {
        match self.bits {
            false => SRXDIS_A::_0,
            true => SRXDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRXDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRXDIS_A::_1
    }
}
#[doc = "Write proxy for field `SRXDIS`"]
pub struct SRXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRXDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Self reception enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRXDIS_A::_0)
    }
    #[doc = "Self reception disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRXDIS_A::_1)
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
#[doc = "Low-Power Mode Acknowledge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACK_A {
    #[doc = "0: FlexCAN is not in a low-power mode."]
    _0 = 0,
    #[doc = "1: FlexCAN is in a low-power mode."]
    _1 = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPMACK`"]
pub type LPMACK_R = crate::R<bool, LPMACK_A>;
impl LPMACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::_0,
            true => LPMACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPMACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPMACK_A::_1
    }
}
#[doc = "Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRNEN_A {
    #[doc = "0: TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    _0 = 0,
    #[doc = "1: TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    _1 = 1,
}
impl From<WRNEN_A> for bool {
    #[inline(always)]
    fn from(variant: WRNEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRNEN`"]
pub type WRNEN_R = crate::R<bool, WRNEN_A>;
impl WRNEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRNEN_A {
        match self.bits {
            false => WRNEN_A::_0,
            true => WRNEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WRNEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRNEN_A::_1
    }
}
#[doc = "Write proxy for field `WRNEN`"]
pub struct WRNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRNEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRNEN_A::_0)
    }
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRNEN_A::_1)
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
#[doc = "Reader of field `SUPV`"]
pub type SUPV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUPV`"]
pub struct SUPV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPV_W<'a> {
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
#[doc = "Freeze Mode Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZACK_A {
    #[doc = "0: FlexCAN not in Freeze mode, prescaler running."]
    _0 = 0,
    #[doc = "1: FlexCAN in Freeze mode, prescaler stopped."]
    _1 = 1,
}
impl From<FRZACK_A> for bool {
    #[inline(always)]
    fn from(variant: FRZACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRZACK`"]
pub type FRZACK_R = crate::R<bool, FRZACK_A>;
impl FRZACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZACK_A {
        match self.bits {
            false => FRZACK_A::_0,
            true => FRZACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRZACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRZACK_A::_1
    }
}
#[doc = "Soft Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTRST_A {
    #[doc = "0: No reset request."]
    _0 = 0,
    #[doc = "1: Resets the registers affected by soft reset."]
    _1 = 1,
}
impl From<SOFTRST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOFTRST`"]
pub type SOFTRST_R = crate::R<bool, SOFTRST_A>;
impl SOFTRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTRST_A {
        match self.bits {
            false => SOFTRST_A::_0,
            true => SOFTRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFTRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFTRST_A::_1
    }
}
#[doc = "Write proxy for field `SOFTRST`"]
pub struct SOFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFTRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTRST_A::_0)
    }
    #[doc = "Resets the registers affected by soft reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTRST_A::_1)
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
#[doc = "FlexCAN Not Ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRDY_A {
    #[doc = "0: FlexCAN module is either in Normal mode, Listen-Only mode or Loop-Back mode."]
    _0 = 0,
}
impl From<NOTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOTRDY`"]
pub type NOTRDY_R = crate::R<bool, NOTRDY_A>;
impl NOTRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, NOTRDY_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(NOTRDY_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOTRDY_A::_0
    }
}
#[doc = "Halt FlexCAN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: No Freeze mode request."]
    _0 = 0,
    #[doc = "1: Enters Freeze mode if the FRZ bit is asserted."]
    _1 = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HALT`"]
pub type HALT_R = crate::R<bool, HALT_A>;
impl HALT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::_0,
            true => HALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HALT_A::_1
    }
}
#[doc = "Write proxy for field `HALT`"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Freeze mode request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALT_A::_1)
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
#[doc = "Rx FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFEN_A {
    #[doc = "0: Rx FIFO not enabled."]
    _0 = 0,
    #[doc = "1: Rx FIFO enabled."]
    _1 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RFEN`"]
pub type RFEN_R = crate::R<bool, RFEN_A>;
impl RFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::_0,
            true => RFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFEN_A::_1
    }
}
#[doc = "Write proxy for field `RFEN`"]
pub struct RFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFEN_A::_0)
    }
    #[doc = "Rx FIFO enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFEN_A::_1)
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
#[doc = "Freeze Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZ_A {
    #[doc = "0: Not enabled to enter Freeze mode."]
    _0 = 0,
    #[doc = "1: Enabled to enter Freeze mode."]
    _1 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRZ`"]
pub type FRZ_R = crate::R<bool, FRZ_A>;
impl FRZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::_0,
            true => FRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRZ_A::_1
    }
}
#[doc = "Write proxy for field `FRZ`"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZ_A::_0)
    }
    #[doc = "Enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZ_A::_1)
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
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Enable the FlexCAN module."]
    _0 = 0,
    #[doc = "1: Disable the FlexCAN module."]
    _1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, MDIS_A>;
impl MDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::_0,
            true => MDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDIS_A::_1
    }
}
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the FlexCAN module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Disable the FlexCAN module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIS_A::_1)
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
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&self) -> MAXMB_R {
        MAXMB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&self) -> IDAM_R {
        IDAM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - CAN FD operation enable"]
    #[inline(always)]
    pub fn fden(&self) -> FDEN_R {
        FDEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&self) -> LPRIOEN_R {
        LPRIOEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pretended Networking Enable"]
    #[inline(always)]
    pub fn pnet_en(&self) -> PNET_EN_R {
        PNET_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&self) -> IRMQ_R {
        IRMQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&self) -> SRXDIS_R {
        SRXDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Low-Power Mode Acknowledge"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&self) -> WRNEN_R {
        WRNEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&self) -> SUPV_R {
        SUPV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Freeze Mode Acknowledge"]
    #[inline(always)]
    pub fn frzack(&self) -> FRZACK_R {
        FRZACK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FlexCAN Not Ready"]
    #[inline(always)]
    pub fn notrdy(&self) -> NOTRDY_R {
        NOTRDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&mut self) -> MAXMB_W {
        MAXMB_W { w: self }
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&mut self) -> IDAM_W {
        IDAM_W { w: self }
    }
    #[doc = "Bit 11 - CAN FD operation enable"]
    #[inline(always)]
    pub fn fden(&mut self) -> FDEN_W {
        FDEN_W { w: self }
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&mut self) -> AEN_W {
        AEN_W { w: self }
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&mut self) -> LPRIOEN_W {
        LPRIOEN_W { w: self }
    }
    #[doc = "Bit 14 - Pretended Networking Enable"]
    #[inline(always)]
    pub fn pnet_en(&mut self) -> PNET_EN_W {
        PNET_EN_W { w: self }
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&mut self) -> IRMQ_W {
        IRMQ_W { w: self }
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&mut self) -> SRXDIS_W {
        SRXDIS_W { w: self }
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&mut self) -> WRNEN_W {
        WRNEN_W { w: self }
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&mut self) -> SUPV_W {
        SUPV_W { w: self }
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SOFTRST_W {
        SOFTRST_W { w: self }
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W { w: self }
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
}
