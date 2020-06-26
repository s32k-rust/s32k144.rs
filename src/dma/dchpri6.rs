#[doc = "Reader of register DCHPRI6"]
pub type R = crate::R<u8, super::DCHPRI6>;
#[doc = "Writer for register DCHPRI6"]
pub type W = crate::W<u8, super::DCHPRI6>;
#[doc = "Register DCHPRI6 `reset()`'s with value 0x06"]
impl crate::ResetValue for super::DCHPRI6 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Reader of field `CHPRI`"]
pub type CHPRI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHPRI`"]
pub struct CHPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Disable Preempt Ability. This field resets to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPA_A {
    #[doc = "0: Channel n can suspend a lower priority channel."]
    _0 = 0,
    #[doc = "1: Channel n cannot suspend any channel, regardless of channel priority."]
    _1 = 1,
}
impl From<DPA_A> for bool {
    #[inline(always)]
    fn from(variant: DPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPA`"]
pub type DPA_R = crate::R<bool, DPA_A>;
impl DPA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPA_A {
        match self.bits {
            false => DPA_A::_0,
            true => DPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPA_A::_1
    }
}
#[doc = "Write proxy for field `DPA`"]
pub struct DPA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel n can suspend a lower priority channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPA_A::_0)
    }
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable Channel Preemption. This field resets to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECP_A {
    #[doc = "0: Channel n cannot be suspended by a higher priority channel's service request."]
    _0 = 0,
    #[doc = "1: Channel n can be temporarily suspended by the service request of a higher priority channel."]
    _1 = 1,
}
impl From<ECP_A> for bool {
    #[inline(always)]
    fn from(variant: ECP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECP`"]
pub type ECP_R = crate::R<bool, ECP_A>;
impl ECP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECP_A {
        match self.bits {
            false => ECP_A::_0,
            true => ECP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECP_A::_1
    }
}
#[doc = "Write proxy for field `ECP`"]
pub struct ECP_W<'a> {
    w: &'a mut W,
}
impl<'a> ECP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECP_A::_0)
    }
    #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline(always)]
    pub fn chpri(&self) -> CHPRI_R {
        CHPRI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub fn dpa(&self) -> DPA_R {
        DPA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub fn ecp(&self) -> ECP_R {
        ECP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline(always)]
    pub fn chpri(&mut self) -> CHPRI_W {
        CHPRI_W { w: self }
    }
    #[doc = "Bit 6 - Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub fn dpa(&mut self) -> DPA_W {
        DPA_W { w: self }
    }
    #[doc = "Bit 7 - Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub fn ecp(&mut self) -> ECP_W {
        ECP_W { w: self }
    }
}
