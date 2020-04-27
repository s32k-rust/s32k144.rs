#[doc = "Reader of register BAUD"]
pub type R = crate::R<u32, super::BAUD>;
#[doc = "Writer for register BAUD"]
pub type W = crate::W<u32, super::BAUD>;
#[doc = "Register BAUD `reset()`'s with value 0x0f00_0004"]
impl crate::ResetValue for super::BAUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00_0004
    }
}
#[doc = "Reader of field `SBR`"]
pub type SBR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SBR`"]
pub struct SBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Stop Bit Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBNS_A {
    #[doc = "0: One stop bit."]
    _0 = 0,
    #[doc = "1: Two stop bits."]
    _1 = 1,
}
impl From<SBNS_A> for bool {
    #[inline(always)]
    fn from(variant: SBNS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBNS`"]
pub type SBNS_R = crate::R<bool, SBNS_A>;
impl SBNS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBNS_A {
        match self.bits {
            false => SBNS_A::_0,
            true => SBNS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBNS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBNS_A::_1
    }
}
#[doc = "Write proxy for field `SBNS`"]
pub struct SBNS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBNS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One stop bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBNS_A::_0)
    }
    #[doc = "Two stop bits."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBNS_A::_1)
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
#[doc = "RX Input Active Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIE_A {
    #[doc = "0: Hardware interrupts from LPUART_STAT\\[RXEDGIF\\]
disabled."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when LPUART_STAT\\[RXEDGIF\\]
flag is 1."]
    _1 = 1,
}
impl From<RXEDGIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEDGIE`"]
pub type RXEDGIE_R = crate::R<bool, RXEDGIE_A>;
impl RXEDGIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIE_A {
        match self.bits {
            false => RXEDGIE_A::_0,
            true => RXEDGIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIE_A::_1
    }
}
#[doc = "Write proxy for field `RXEDGIE`"]
pub struct RXEDGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEDGIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEDGIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT\\[RXEDGIF\\]
disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[RXEDGIF\\]
flag is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIE_A::_1)
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
#[doc = "LIN Break Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIE_A {
    #[doc = "0: Hardware interrupts from LPUART_STAT\\[LBKDIF\\]
disabled (use polling)."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when LPUART_STAT\\[LBKDIF\\]
flag is 1."]
    _1 = 1,
}
impl From<LBKDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBKDIE`"]
pub type LBKDIE_R = crate::R<bool, LBKDIE_A>;
impl LBKDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIE_A {
        match self.bits {
            false => LBKDIE_A::_0,
            true => LBKDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDIE_A::_1
    }
}
#[doc = "Write proxy for field `LBKDIE`"]
pub struct LBKDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT\\[LBKDIF\\]
disabled (use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[LBKDIF\\]
flag is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIE_A::_1)
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
#[doc = "Resynchronization Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNCDIS_A {
    #[doc = "0: Resynchronization during received data word is supported"]
    _0 = 0,
    #[doc = "1: Resynchronization during received data word is disabled"]
    _1 = 1,
}
impl From<RESYNCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNCDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESYNCDIS`"]
pub type RESYNCDIS_R = crate::R<bool, RESYNCDIS_A>;
impl RESYNCDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESYNCDIS_A {
        match self.bits {
            false => RESYNCDIS_A::_0,
            true => RESYNCDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESYNCDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESYNCDIS_A::_1
    }
}
#[doc = "Write proxy for field `RESYNCDIS`"]
pub struct RESYNCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESYNCDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESYNCDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resynchronization during received data word is supported"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESYNCDIS_A::_0)
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESYNCDIS_A::_1)
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
#[doc = "Both Edge Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOTHEDGE_A {
    #[doc = "0: Receiver samples input data using the rising edge of the baud rate clock."]
    _0 = 0,
    #[doc = "1: Receiver samples input data using the rising and falling edge of the baud rate clock."]
    _1 = 1,
}
impl From<BOTHEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: BOTHEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOTHEDGE`"]
pub type BOTHEDGE_R = crate::R<bool, BOTHEDGE_A>;
impl BOTHEDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOTHEDGE_A {
        match self.bits {
            false => BOTHEDGE_A::_0,
            true => BOTHEDGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOTHEDGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOTHEDGE_A::_1
    }
}
#[doc = "Write proxy for field `BOTHEDGE`"]
pub struct BOTHEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOTHEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOTHEDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOTHEDGE_A::_0)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOTHEDGE_A::_1)
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
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Address Match Wakeup"]
    _00 = 0,
    #[doc = "1: Idle Match Wakeup"]
    _01 = 1,
    #[doc = "2: Match On and Match Off"]
    _10 = 2,
}
impl From<MATCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MATCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MATCFG`"]
pub type MATCFG_R = crate::R<u8, MATCFG_A>;
impl MATCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MATCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MATCFG_A::_00),
            1 => Val(MATCFG_A::_01),
            2 => Val(MATCFG_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MATCFG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MATCFG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MATCFG_A::_10
    }
}
#[doc = "Write proxy for field `MATCFG`"]
pub struct MATCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Address Match Wakeup"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MATCFG_A::_00)
    }
    #[doc = "Idle Match Wakeup"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MATCFG_A::_01)
    }
    #[doc = "Match On and Match Off"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MATCFG_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Receiver Idle DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIDMAE_A {
    #[doc = "0: DMA request disabled."]
    _0 = 0,
    #[doc = "1: DMA request enabled."]
    _1 = 1,
}
impl From<RIDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RIDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RIDMAE`"]
pub type RIDMAE_R = crate::R<bool, RIDMAE_A>;
impl RIDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIDMAE_A {
        match self.bits {
            false => RIDMAE_A::_0,
            true => RIDMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIDMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIDMAE_A::_1
    }
}
#[doc = "Write proxy for field `RIDMAE`"]
pub struct RIDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIDMAE_A::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIDMAE_A::_1)
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
#[doc = "Receiver Full DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAE_A {
    #[doc = "0: DMA request disabled."]
    _0 = 0,
    #[doc = "1: DMA request enabled."]
    _1 = 1,
}
impl From<RDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDMAE`"]
pub type RDMAE_R = crate::R<bool, RDMAE_A>;
impl RDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAE_A {
        match self.bits {
            false => RDMAE_A::_0,
            true => RDMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDMAE_A::_1
    }
}
#[doc = "Write proxy for field `RDMAE`"]
pub struct RDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMAE_A::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMAE_A::_1)
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
#[doc = "Transmitter DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAE_A {
    #[doc = "0: DMA request disabled."]
    _0 = 0,
    #[doc = "1: DMA request enabled."]
    _1 = 1,
}
impl From<TDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDMAE`"]
pub type TDMAE_R = crate::R<bool, TDMAE_A>;
impl TDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMAE_A {
        match self.bits {
            false => TDMAE_A::_0,
            true => TDMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDMAE_A::_1
    }
}
#[doc = "Write proxy for field `TDMAE`"]
pub struct TDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMAE_A::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMAE_A::_1)
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
#[doc = "Oversampling Ratio\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSR_A {
    #[doc = "0: Writing 0 to this field will result in an oversampling ratio of 16"]
    _00000 = 0,
    #[doc = "3: Oversampling ratio of 4, requires BOTHEDGE to be set."]
    _00011 = 3,
    #[doc = "4: Oversampling ratio of 5, requires BOTHEDGE to be set."]
    _00100 = 4,
    #[doc = "5: Oversampling ratio of 6, requires BOTHEDGE to be set."]
    _00101 = 5,
    #[doc = "6: Oversampling ratio of 7, requires BOTHEDGE to be set."]
    _00110 = 6,
    #[doc = "7: Oversampling ratio of 8."]
    _00111 = 7,
    #[doc = "8: Oversampling ratio of 9."]
    _01000 = 8,
    #[doc = "9: Oversampling ratio of 10."]
    _01001 = 9,
    #[doc = "10: Oversampling ratio of 11."]
    _01010 = 10,
    #[doc = "11: Oversampling ratio of 12."]
    _01011 = 11,
    #[doc = "12: Oversampling ratio of 13."]
    _01100 = 12,
    #[doc = "13: Oversampling ratio of 14."]
    _01101 = 13,
    #[doc = "14: Oversampling ratio of 15."]
    _01110 = 14,
    #[doc = "15: Oversampling ratio of 16."]
    _01111 = 15,
    #[doc = "16: Oversampling ratio of 17."]
    _10000 = 16,
    #[doc = "17: Oversampling ratio of 18."]
    _10001 = 17,
    #[doc = "18: Oversampling ratio of 19."]
    _10010 = 18,
    #[doc = "19: Oversampling ratio of 20."]
    _10011 = 19,
    #[doc = "20: Oversampling ratio of 21."]
    _10100 = 20,
    #[doc = "21: Oversampling ratio of 22."]
    _10101 = 21,
    #[doc = "22: Oversampling ratio of 23."]
    _10110 = 22,
    #[doc = "23: Oversampling ratio of 24."]
    _10111 = 23,
    #[doc = "24: Oversampling ratio of 25."]
    _11000 = 24,
    #[doc = "25: Oversampling ratio of 26."]
    _11001 = 25,
    #[doc = "26: Oversampling ratio of 27."]
    _11010 = 26,
    #[doc = "27: Oversampling ratio of 28."]
    _11011 = 27,
    #[doc = "28: Oversampling ratio of 29."]
    _11100 = 28,
    #[doc = "29: Oversampling ratio of 30."]
    _11101 = 29,
    #[doc = "30: Oversampling ratio of 31."]
    _11110 = 30,
    #[doc = "31: Oversampling ratio of 32."]
    _11111 = 31,
}
impl From<OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSR`"]
pub type OSR_R = crate::R<u8, OSR_A>;
impl OSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSR_A::_00000),
            3 => Val(OSR_A::_00011),
            4 => Val(OSR_A::_00100),
            5 => Val(OSR_A::_00101),
            6 => Val(OSR_A::_00110),
            7 => Val(OSR_A::_00111),
            8 => Val(OSR_A::_01000),
            9 => Val(OSR_A::_01001),
            10 => Val(OSR_A::_01010),
            11 => Val(OSR_A::_01011),
            12 => Val(OSR_A::_01100),
            13 => Val(OSR_A::_01101),
            14 => Val(OSR_A::_01110),
            15 => Val(OSR_A::_01111),
            16 => Val(OSR_A::_10000),
            17 => Val(OSR_A::_10001),
            18 => Val(OSR_A::_10010),
            19 => Val(OSR_A::_10011),
            20 => Val(OSR_A::_10100),
            21 => Val(OSR_A::_10101),
            22 => Val(OSR_A::_10110),
            23 => Val(OSR_A::_10111),
            24 => Val(OSR_A::_11000),
            25 => Val(OSR_A::_11001),
            26 => Val(OSR_A::_11010),
            27 => Val(OSR_A::_11011),
            28 => Val(OSR_A::_11100),
            29 => Val(OSR_A::_11101),
            30 => Val(OSR_A::_11110),
            31 => Val(OSR_A::_11111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == OSR_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == OSR_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == OSR_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == OSR_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == OSR_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == OSR_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == OSR_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == OSR_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == OSR_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == OSR_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == OSR_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == OSR_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == OSR_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == OSR_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == OSR_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == OSR_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == OSR_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == OSR_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == OSR_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == OSR_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == OSR_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == OSR_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == OSR_A::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == OSR_A::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == OSR_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == OSR_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == OSR_A::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == OSR_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == OSR_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == OSR_A::_11111
    }
}
#[doc = "Write proxy for field `OSR`"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(OSR_A::_00000)
    }
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(OSR_A::_00011)
    }
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(OSR_A::_00100)
    }
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(OSR_A::_00101)
    }
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(OSR_A::_00110)
    }
    #[doc = "Oversampling ratio of 8."]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(OSR_A::_00111)
    }
    #[doc = "Oversampling ratio of 9."]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(OSR_A::_01000)
    }
    #[doc = "Oversampling ratio of 10."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(OSR_A::_01001)
    }
    #[doc = "Oversampling ratio of 11."]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(OSR_A::_01010)
    }
    #[doc = "Oversampling ratio of 12."]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(OSR_A::_01011)
    }
    #[doc = "Oversampling ratio of 13."]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(OSR_A::_01100)
    }
    #[doc = "Oversampling ratio of 14."]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(OSR_A::_01101)
    }
    #[doc = "Oversampling ratio of 15."]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(OSR_A::_01110)
    }
    #[doc = "Oversampling ratio of 16."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(OSR_A::_01111)
    }
    #[doc = "Oversampling ratio of 17."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(OSR_A::_10000)
    }
    #[doc = "Oversampling ratio of 18."]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(OSR_A::_10001)
    }
    #[doc = "Oversampling ratio of 19."]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(OSR_A::_10010)
    }
    #[doc = "Oversampling ratio of 20."]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(OSR_A::_10011)
    }
    #[doc = "Oversampling ratio of 21."]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(OSR_A::_10100)
    }
    #[doc = "Oversampling ratio of 22."]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(OSR_A::_10101)
    }
    #[doc = "Oversampling ratio of 23."]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(OSR_A::_10110)
    }
    #[doc = "Oversampling ratio of 24."]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(OSR_A::_10111)
    }
    #[doc = "Oversampling ratio of 25."]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut W {
        self.variant(OSR_A::_11000)
    }
    #[doc = "Oversampling ratio of 26."]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut W {
        self.variant(OSR_A::_11001)
    }
    #[doc = "Oversampling ratio of 27."]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(OSR_A::_11010)
    }
    #[doc = "Oversampling ratio of 28."]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(OSR_A::_11011)
    }
    #[doc = "Oversampling ratio of 29."]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut W {
        self.variant(OSR_A::_11100)
    }
    #[doc = "Oversampling ratio of 30."]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(OSR_A::_11101)
    }
    #[doc = "Oversampling ratio of 31."]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(OSR_A::_11110)
    }
    #[doc = "Oversampling ratio of 32."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(OSR_A::_11111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M10_A {
    #[doc = "0: Receiver and transmitter use 7-bit to 9-bit data characters."]
    _0 = 0,
    #[doc = "1: Receiver and transmitter use 10-bit data characters."]
    _1 = 1,
}
impl From<M10_A> for bool {
    #[inline(always)]
    fn from(variant: M10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `M10`"]
pub type M10_R = crate::R<bool, M10_A>;
impl M10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M10_A {
        match self.bits {
            false => M10_A::_0,
            true => M10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M10_A::_1
    }
}
#[doc = "Write proxy for field `M10`"]
pub struct M10_W<'a> {
    w: &'a mut W,
}
impl<'a> M10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M10_A::_0)
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M10_A::_1)
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
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN2_A {
    #[doc = "0: Normal operation."]
    _0 = 0,
    #[doc = "1: Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    _1 = 1,
}
impl From<MAEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAEN2`"]
pub type MAEN2_R = crate::R<bool, MAEN2_A>;
impl MAEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN2_A {
        match self.bits {
            false => MAEN2_A::_0,
            true => MAEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAEN2_A::_1
    }
}
#[doc = "Write proxy for field `MAEN2`"]
pub struct MAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN2_A::_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN2_A::_1)
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
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN1_A {
    #[doc = "0: Normal operation."]
    _0 = 0,
    #[doc = "1: Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    _1 = 1,
}
impl From<MAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAEN1`"]
pub type MAEN1_R = crate::R<bool, MAEN1_A>;
impl MAEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN1_A {
        match self.bits {
            false => MAEN1_A::_0,
            true => MAEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAEN1_A::_1
    }
}
#[doc = "Write proxy for field `MAEN1`"]
pub struct MAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN1_A::_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN1_A::_1)
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
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&self) -> SBNS_R {
        SBNS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&self) -> RXEDGIE_R {
        RXEDGIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lbkdie(&self) -> LBKDIE_R {
        LBKDIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline(always)]
    pub fn resyncdis(&self) -> RESYNCDIS_R {
        RESYNCDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline(always)]
    pub fn bothedge(&self) -> BOTHEDGE_R {
        BOTHEDGE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Receiver Idle DMA Enable"]
    #[inline(always)]
    pub fn ridmae(&self) -> RIDMAE_R {
        RIDMAE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10_R {
        M10_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> MAEN2_R {
        MAEN2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> MAEN1_R {
        MAEN1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub fn sbr(&mut self) -> SBR_W {
        SBR_W { w: self }
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&mut self) -> SBNS_W {
        SBNS_W { w: self }
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&mut self) -> RXEDGIE_W {
        RXEDGIE_W { w: self }
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lbkdie(&mut self) -> LBKDIE_W {
        LBKDIE_W { w: self }
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline(always)]
    pub fn resyncdis(&mut self) -> RESYNCDIS_W {
        RESYNCDIS_W { w: self }
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline(always)]
    pub fn bothedge(&mut self) -> BOTHEDGE_W {
        BOTHEDGE_W { w: self }
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&mut self) -> MATCFG_W {
        MATCFG_W { w: self }
    }
    #[doc = "Bit 20 - Receiver Idle DMA Enable"]
    #[inline(always)]
    pub fn ridmae(&mut self) -> RIDMAE_W {
        RIDMAE_W { w: self }
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline(always)]
    pub fn rdmae(&mut self) -> RDMAE_W {
        RDMAE_W { w: self }
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline(always)]
    pub fn tdmae(&mut self) -> TDMAE_W {
        TDMAE_W { w: self }
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&mut self) -> M10_W {
        M10_W { w: self }
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&mut self) -> MAEN2_W {
        MAEN2_W { w: self }
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&mut self) -> MAEN1_W {
        MAEN1_W { w: self }
    }
}
