#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0x00c0_0000"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00c0_0000
    }
}
#[doc = "Match 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA2F_A {
    #[doc = "0: Received data is not equal to MA2"]
    _0 = 0,
    #[doc = "1: Received data is equal to MA2"]
    _1 = 1,
}
impl From<MA2F_A> for bool {
    #[inline(always)]
    fn from(variant: MA2F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MA2F`"]
pub type MA2F_R = crate::R<bool, MA2F_A>;
impl MA2F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA2F_A {
        match self.bits {
            false => MA2F_A::_0,
            true => MA2F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MA2F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MA2F_A::_1
    }
}
#[doc = "Write proxy for field `MA2F`"]
pub struct MA2F_W<'a> {
    w: &'a mut W,
}
impl<'a> MA2F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MA2F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Received data is not equal to MA2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MA2F_A::_0)
    }
    #[doc = "Received data is equal to MA2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MA2F_A::_1)
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
#[doc = "Match 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA1F_A {
    #[doc = "0: Received data is not equal to MA1"]
    _0 = 0,
    #[doc = "1: Received data is equal to MA1"]
    _1 = 1,
}
impl From<MA1F_A> for bool {
    #[inline(always)]
    fn from(variant: MA1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MA1F`"]
pub type MA1F_R = crate::R<bool, MA1F_A>;
impl MA1F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA1F_A {
        match self.bits {
            false => MA1F_A::_0,
            true => MA1F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MA1F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MA1F_A::_1
    }
}
#[doc = "Write proxy for field `MA1F`"]
pub struct MA1F_W<'a> {
    w: &'a mut W,
}
impl<'a> MA1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MA1F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Received data is not equal to MA1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MA1F_A::_0)
    }
    #[doc = "Received data is equal to MA1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MA1F_A::_1)
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
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF_A {
    #[doc = "0: No parity error."]
    _0 = 0,
    #[doc = "1: Parity error."]
    _1 = 1,
}
impl From<PF_A> for bool {
    #[inline(always)]
    fn from(variant: PF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PF`"]
pub type PF_R = crate::R<bool, PF_A>;
impl PF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF_A {
        match self.bits {
            false => PF_A::_0,
            true => PF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PF_A::_1
    }
}
#[doc = "Write proxy for field `PF`"]
pub struct PF_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No parity error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PF_A::_0)
    }
    #[doc = "Parity error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PF_A::_1)
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
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: No framing error detected. This does not guarantee the framing is correct."]
    _0 = 0,
    #[doc = "1: Framing error."]
    _1 = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FE`"]
pub type FE_R = crate::R<bool, FE_A>;
impl FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::_0,
            true => FE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FE_A::_1
    }
}
#[doc = "Write proxy for field `FE`"]
pub struct FE_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FE_A::_0)
    }
    #[doc = "Framing error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FE_A::_1)
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
#[doc = "Noise Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NF_A {
    #[doc = "0: No noise detected."]
    _0 = 0,
    #[doc = "1: Noise detected in the received character in LPUART_DATA."]
    _1 = 1,
}
impl From<NF_A> for bool {
    #[inline(always)]
    fn from(variant: NF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NF`"]
pub type NF_R = crate::R<bool, NF_A>;
impl NF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NF_A {
        match self.bits {
            false => NF_A::_0,
            true => NF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NF_A::_1
    }
}
#[doc = "Write proxy for field `NF`"]
pub struct NF_W<'a> {
    w: &'a mut W,
}
impl<'a> NF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No noise detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NF_A::_0)
    }
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NF_A::_1)
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
#[doc = "Receiver Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OR_A {
    #[doc = "0: No overrun."]
    _0 = 0,
    #[doc = "1: Receive overrun (new LPUART data lost)."]
    _1 = 1,
}
impl From<OR_A> for bool {
    #[inline(always)]
    fn from(variant: OR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OR`"]
pub type OR_R = crate::R<bool, OR_A>;
impl OR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR_A {
        match self.bits {
            false => OR_A::_0,
            true => OR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OR_A::_1
    }
}
#[doc = "Write proxy for field `OR`"]
pub struct OR_W<'a> {
    w: &'a mut W,
}
impl<'a> OR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No overrun."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OR_A::_0)
    }
    #[doc = "Receive overrun (new LPUART data lost)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OR_A::_1)
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
#[doc = "Idle Line Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_A {
    #[doc = "0: No idle line detected."]
    _0 = 0,
    #[doc = "1: Idle line was detected."]
    _1 = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, IDLE_A>;
impl IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::_0,
            true => IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDLE_A::_1
    }
}
#[doc = "Write proxy for field `IDLE`"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No idle line detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDLE_A::_0)
    }
    #[doc = "Idle line was detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDLE_A::_1)
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
#[doc = "Receive Data Register Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRF_A {
    #[doc = "0: Receive data buffer empty."]
    _0 = 0,
    #[doc = "1: Receive data buffer full."]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDRF`"]
pub type RDRF_R = crate::R<bool, RDRF_A>;
impl RDRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
#[doc = "Transmission Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    #[doc = "0: Transmitter active (sending data, a preamble, or a break)."]
    _0 = 0,
    #[doc = "1: Transmitter idle (transmission activity complete)."]
    _1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, TC_A>;
impl TC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::_0,
            true => TC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TC_A::_1
    }
}
#[doc = "Transmit Data Register Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
    #[doc = "0: Transmit data buffer full."]
    _0 = 0,
    #[doc = "1: Transmit data buffer empty."]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDRE`"]
pub type TDRE_R = crate::R<bool, TDRE_A>;
impl TDRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
#[doc = "Receiver Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAF_A {
    #[doc = "0: LPUART receiver idle waiting for a start bit."]
    _0 = 0,
    #[doc = "1: LPUART receiver active (RXD input not idle)."]
    _1 = 1,
}
impl From<RAF_A> for bool {
    #[inline(always)]
    fn from(variant: RAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAF`"]
pub type RAF_R = crate::R<bool, RAF_A>;
impl RAF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAF_A {
        match self.bits {
            false => RAF_A::_0,
            true => RAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAF_A::_1
    }
}
#[doc = "LIN Break Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDE_A {
    #[doc = "0: LIN break detect is disabled, normal break character can be detected."]
    _0 = 0,
    #[doc = "1: LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    _1 = 1,
}
impl From<LBKDE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBKDE`"]
pub type LBKDE_R = crate::R<bool, LBKDE_A>;
impl LBKDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDE_A {
        match self.bits {
            false => LBKDE_A::_0,
            true => LBKDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDE_A::_1
    }
}
#[doc = "Write proxy for field `LBKDE`"]
pub struct LBKDE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LIN break detect is disabled, normal break character can be detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDE_A::_0)
    }
    #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDE_A::_1)
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
#[doc = "Break Character Generation Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK13_A {
    #[doc = "0: Break character is transmitted with length of 9 to 13 bit times."]
    _0 = 0,
    #[doc = "1: Break character is transmitted with length of 12 to 15 bit times."]
    _1 = 1,
}
impl From<BRK13_A> for bool {
    #[inline(always)]
    fn from(variant: BRK13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRK13`"]
pub type BRK13_R = crate::R<bool, BRK13_A>;
impl BRK13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK13_A {
        match self.bits {
            false => BRK13_A::_0,
            true => BRK13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRK13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRK13_A::_1
    }
}
#[doc = "Write proxy for field `BRK13`"]
pub struct BRK13_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK13_A::_0)
    }
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK13_A::_1)
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
#[doc = "Receive Wake Up Idle Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUID_A {
    #[doc = "0: During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    _0 = 0,
    #[doc = "1: During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    _1 = 1,
}
impl From<RWUID_A> for bool {
    #[inline(always)]
    fn from(variant: RWUID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RWUID`"]
pub type RWUID_R = crate::R<bool, RWUID_A>;
impl RWUID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWUID_A {
        match self.bits {
            false => RWUID_A::_0,
            true => RWUID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWUID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWUID_A::_1
    }
}
#[doc = "Write proxy for field `RWUID`"]
pub struct RWUID_W<'a> {
    w: &'a mut W,
}
impl<'a> RWUID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWUID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWUID_A::_0)
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWUID_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Receive Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINV_A {
    #[doc = "0: Receive data not inverted."]
    _0 = 0,
    #[doc = "1: Receive data inverted."]
    _1 = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXINV`"]
pub type RXINV_R = crate::R<bool, RXINV_A>;
impl RXINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::_0,
            true => RXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXINV_A::_1
    }
}
#[doc = "Write proxy for field `RXINV`"]
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive data not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXINV_A::_0)
    }
    #[doc = "Receive data inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXINV_A::_1)
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
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBF_A {
    #[doc = "0: LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    _0 = 0,
    #[doc = "1: MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\]
and BAUD\\[M10\\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL\\[M\\]
and CTRL\\[PE\\]."]
    _1 = 1,
}
impl From<MSBF_A> for bool {
    #[inline(always)]
    fn from(variant: MSBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSBF`"]
pub type MSBF_R = crate::R<bool, MSBF_A>;
impl MSBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBF_A {
        match self.bits {
            false => MSBF_A::_0,
            true => MSBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSBF_A::_1
    }
}
#[doc = "Write proxy for field `MSBF`"]
pub struct MSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSBF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBF_A::_0)
    }
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\]
and BAUD\\[M10\\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL\\[M\\]
and CTRL\\[PE\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBF_A::_1)
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
#[doc = "RXD Pin Active Edge Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIF_A {
    #[doc = "0: No active edge on the receive pin has occurred."]
    _0 = 0,
    #[doc = "1: An active edge on the receive pin has occurred."]
    _1 = 1,
}
impl From<RXEDGIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEDGIF`"]
pub type RXEDGIF_R = crate::R<bool, RXEDGIF_A>;
impl RXEDGIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIF_A {
        match self.bits {
            false => RXEDGIF_A::_0,
            true => RXEDGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIF_A::_1
    }
}
#[doc = "Write proxy for field `RXEDGIF`"]
pub struct RXEDGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEDGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEDGIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIF_A::_0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIF_A::_1)
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
#[doc = "LIN Break Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIF_A {
    #[doc = "0: No LIN break character has been detected."]
    _0 = 0,
    #[doc = "1: LIN break character has been detected."]
    _1 = 1,
}
impl From<LBKDIF_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBKDIF`"]
pub type LBKDIF_R = crate::R<bool, LBKDIF_A>;
impl LBKDIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIF_A {
        match self.bits {
            false => LBKDIF_A::_0,
            true => LBKDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDIF_A::_1
    }
}
#[doc = "Write proxy for field `LBKDIF`"]
pub struct LBKDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No LIN break character has been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIF_A::_0)
    }
    #[doc = "LIN break character has been detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIF_A::_1)
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
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline(always)]
    pub fn ma2f(&self) -> MA2F_R {
        MA2F_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline(always)]
    pub fn ma1f(&self) -> MA1F_R {
        MA1F_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Receive Data Register Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmission Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Receiver Active Flag"]
    #[inline(always)]
    pub fn raf(&self) -> RAF_R {
        RAF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&self) -> LBKDE_R {
        LBKDE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline(always)]
    pub fn brk13(&self) -> BRK13_R {
        BRK13_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&self) -> RWUID_R {
        RWUID_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RXD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&self) -> RXEDGIF_R {
        RXEDGIF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&self) -> LBKDIF_R {
        LBKDIF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline(always)]
    pub fn ma2f(&mut self) -> MA2F_W {
        MA2F_W { w: self }
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline(always)]
    pub fn ma1f(&mut self) -> MA1F_W {
        MA1F_W { w: self }
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W {
        PF_W { w: self }
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W {
        FE_W { w: self }
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&mut self) -> NF_W {
        NF_W { w: self }
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&mut self) -> OR_W {
        OR_W { w: self }
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&mut self) -> LBKDE_W {
        LBKDE_W { w: self }
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline(always)]
    pub fn brk13(&mut self) -> BRK13_W {
        BRK13_W { w: self }
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&mut self) -> RWUID_W {
        RWUID_W { w: self }
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W { w: self }
    }
    #[doc = "Bit 30 - RXD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&mut self) -> RXEDGIF_W {
        RXEDGIF_W { w: self }
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&mut self) -> LBKDIF_W {
        LBKDIF_W { w: self }
    }
}
