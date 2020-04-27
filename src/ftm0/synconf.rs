#[doc = "Reader of register SYNCONF"]
pub type R = crate::R<u32, super::SYNCONF>;
#[doc = "Writer for register SYNCONF"]
pub type W = crate::W<u32, super::SYNCONF>;
#[doc = "Register SYNCONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hardware Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTRIGMODE_A {
    #[doc = "0: FTM clears the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    _0 = 0,
    #[doc = "1: FTM does not clear the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    _1 = 1,
}
impl From<HWTRIGMODE_A> for bool {
    #[inline(always)]
    fn from(variant: HWTRIGMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HWTRIGMODE`"]
pub type HWTRIGMODE_R = crate::R<bool, HWTRIGMODE_A>;
impl HWTRIGMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWTRIGMODE_A {
        match self.bits {
            false => HWTRIGMODE_A::_0,
            true => HWTRIGMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWTRIGMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWTRIGMODE_A::_1
    }
}
#[doc = "Write proxy for field `HWTRIGMODE`"]
pub struct HWTRIGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HWTRIGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWTRIGMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM clears the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWTRIGMODE_A::_0)
    }
    #[doc = "FTM does not clear the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWTRIGMODE_A::_1)
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
#[doc = "CNTIN Register Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTINC_A {
    #[doc = "0: CNTIN register is updated with its buffer value at all rising edges of FTM input clock."]
    _0 = 0,
    #[doc = "1: CNTIN register is updated with its buffer value by the PWM synchronization."]
    _1 = 1,
}
impl From<CNTINC_A> for bool {
    #[inline(always)]
    fn from(variant: CNTINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNTINC`"]
pub type CNTINC_R = crate::R<bool, CNTINC_A>;
impl CNTINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTINC_A {
        match self.bits {
            false => CNTINC_A::_0,
            true => CNTINC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTINC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTINC_A::_1
    }
}
#[doc = "Write proxy for field `CNTINC`"]
pub struct CNTINC_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CNTIN register is updated with its buffer value at all rising edges of FTM input clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTINC_A::_0)
    }
    #[doc = "CNTIN register is updated with its buffer value by the PWM synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTINC_A::_1)
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
#[doc = "INVCTRL Register Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVC_A {
    #[doc = "0: INVCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    _0 = 0,
    #[doc = "1: INVCTRL register is updated with its buffer value by the PWM synchronization."]
    _1 = 1,
}
impl From<INVC_A> for bool {
    #[inline(always)]
    fn from(variant: INVC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVC`"]
pub type INVC_R = crate::R<bool, INVC_A>;
impl INVC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVC_A {
        match self.bits {
            false => INVC_A::_0,
            true => INVC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVC_A::_1
    }
}
#[doc = "Write proxy for field `INVC`"]
pub struct INVC_W<'a> {
    w: &'a mut W,
}
impl<'a> INVC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "INVCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVC_A::_0)
    }
    #[doc = "INVCTRL register is updated with its buffer value by the PWM synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVC_A::_1)
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
#[doc = "SWOCTRL Register Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWOC_A {
    #[doc = "0: SWOCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    _0 = 0,
    #[doc = "1: SWOCTRL register is updated with its buffer value by the PWM synchronization."]
    _1 = 1,
}
impl From<SWOC_A> for bool {
    #[inline(always)]
    fn from(variant: SWOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWOC`"]
pub type SWOC_R = crate::R<bool, SWOC_A>;
impl SWOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWOC_A {
        match self.bits {
            false => SWOC_A::_0,
            true => SWOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWOC_A::_1
    }
}
#[doc = "Write proxy for field `SWOC`"]
pub struct SWOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SWOCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWOC_A::_0)
    }
    #[doc = "SWOCTRL register is updated with its buffer value by the PWM synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWOC_A::_1)
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
#[doc = "Synchronization Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCMODE_A {
    #[doc = "0: Legacy PWM synchronization is selected."]
    _0 = 0,
    #[doc = "1: Enhanced PWM synchronization is selected."]
    _1 = 1,
}
impl From<SYNCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCMODE`"]
pub type SYNCMODE_R = crate::R<bool, SYNCMODE_A>;
impl SYNCMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCMODE_A {
        match self.bits {
            false => SYNCMODE_A::_0,
            true => SYNCMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCMODE_A::_1
    }
}
#[doc = "Write proxy for field `SYNCMODE`"]
pub struct SYNCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Legacy PWM synchronization is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCMODE_A::_0)
    }
    #[doc = "Enhanced PWM synchronization is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCMODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "FTM counter synchronization is activated by the software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTCNT_A {
    #[doc = "0: The software trigger does not activate the FTM counter synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates the FTM counter synchronization."]
    _1 = 1,
}
impl From<SWRSTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTCNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRSTCNT`"]
pub type SWRSTCNT_R = crate::R<bool, SWRSTCNT_A>;
impl SWRSTCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTCNT_A {
        match self.bits {
            false => SWRSTCNT_A::_0,
            true => SWRSTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRSTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRSTCNT_A::_1
    }
}
#[doc = "Write proxy for field `SWRSTCNT`"]
pub struct SWRSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTCNT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software trigger does not activate the FTM counter synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTCNT_A::_0)
    }
    #[doc = "The software trigger activates the FTM counter synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTCNT_A::_1)
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
#[doc = "MOD, HCR, CNTIN, and CV registers synchronization is activated by the software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWWRBUF_A {
    #[doc = "0: The software trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates MOD, HCR, CNTIN, and CV registers synchronization."]
    _1 = 1,
}
impl From<SWWRBUF_A> for bool {
    #[inline(always)]
    fn from(variant: SWWRBUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWWRBUF`"]
pub type SWWRBUF_R = crate::R<bool, SWWRBUF_A>;
impl SWWRBUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWWRBUF_A {
        match self.bits {
            false => SWWRBUF_A::_0,
            true => SWWRBUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWWRBUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWWRBUF_A::_1
    }
}
#[doc = "Write proxy for field `SWWRBUF`"]
pub struct SWWRBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> SWWRBUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWWRBUF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWWRBUF_A::_0)
    }
    #[doc = "The software trigger activates MOD, HCR, CNTIN, and CV registers synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWWRBUF_A::_1)
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
#[doc = "Output mask synchronization is activated by the software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWOM_A {
    #[doc = "0: The software trigger does not activate the OUTMASK register synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates the OUTMASK register synchronization."]
    _1 = 1,
}
impl From<SWOM_A> for bool {
    #[inline(always)]
    fn from(variant: SWOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWOM`"]
pub type SWOM_R = crate::R<bool, SWOM_A>;
impl SWOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWOM_A {
        match self.bits {
            false => SWOM_A::_0,
            true => SWOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWOM_A::_1
    }
}
#[doc = "Write proxy for field `SWOM`"]
pub struct SWOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SWOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software trigger does not activate the OUTMASK register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWOM_A::_0)
    }
    #[doc = "The software trigger activates the OUTMASK register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWOM_A::_1)
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
#[doc = "Inverting control synchronization is activated by the software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWINVC_A {
    #[doc = "0: The software trigger does not activate the INVCTRL register synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates the INVCTRL register synchronization."]
    _1 = 1,
}
impl From<SWINVC_A> for bool {
    #[inline(always)]
    fn from(variant: SWINVC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWINVC`"]
pub type SWINVC_R = crate::R<bool, SWINVC_A>;
impl SWINVC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWINVC_A {
        match self.bits {
            false => SWINVC_A::_0,
            true => SWINVC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWINVC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWINVC_A::_1
    }
}
#[doc = "Write proxy for field `SWINVC`"]
pub struct SWINVC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWINVC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWINVC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software trigger does not activate the INVCTRL register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWINVC_A::_0)
    }
    #[doc = "The software trigger activates the INVCTRL register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWINVC_A::_1)
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
#[doc = "Software output control synchronization is activated by the software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSOC_A {
    #[doc = "0: The software trigger does not activate the SWOCTRL register synchronization."]
    _0 = 0,
    #[doc = "1: The software trigger activates the SWOCTRL register synchronization."]
    _1 = 1,
}
impl From<SWSOC_A> for bool {
    #[inline(always)]
    fn from(variant: SWSOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWSOC`"]
pub type SWSOC_R = crate::R<bool, SWSOC_A>;
impl SWSOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSOC_A {
        match self.bits {
            false => SWSOC_A::_0,
            true => SWSOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSOC_A::_1
    }
}
#[doc = "Write proxy for field `SWSOC`"]
pub struct SWSOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWSOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software trigger does not activate the SWOCTRL register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSOC_A::_0)
    }
    #[doc = "The software trigger activates the SWOCTRL register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSOC_A::_1)
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
#[doc = "FTM counter synchronization is activated by a hardware trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWRSTCNT_A {
    #[doc = "0: A hardware trigger does not activate the FTM counter synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates the FTM counter synchronization."]
    _1 = 1,
}
impl From<HWRSTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: HWRSTCNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HWRSTCNT`"]
pub type HWRSTCNT_R = crate::R<bool, HWRSTCNT_A>;
impl HWRSTCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWRSTCNT_A {
        match self.bits {
            false => HWRSTCNT_A::_0,
            true => HWRSTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWRSTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWRSTCNT_A::_1
    }
}
#[doc = "Write proxy for field `HWRSTCNT`"]
pub struct HWRSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HWRSTCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWRSTCNT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware trigger does not activate the FTM counter synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWRSTCNT_A::_0)
    }
    #[doc = "A hardware trigger activates the FTM counter synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWRSTCNT_A::_1)
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
#[doc = "MOD, HCR, CNTIN, and CV registers synchronization is activated by a hardware trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWWRBUF_A {
    #[doc = "0: A hardware trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates MOD, HCR, CNTIN, and CV registers synchronization."]
    _1 = 1,
}
impl From<HWWRBUF_A> for bool {
    #[inline(always)]
    fn from(variant: HWWRBUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HWWRBUF`"]
pub type HWWRBUF_R = crate::R<bool, HWWRBUF_A>;
impl HWWRBUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWWRBUF_A {
        match self.bits {
            false => HWWRBUF_A::_0,
            true => HWWRBUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWWRBUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWWRBUF_A::_1
    }
}
#[doc = "Write proxy for field `HWWRBUF`"]
pub struct HWWRBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> HWWRBUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWWRBUF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWWRBUF_A::_0)
    }
    #[doc = "A hardware trigger activates MOD, HCR, CNTIN, and CV registers synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWWRBUF_A::_1)
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
#[doc = "Output mask synchronization is activated by a hardware trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWOM_A {
    #[doc = "0: A hardware trigger does not activate the OUTMASK register synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates the OUTMASK register synchronization."]
    _1 = 1,
}
impl From<HWOM_A> for bool {
    #[inline(always)]
    fn from(variant: HWOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HWOM`"]
pub type HWOM_R = crate::R<bool, HWOM_A>;
impl HWOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWOM_A {
        match self.bits {
            false => HWOM_A::_0,
            true => HWOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWOM_A::_1
    }
}
#[doc = "Write proxy for field `HWOM`"]
pub struct HWOM_W<'a> {
    w: &'a mut W,
}
impl<'a> HWOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware trigger does not activate the OUTMASK register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWOM_A::_0)
    }
    #[doc = "A hardware trigger activates the OUTMASK register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWOM_A::_1)
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
#[doc = "Inverting control synchronization is activated by a hardware trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWINVC_A {
    #[doc = "0: A hardware trigger does not activate the INVCTRL register synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates the INVCTRL register synchronization."]
    _1 = 1,
}
impl From<HWINVC_A> for bool {
    #[inline(always)]
    fn from(variant: HWINVC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HWINVC`"]
pub type HWINVC_R = crate::R<bool, HWINVC_A>;
impl HWINVC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWINVC_A {
        match self.bits {
            false => HWINVC_A::_0,
            true => HWINVC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWINVC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWINVC_A::_1
    }
}
#[doc = "Write proxy for field `HWINVC`"]
pub struct HWINVC_W<'a> {
    w: &'a mut W,
}
impl<'a> HWINVC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWINVC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware trigger does not activate the INVCTRL register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWINVC_A::_0)
    }
    #[doc = "A hardware trigger activates the INVCTRL register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWINVC_A::_1)
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
#[doc = "Software output control synchronization is activated by a hardware trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWSOC_A {
    #[doc = "0: A hardware trigger does not activate the SWOCTRL register synchronization."]
    _0 = 0,
    #[doc = "1: A hardware trigger activates the SWOCTRL register synchronization."]
    _1 = 1,
}
impl From<HWSOC_A> for bool {
    #[inline(always)]
    fn from(variant: HWSOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HWSOC`"]
pub type HWSOC_R = crate::R<bool, HWSOC_A>;
impl HWSOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWSOC_A {
        match self.bits {
            false => HWSOC_A::_0,
            true => HWSOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWSOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWSOC_A::_1
    }
}
#[doc = "Write proxy for field `HWSOC`"]
pub struct HWSOC_W<'a> {
    w: &'a mut W,
}
impl<'a> HWSOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWSOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware trigger does not activate the SWOCTRL register synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWSOC_A::_0)
    }
    #[doc = "A hardware trigger activates the SWOCTRL register synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWSOC_A::_1)
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
impl R {
    #[doc = "Bit 0 - Hardware Trigger Mode"]
    #[inline(always)]
    pub fn hwtrigmode(&self) -> HWTRIGMODE_R {
        HWTRIGMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CNTIN Register Synchronization"]
    #[inline(always)]
    pub fn cntinc(&self) -> CNTINC_R {
        CNTINC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INVCTRL Register Synchronization"]
    #[inline(always)]
    pub fn invc(&self) -> INVC_R {
        INVC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SWOCTRL Register Synchronization"]
    #[inline(always)]
    pub fn swoc(&self) -> SWOC_R {
        SWOC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Synchronization Mode"]
    #[inline(always)]
    pub fn syncmode(&self) -> SYNCMODE_R {
        SYNCMODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FTM counter synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swrstcnt(&self) -> SWRSTCNT_R {
        SWRSTCNT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MOD, HCR, CNTIN, and CV registers synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swwrbuf(&self) -> SWWRBUF_R {
        SWWRBUF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output mask synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swom(&self) -> SWOM_R {
        SWOM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Inverting control synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swinvc(&self) -> SWINVC_R {
        SWINVC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software output control synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swsoc(&self) -> SWSOC_R {
        SWSOC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FTM counter synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwrstcnt(&self) -> HWRSTCNT_R {
        HWRSTCNT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MOD, HCR, CNTIN, and CV registers synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwwrbuf(&self) -> HWWRBUF_R {
        HWWRBUF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output mask synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwom(&self) -> HWOM_R {
        HWOM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Inverting control synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwinvc(&self) -> HWINVC_R {
        HWINVC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Software output control synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwsoc(&self) -> HWSOC_R {
        HWSOC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware Trigger Mode"]
    #[inline(always)]
    pub fn hwtrigmode(&mut self) -> HWTRIGMODE_W {
        HWTRIGMODE_W { w: self }
    }
    #[doc = "Bit 2 - CNTIN Register Synchronization"]
    #[inline(always)]
    pub fn cntinc(&mut self) -> CNTINC_W {
        CNTINC_W { w: self }
    }
    #[doc = "Bit 4 - INVCTRL Register Synchronization"]
    #[inline(always)]
    pub fn invc(&mut self) -> INVC_W {
        INVC_W { w: self }
    }
    #[doc = "Bit 5 - SWOCTRL Register Synchronization"]
    #[inline(always)]
    pub fn swoc(&mut self) -> SWOC_W {
        SWOC_W { w: self }
    }
    #[doc = "Bit 7 - Synchronization Mode"]
    #[inline(always)]
    pub fn syncmode(&mut self) -> SYNCMODE_W {
        SYNCMODE_W { w: self }
    }
    #[doc = "Bit 8 - FTM counter synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swrstcnt(&mut self) -> SWRSTCNT_W {
        SWRSTCNT_W { w: self }
    }
    #[doc = "Bit 9 - MOD, HCR, CNTIN, and CV registers synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swwrbuf(&mut self) -> SWWRBUF_W {
        SWWRBUF_W { w: self }
    }
    #[doc = "Bit 10 - Output mask synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swom(&mut self) -> SWOM_W {
        SWOM_W { w: self }
    }
    #[doc = "Bit 11 - Inverting control synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swinvc(&mut self) -> SWINVC_W {
        SWINVC_W { w: self }
    }
    #[doc = "Bit 12 - Software output control synchronization is activated by the software trigger"]
    #[inline(always)]
    pub fn swsoc(&mut self) -> SWSOC_W {
        SWSOC_W { w: self }
    }
    #[doc = "Bit 16 - FTM counter synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwrstcnt(&mut self) -> HWRSTCNT_W {
        HWRSTCNT_W { w: self }
    }
    #[doc = "Bit 17 - MOD, HCR, CNTIN, and CV registers synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwwrbuf(&mut self) -> HWWRBUF_W {
        HWWRBUF_W { w: self }
    }
    #[doc = "Bit 18 - Output mask synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwom(&mut self) -> HWOM_W {
        HWOM_W { w: self }
    }
    #[doc = "Bit 19 - Inverting control synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwinvc(&mut self) -> HWINVC_W {
        HWINVC_W { w: self }
    }
    #[doc = "Bit 20 - Software output control synchronization is activated by a hardware trigger"]
    #[inline(always)]
    pub fn hwsoc(&mut self) -> HWSOC_W {
        HWSOC_W { w: self }
    }
}
