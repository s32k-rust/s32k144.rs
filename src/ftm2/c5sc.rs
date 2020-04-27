#[doc = "Reader of register C5SC"]
pub type R = crate::R<u32, super::C5SC>;
#[doc = "Writer for register C5SC"]
pub type W = crate::W<u32, super::C5SC>;
#[doc = "Register C5SC `reset()`'s with value 0"]
impl crate::ResetValue for super::C5SC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Disable DMA transfers."]
    _0 = 0,
    #[doc = "1: Enable DMA transfers."]
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
    #[doc = "Disable DMA transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "Enable DMA transfers."]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "FTM counter reset by the selected input capture event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICRST_A {
    #[doc = "0: FTM counter is not reset when the selected channel (n) input event is detected."]
    _0 = 0,
    #[doc = "1: FTM counter is reset when the selected channel (n) input event is detected."]
    _1 = 1,
}
impl From<ICRST_A> for bool {
    #[inline(always)]
    fn from(variant: ICRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICRST`"]
pub type ICRST_R = crate::R<bool, ICRST_A>;
impl ICRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICRST_A {
        match self.bits {
            false => ICRST_A::_0,
            true => ICRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICRST_A::_1
    }
}
#[doc = "Write proxy for field `ICRST`"]
pub struct ICRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM counter is not reset when the selected channel (n) input event is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICRST_A::_0)
    }
    #[doc = "FTM counter is reset when the selected channel (n) input event is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICRST_A::_1)
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
#[doc = "Reader of field `ELSA`"]
pub type ELSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELSA`"]
pub struct ELSA_W<'a> {
    w: &'a mut W,
}
impl<'a> ELSA_W<'a> {
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
#[doc = "Reader of field `ELSB`"]
pub type ELSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELSB`"]
pub struct ELSB_W<'a> {
    w: &'a mut W,
}
impl<'a> ELSB_W<'a> {
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
#[doc = "Reader of field `MSA`"]
pub type MSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSA`"]
pub struct MSA_W<'a> {
    w: &'a mut W,
}
impl<'a> MSA_W<'a> {
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
#[doc = "Reader of field `MSB`"]
pub type MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSB`"]
pub struct MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MSB_W<'a> {
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
#[doc = "Channel (n) Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHIE_A {
    #[doc = "0: Disable channel (n) interrupt. Use software polling."]
    _0 = 0,
    #[doc = "1: Enable channel (n) interrupt."]
    _1 = 1,
}
impl From<CHIE_A> for bool {
    #[inline(always)]
    fn from(variant: CHIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHIE`"]
pub type CHIE_R = crate::R<bool, CHIE_A>;
impl CHIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIE_A {
        match self.bits {
            false => CHIE_A::_0,
            true => CHIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHIE_A::_1
    }
}
#[doc = "Write proxy for field `CHIE`"]
pub struct CHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable channel (n) interrupt. Use software polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHIE_A::_0)
    }
    #[doc = "Enable channel (n) interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Channel (n) Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHF_A {
    #[doc = "0: No channel (n) event has occurred."]
    _0 = 0,
    #[doc = "1: A channel (n) event has occurred."]
    _1 = 1,
}
impl From<CHF_A> for bool {
    #[inline(always)]
    fn from(variant: CHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHF`"]
pub type CHF_R = crate::R<bool, CHF_A>;
impl CHF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHF_A {
        match self.bits {
            false => CHF_A::_0,
            true => CHF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHF_A::_1
    }
}
#[doc = "Trigger mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGMODE_A {
    #[doc = "0: Channel outputs will generate the normal PWM outputs without generating a pulse."]
    _0 = 0,
    #[doc = "1: If a match in the channel occurs, a trigger generation on channel output will happen. The trigger pulse width has one FTM clock cycle."]
    _1 = 1,
}
impl From<TRIGMODE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGMODE`"]
pub type TRIGMODE_R = crate::R<bool, TRIGMODE_A>;
impl TRIGMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGMODE_A {
        match self.bits {
            false => TRIGMODE_A::_0,
            true => TRIGMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIGMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIGMODE_A::_1
    }
}
#[doc = "Write proxy for field `TRIGMODE`"]
pub struct TRIGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel outputs will generate the normal PWM outputs without generating a pulse."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIGMODE_A::_0)
    }
    #[doc = "If a match in the channel occurs, a trigger generation on channel output will happen. The trigger pulse width has one FTM clock cycle."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIGMODE_A::_1)
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
#[doc = "Channel (n) Input State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHIS_A {
    #[doc = "0: The channel (n) input is zero."]
    _0 = 0,
    #[doc = "1: The channel (n) input is one."]
    _1 = 1,
}
impl From<CHIS_A> for bool {
    #[inline(always)]
    fn from(variant: CHIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHIS`"]
pub type CHIS_R = crate::R<bool, CHIS_A>;
impl CHIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIS_A {
        match self.bits {
            false => CHIS_A::_0,
            true => CHIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHIS_A::_1
    }
}
#[doc = "Channel (n) Output Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHOV_A {
    #[doc = "0: The channel (n) output is zero."]
    _0 = 0,
    #[doc = "1: The channel (n) output is one."]
    _1 = 1,
}
impl From<CHOV_A> for bool {
    #[inline(always)]
    fn from(variant: CHOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHOV`"]
pub type CHOV_R = crate::R<bool, CHOV_A>;
impl CHOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHOV_A {
        match self.bits {
            false => CHOV_A::_0,
            true => CHOV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHOV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHOV_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FTM counter reset by the selected input capture event."]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel (n) Edge or Level Select"]
    #[inline(always)]
    pub fn elsa(&self) -> ELSA_R {
        ELSA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel (n) Edge or Level Select"]
    #[inline(always)]
    pub fn elsb(&self) -> ELSB_R {
        ELSB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel (n) Mode Select"]
    #[inline(always)]
    pub fn msa(&self) -> MSA_R {
        MSA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel (n) Mode Select"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel (n) Interrupt Enable"]
    #[inline(always)]
    pub fn chie(&self) -> CHIE_R {
        CHIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel (n) Flag"]
    #[inline(always)]
    pub fn chf(&self) -> CHF_R {
        CHF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Trigger mode control"]
    #[inline(always)]
    pub fn trigmode(&self) -> TRIGMODE_R {
        TRIGMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel (n) Input State"]
    #[inline(always)]
    pub fn chis(&self) -> CHIS_R {
        CHIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel (n) Output Value"]
    #[inline(always)]
    pub fn chov(&self) -> CHOV_R {
        CHOV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 1 - FTM counter reset by the selected input capture event."]
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W {
        ICRST_W { w: self }
    }
    #[doc = "Bit 2 - Channel (n) Edge or Level Select"]
    #[inline(always)]
    pub fn elsa(&mut self) -> ELSA_W {
        ELSA_W { w: self }
    }
    #[doc = "Bit 3 - Channel (n) Edge or Level Select"]
    #[inline(always)]
    pub fn elsb(&mut self) -> ELSB_W {
        ELSB_W { w: self }
    }
    #[doc = "Bit 4 - Channel (n) Mode Select"]
    #[inline(always)]
    pub fn msa(&mut self) -> MSA_W {
        MSA_W { w: self }
    }
    #[doc = "Bit 5 - Channel (n) Mode Select"]
    #[inline(always)]
    pub fn msb(&mut self) -> MSB_W {
        MSB_W { w: self }
    }
    #[doc = "Bit 6 - Channel (n) Interrupt Enable"]
    #[inline(always)]
    pub fn chie(&mut self) -> CHIE_W {
        CHIE_W { w: self }
    }
    #[doc = "Bit 8 - Trigger mode control"]
    #[inline(always)]
    pub fn trigmode(&mut self) -> TRIGMODE_W {
        TRIGMODE_W { w: self }
    }
}
