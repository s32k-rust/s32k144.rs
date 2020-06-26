#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
    #[doc = "0: LPTMR is disabled and internal logic is reset."]
    _0 = 0,
    #[doc = "1: LPTMR is enabled."]
    _1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEN`"]
pub type TEN_R = crate::R<bool, TEN_A>;
impl TEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::_0,
            true => TEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEN_A::_1
    }
}
#[doc = "Write proxy for field `TEN`"]
pub struct TEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPTMR is disabled and internal logic is reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEN_A::_0)
    }
    #[doc = "LPTMR is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEN_A::_1)
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
#[doc = "Timer Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMS_A {
    #[doc = "0: Time Counter mode."]
    _0 = 0,
    #[doc = "1: Pulse Counter mode."]
    _1 = 1,
}
impl From<TMS_A> for bool {
    #[inline(always)]
    fn from(variant: TMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMS`"]
pub type TMS_R = crate::R<bool, TMS_A>;
impl TMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMS_A {
        match self.bits {
            false => TMS_A::_0,
            true => TMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMS_A::_1
    }
}
#[doc = "Write proxy for field `TMS`"]
pub struct TMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Time Counter mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMS_A::_0)
    }
    #[doc = "Pulse Counter mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMS_A::_1)
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
#[doc = "Timer Free-Running Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFC_A {
    #[doc = "0: CNR is reset whenever TCF is set."]
    _0 = 0,
    #[doc = "1: CNR is reset on overflow."]
    _1 = 1,
}
impl From<TFC_A> for bool {
    #[inline(always)]
    fn from(variant: TFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TFC`"]
pub type TFC_R = crate::R<bool, TFC_A>;
impl TFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFC_A {
        match self.bits {
            false => TFC_A::_0,
            true => TFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFC_A::_1
    }
}
#[doc = "Write proxy for field `TFC`"]
pub struct TFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CNR is reset whenever TCF is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFC_A::_0)
    }
    #[doc = "CNR is reset on overflow."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFC_A::_1)
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
#[doc = "Timer Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPP_A {
    #[doc = "0: Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    _0 = 0,
    #[doc = "1: Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    _1 = 1,
}
impl From<TPP_A> for bool {
    #[inline(always)]
    fn from(variant: TPP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPP`"]
pub type TPP_R = crate::R<bool, TPP_A>;
impl TPP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPP_A {
        match self.bits {
            false => TPP_A::_0,
            true => TPP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPP_A::_1
    }
}
#[doc = "Write proxy for field `TPP`"]
pub struct TPP_W<'a> {
    w: &'a mut W,
}
impl<'a> TPP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPP_A::_0)
    }
    #[doc = "Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPP_A::_1)
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
#[doc = "Timer Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPS_A {
    #[doc = "0: Pulse counter input 0 is selected."]
    _00 = 0,
    #[doc = "1: Pulse counter input 1 is selected."]
    _01 = 1,
    #[doc = "2: Pulse counter input 2 is selected."]
    _10 = 2,
    #[doc = "3: Pulse counter input 3 is selected."]
    _11 = 3,
}
impl From<TPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TPS`"]
pub type TPS_R = crate::R<u8, TPS_A>;
impl TPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPS_A {
        match self.bits {
            0 => TPS_A::_00,
            1 => TPS_A::_01,
            2 => TPS_A::_10,
            3 => TPS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TPS_A::_11
    }
}
#[doc = "Write proxy for field `TPS`"]
pub struct TPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pulse counter input 0 is selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPS_A::_00)
    }
    #[doc = "Pulse counter input 1 is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPS_A::_01)
    }
    #[doc = "Pulse counter input 2 is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPS_A::_10)
    }
    #[doc = "Pulse counter input 3 is selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Timer interrupt disabled."]
    _0 = 0,
    #[doc = "1: Timer interrupt enabled."]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Timer interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
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
#[doc = "Timer Compare Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    #[doc = "0: The value of CNR is not equal to CMR and increments."]
    _0 = 0,
    #[doc = "1: The value of CNR is equal to CMR and increments."]
    _1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, TCF_A>;
impl TCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::_0,
            true => TCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCF_A::_1
    }
}
#[doc = "Write proxy for field `TCF`"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The value of CNR is not equal to CMR and increments."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_A::_0)
    }
    #[doc = "The value of CNR is equal to CMR and increments."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCF_A::_1)
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
#[doc = "Timer DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
    #[doc = "0: Timer DMA Request disabled."]
    _0 = 0,
    #[doc = "1: Timer DMA Request enabled."]
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
#[doc = "Write proxy for field `TDRE`"]
pub struct TDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer DMA Request disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDRE_A::_0)
    }
    #[doc = "Timer DMA Request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDRE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Mode Select"]
    #[inline(always)]
    pub fn tms(&self) -> TMS_R {
        TMS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer Free-Running Counter"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn tpp(&self) -> TPP_R {
        TPP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Timer Pin Select"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Compare Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W { w: self }
    }
    #[doc = "Bit 1 - Timer Mode Select"]
    #[inline(always)]
    pub fn tms(&mut self) -> TMS_W {
        TMS_W { w: self }
    }
    #[doc = "Bit 2 - Timer Free-Running Counter"]
    #[inline(always)]
    pub fn tfc(&mut self) -> TFC_W {
        TFC_W { w: self }
    }
    #[doc = "Bit 3 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn tpp(&mut self) -> TPP_W {
        TPP_W { w: self }
    }
    #[doc = "Bits 4:5 - Timer Pin Select"]
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W {
        TPS_W { w: self }
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 7 - Timer Compare Flag"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    #[doc = "Bit 8 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TDRE_W {
        TDRE_W { w: self }
    }
}
