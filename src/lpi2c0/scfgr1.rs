#[doc = "Reader of register SCFGR1"]
pub type R = crate::R<u32, super::SCFGR1>;
#[doc = "Writer for register SCFGR1"]
pub type W = crate::W<u32, super::SCFGR1>;
#[doc = "Register SCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Address SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRSTALL_A {
    #[doc = "0: Clock stretching disabled."]
    _0 = 0,
    #[doc = "1: Clock stretching enabled."]
    _1 = 1,
}
impl From<ADRSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: ADRSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADRSTALL`"]
pub type ADRSTALL_R = crate::R<bool, ADRSTALL_A>;
impl ADRSTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRSTALL_A {
        match self.bits {
            false => ADRSTALL_A::_0,
            true => ADRSTALL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADRSTALL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADRSTALL_A::_1
    }
}
#[doc = "Write proxy for field `ADRSTALL`"]
pub struct ADRSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADRSTALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock stretching disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADRSTALL_A::_0)
    }
    #[doc = "Clock stretching enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADRSTALL_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "RX SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTALL_A {
    #[doc = "0: Clock stretching disabled."]
    _0 = 0,
    #[doc = "1: Clock stretching enabled."]
    _1 = 1,
}
impl From<RXSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXSTALL`"]
pub type RXSTALL_R = crate::R<bool, RXSTALL_A>;
impl RXSTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTALL_A {
        match self.bits {
            false => RXSTALL_A::_0,
            true => RXSTALL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXSTALL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXSTALL_A::_1
    }
}
#[doc = "Write proxy for field `RXSTALL`"]
pub struct RXSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSTALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock stretching disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXSTALL_A::_0)
    }
    #[doc = "Clock stretching enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXSTALL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "TX Data SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDSTALL_A {
    #[doc = "0: Clock stretching disabled."]
    _0 = 0,
    #[doc = "1: Clock stretching enabled."]
    _1 = 1,
}
impl From<TXDSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: TXDSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDSTALL`"]
pub type TXDSTALL_R = crate::R<bool, TXDSTALL_A>;
impl TXDSTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDSTALL_A {
        match self.bits {
            false => TXDSTALL_A::_0,
            true => TXDSTALL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXDSTALL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXDSTALL_A::_1
    }
}
#[doc = "Write proxy for field `TXDSTALL`"]
pub struct TXDSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDSTALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock stretching disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDSTALL_A::_0)
    }
    #[doc = "Clock stretching enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDSTALL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "ACK SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKSTALL_A {
    #[doc = "0: Clock stretching disabled."]
    _0 = 0,
    #[doc = "1: Clock stretching enabled."]
    _1 = 1,
}
impl From<ACKSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: ACKSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACKSTALL`"]
pub type ACKSTALL_R = crate::R<bool, ACKSTALL_A>;
impl ACKSTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKSTALL_A {
        match self.bits {
            false => ACKSTALL_A::_0,
            true => ACKSTALL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKSTALL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKSTALL_A::_1
    }
}
#[doc = "Write proxy for field `ACKSTALL`"]
pub struct ACKSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKSTALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock stretching disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKSTALL_A::_0)
    }
    #[doc = "Clock stretching enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKSTALL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "General Call Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCEN_A {
    #[doc = "0: General Call address is disabled."]
    _0 = 0,
    #[doc = "1: General call address is enabled."]
    _1 = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GCEN`"]
pub type GCEN_R = crate::R<bool, GCEN_A>;
impl GCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::_0,
            true => GCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCEN_A::_1
    }
}
#[doc = "Write proxy for field `GCEN`"]
pub struct GCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "General Call address is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCEN_A::_0)
    }
    #[doc = "General call address is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCEN_A::_1)
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
#[doc = "SMBus Alert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAEN_A {
    #[doc = "0: Disables match on SMBus Alert."]
    _0 = 0,
    #[doc = "1: Enables match on SMBus Alert."]
    _1 = 1,
}
impl From<SAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAEN`"]
pub type SAEN_R = crate::R<bool, SAEN_A>;
impl SAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAEN_A {
        match self.bits {
            false => SAEN_A::_0,
            true => SAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAEN_A::_1
    }
}
#[doc = "Write proxy for field `SAEN`"]
pub struct SAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables match on SMBus Alert."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAEN_A::_0)
    }
    #[doc = "Enables match on SMBus Alert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAEN_A::_1)
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
#[doc = "Transmit Flag Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCFG_A {
    #[doc = "0: Transmit Data Flag will only assert during a slave-transmit transfer when the transmit data register is empty."]
    _0 = 0,
    #[doc = "1: Transmit Data Flag will assert whenever the transmit data register is empty."]
    _1 = 1,
}
impl From<TXCFG_A> for bool {
    #[inline(always)]
    fn from(variant: TXCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXCFG`"]
pub type TXCFG_R = crate::R<bool, TXCFG_A>;
impl TXCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCFG_A {
        match self.bits {
            false => TXCFG_A::_0,
            true => TXCFG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXCFG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXCFG_A::_1
    }
}
#[doc = "Write proxy for field `TXCFG`"]
pub struct TXCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the transmit data register is empty."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCFG_A::_0)
    }
    #[doc = "Transmit Data Flag will assert whenever the transmit data register is empty."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCFG_A::_1)
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
#[doc = "Receive Data Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCFG_A {
    #[doc = "0: Reading the receive data register will return receive data and clear the receive data flag."]
    _0 = 0,
    #[doc = "1: Reading the receive data register when the address valid flag is set will return the address status register and clear the address valid flag. Reading the receive data register when the address valid flag is clear will return receive data and clear the receive data flag."]
    _1 = 1,
}
impl From<RXCFG_A> for bool {
    #[inline(always)]
    fn from(variant: RXCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXCFG`"]
pub type RXCFG_R = crate::R<bool, RXCFG_A>;
impl RXCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCFG_A {
        match self.bits {
            false => RXCFG_A::_0,
            true => RXCFG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXCFG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXCFG_A::_1
    }
}
#[doc = "Write proxy for field `RXCFG`"]
pub struct RXCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reading the receive data register will return receive data and clear the receive data flag."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXCFG_A::_0)
    }
    #[doc = "Reading the receive data register when the address valid flag is set will return the address status register and clear the address valid flag. Reading the receive data register when the address valid flag is clear will return receive data and clear the receive data flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXCFG_A::_1)
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
#[doc = "Ignore NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNACK_A {
    #[doc = "0: Slave will end transfer when NACK detected."]
    _0 = 0,
    #[doc = "1: Slave will not end transfer when NACK detected."]
    _1 = 1,
}
impl From<IGNACK_A> for bool {
    #[inline(always)]
    fn from(variant: IGNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IGNACK`"]
pub type IGNACK_R = crate::R<bool, IGNACK_A>;
impl IGNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGNACK_A {
        match self.bits {
            false => IGNACK_A::_0,
            true => IGNACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IGNACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IGNACK_A::_1
    }
}
#[doc = "Write proxy for field `IGNACK`"]
pub struct IGNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave will end transfer when NACK detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGNACK_A::_0)
    }
    #[doc = "Slave will not end transfer when NACK detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGNACK_A::_1)
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
#[doc = "High Speed Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSMEN_A {
    #[doc = "0: Disables detection of Hs-mode master code."]
    _0 = 0,
    #[doc = "1: Enables detection of Hs-mode master code."]
    _1 = 1,
}
impl From<HSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSMEN`"]
pub type HSMEN_R = crate::R<bool, HSMEN_A>;
impl HSMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMEN_A {
        match self.bits {
            false => HSMEN_A::_0,
            true => HSMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSMEN_A::_1
    }
}
#[doc = "Write proxy for field `HSMEN`"]
pub struct HSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables detection of Hs-mode master code."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSMEN_A::_0)
    }
    #[doc = "Enables detection of Hs-mode master code."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSMEN_A::_1)
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
#[doc = "Address Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADDRCFG_A {
    #[doc = "0: Address match 0 (7-bit)."]
    _000 = 0,
    #[doc = "1: Address match 0 (10-bit)."]
    _001 = 1,
    #[doc = "2: Address match 0 (7-bit) or Address match 1 (7-bit)."]
    _010 = 2,
    #[doc = "3: Address match 0 (10-bit) or Address match 1 (10-bit)."]
    _011 = 3,
    #[doc = "4: Address match 0 (7-bit) or Address match 1 (10-bit)."]
    _100 = 4,
    #[doc = "5: Address match 0 (10-bit) or Address match 1 (7-bit)."]
    _101 = 5,
    #[doc = "6: From Address match 0 (7-bit) to Address match 1 (7-bit)."]
    _110 = 6,
    #[doc = "7: From Address match 0 (10-bit) to Address match 1 (10-bit)."]
    _111 = 7,
}
impl From<ADDRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDRCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADDRCFG`"]
pub type ADDRCFG_R = crate::R<u8, ADDRCFG_A>;
impl ADDRCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRCFG_A {
        match self.bits {
            0 => ADDRCFG_A::_000,
            1 => ADDRCFG_A::_001,
            2 => ADDRCFG_A::_010,
            3 => ADDRCFG_A::_011,
            4 => ADDRCFG_A::_100,
            5 => ADDRCFG_A::_101,
            6 => ADDRCFG_A::_110,
            7 => ADDRCFG_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ADDRCFG_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ADDRCFG_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ADDRCFG_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ADDRCFG_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ADDRCFG_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ADDRCFG_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ADDRCFG_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == ADDRCFG_A::_111
    }
}
#[doc = "Write proxy for field `ADDRCFG`"]
pub struct ADDRCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Address match 0 (7-bit)."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADDRCFG_A::_000)
    }
    #[doc = "Address match 0 (10-bit)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADDRCFG_A::_001)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADDRCFG_A::_010)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADDRCFG_A::_011)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ADDRCFG_A::_100)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADDRCFG_A::_101)
    }
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ADDRCFG_A::_110)
    }
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(ADDRCFG_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline(always)]
    pub fn adrstall(&self) -> ADRSTALL_R {
        ADRSTALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline(always)]
    pub fn rxstall(&self) -> RXSTALL_R {
        RXSTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline(always)]
    pub fn txdstall(&self) -> TXDSTALL_R {
        TXDSTALL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline(always)]
    pub fn ackstall(&self) -> ACKSTALL_R {
        ACKSTALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline(always)]
    pub fn saen(&self) -> SAEN_R {
        SAEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline(always)]
    pub fn txcfg(&self) -> TXCFG_R {
        TXCFG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline(always)]
    pub fn rxcfg(&self) -> RXCFG_R {
        RXCFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsmen(&self) -> HSMEN_R {
        HSMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline(always)]
    pub fn addrcfg(&self) -> ADDRCFG_R {
        ADDRCFG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline(always)]
    pub fn adrstall(&mut self) -> ADRSTALL_W {
        ADRSTALL_W { w: self }
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline(always)]
    pub fn rxstall(&mut self) -> RXSTALL_W {
        RXSTALL_W { w: self }
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline(always)]
    pub fn txdstall(&mut self) -> TXDSTALL_W {
        TXDSTALL_W { w: self }
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline(always)]
    pub fn ackstall(&mut self) -> ACKSTALL_W {
        ACKSTALL_W { w: self }
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W { w: self }
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline(always)]
    pub fn saen(&mut self) -> SAEN_W {
        SAEN_W { w: self }
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline(always)]
    pub fn txcfg(&mut self) -> TXCFG_W {
        TXCFG_W { w: self }
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline(always)]
    pub fn rxcfg(&mut self) -> RXCFG_W {
        RXCFG_W { w: self }
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IGNACK_W {
        IGNACK_W { w: self }
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsmen(&mut self) -> HSMEN_W {
        HSMEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline(always)]
    pub fn addrcfg(&mut self) -> ADDRCFG_W {
        ADDRCFG_W { w: self }
    }
}
