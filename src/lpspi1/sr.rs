#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Transmit Data Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDF_A {
    #[doc = "0: Transmit data not requested."]
    _0 = 0,
    #[doc = "1: Transmit data is requested."]
    _1 = 1,
}
impl From<TDF_A> for bool {
    #[inline(always)]
    fn from(variant: TDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDF`"]
pub type TDF_R = crate::R<bool, TDF_A>;
impl TDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDF_A {
        match self.bits {
            false => TDF_A::_0,
            true => TDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDF_A::_1
    }
}
#[doc = "Receive Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDF_A {
    #[doc = "0: Receive Data is not ready."]
    _0 = 0,
    #[doc = "1: Receive data is ready."]
    _1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDF`"]
pub type RDF_R = crate::R<bool, RDF_A>;
impl RDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::_0,
            true => RDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDF_A::_1
    }
}
#[doc = "Word Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCF_A {
    #[doc = "0: Transfer word not completed."]
    _0 = 0,
    #[doc = "1: Transfer word completed."]
    _1 = 1,
}
impl From<WCF_A> for bool {
    #[inline(always)]
    fn from(variant: WCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCF`"]
pub type WCF_R = crate::R<bool, WCF_A>;
impl WCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCF_A {
        match self.bits {
            false => WCF_A::_0,
            true => WCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WCF_A::_1
    }
}
#[doc = "Write proxy for field `WCF`"]
pub struct WCF_W<'a> {
    w: &'a mut W,
}
impl<'a> WCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer word not completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WCF_A::_0)
    }
    #[doc = "Transfer word completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WCF_A::_1)
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
#[doc = "Frame Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCF_A {
    #[doc = "0: Frame transfer has not completed."]
    _0 = 0,
    #[doc = "1: Frame transfer has completed."]
    _1 = 1,
}
impl From<FCF_A> for bool {
    #[inline(always)]
    fn from(variant: FCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCF`"]
pub type FCF_R = crate::R<bool, FCF_A>;
impl FCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCF_A {
        match self.bits {
            false => FCF_A::_0,
            true => FCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCF_A::_1
    }
}
#[doc = "Write proxy for field `FCF`"]
pub struct FCF_W<'a> {
    w: &'a mut W,
}
impl<'a> FCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame transfer has not completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCF_A::_0)
    }
    #[doc = "Frame transfer has completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCF_A::_1)
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
#[doc = "Transfer Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    #[doc = "0: All transfers have not completed."]
    _0 = 0,
    #[doc = "1: All transfers have completed."]
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
    #[doc = "All transfers have not completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_A::_0)
    }
    #[doc = "All transfers have completed."]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Transmit Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEF_A {
    #[doc = "0: Transmit FIFO underrun has not occurred."]
    _0 = 0,
    #[doc = "1: Transmit FIFO underrun has occurred"]
    _1 = 1,
}
impl From<TEF_A> for bool {
    #[inline(always)]
    fn from(variant: TEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEF`"]
pub type TEF_R = crate::R<bool, TEF_A>;
impl TEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEF_A {
        match self.bits {
            false => TEF_A::_0,
            true => TEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEF_A::_1
    }
}
#[doc = "Write proxy for field `TEF`"]
pub struct TEF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit FIFO underrun has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEF_A::_0)
    }
    #[doc = "Transmit FIFO underrun has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEF_A::_1)
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
#[doc = "Receive Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_A {
    #[doc = "0: Receive FIFO has not overflowed."]
    _0 = 0,
    #[doc = "1: Receive FIFO has overflowed."]
    _1 = 1,
}
impl From<REF_A> for bool {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REF`"]
pub type REF_R = crate::R<bool, REF_A>;
impl REF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_A {
        match self.bits {
            false => REF_A::_0,
            true => REF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REF_A::_1
    }
}
#[doc = "Write proxy for field `REF`"]
pub struct REF_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive FIFO has not overflowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REF_A::_0)
    }
    #[doc = "Receive FIFO has overflowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REF_A::_1)
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
#[doc = "Data Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMF_A {
    #[doc = "0: Have not received matching data."]
    _0 = 0,
    #[doc = "1: Have received matching data."]
    _1 = 1,
}
impl From<DMF_A> for bool {
    #[inline(always)]
    fn from(variant: DMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMF`"]
pub type DMF_R = crate::R<bool, DMF_A>;
impl DMF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMF_A {
        match self.bits {
            false => DMF_A::_0,
            true => DMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMF_A::_1
    }
}
#[doc = "Write proxy for field `DMF`"]
pub struct DMF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Have not received matching data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMF_A::_0)
    }
    #[doc = "Have received matching data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMF_A::_1)
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
#[doc = "Module Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBF_A {
    #[doc = "0: LPSPI is idle."]
    _0 = 0,
    #[doc = "1: LPSPI is busy."]
    _1 = 1,
}
impl From<MBF_A> for bool {
    #[inline(always)]
    fn from(variant: MBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MBF`"]
pub type MBF_R = crate::R<bool, MBF_A>;
impl MBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBF_A {
        match self.bits {
            false => MBF_A::_0,
            true => MBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MBF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data Flag"]
    #[inline(always)]
    pub fn tdf(&self) -> TDF_R {
        TDF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Word Complete Flag"]
    #[inline(always)]
    pub fn wcf(&self) -> WCF_R {
        WCF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame Complete Flag"]
    #[inline(always)]
    pub fn fcf(&self) -> FCF_R {
        FCF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Error Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Error Flag"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data Match Flag"]
    #[inline(always)]
    pub fn dmf(&self) -> DMF_R {
        DMF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Module Busy Flag"]
    #[inline(always)]
    pub fn mbf(&self) -> MBF_R {
        MBF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Word Complete Flag"]
    #[inline(always)]
    pub fn wcf(&mut self) -> WCF_W {
        WCF_W { w: self }
    }
    #[doc = "Bit 9 - Frame Complete Flag"]
    #[inline(always)]
    pub fn fcf(&mut self) -> FCF_W {
        FCF_W { w: self }
    }
    #[doc = "Bit 10 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Error Flag"]
    #[inline(always)]
    pub fn tef(&mut self) -> TEF_W {
        TEF_W { w: self }
    }
    #[doc = "Bit 12 - Receive Error Flag"]
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W {
        REF_W { w: self }
    }
    #[doc = "Bit 13 - Data Match Flag"]
    #[inline(always)]
    pub fn dmf(&mut self) -> DMF_W {
        DMF_W { w: self }
    }
}
