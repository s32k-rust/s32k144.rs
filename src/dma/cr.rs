#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EDBG`"]
pub type EDBG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDBG`"]
pub struct EDBG_W<'a> {
    w: &'a mut W,
}
impl<'a> EDBG_W<'a> {
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
#[doc = "Reader of field `ERCA`"]
pub type ERCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERCA`"]
pub struct ERCA_W<'a> {
    w: &'a mut W,
}
impl<'a> ERCA_W<'a> {
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
#[doc = "Halt On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOE_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    _1 = 1,
}
impl From<HOE_A> for bool {
    #[inline(always)]
    fn from(variant: HOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HOE`"]
pub type HOE_R = crate::R<bool, HOE_A>;
impl HOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOE_A {
        match self.bits {
            false => HOE_A::_0,
            true => HOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOE_A::_1
    }
}
#[doc = "Write proxy for field `HOE`"]
pub struct HOE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOE_A::_0)
    }
    #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOE_A::_1)
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
#[doc = "Halt DMA Operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
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
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Continuous Link Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLM_A {
    #[doc = "0: A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    _0 = 0,
    #[doc = "1: A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    _1 = 1,
}
impl From<CLM_A> for bool {
    #[inline(always)]
    fn from(variant: CLM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLM`"]
pub type CLM_R = crate::R<bool, CLM_A>;
impl CLM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLM_A {
        match self.bits {
            false => CLM_A::_0,
            true => CLM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLM_A::_1
    }
}
#[doc = "Write proxy for field `CLM`"]
pub struct CLM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLM_A::_0)
    }
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLM_A::_1)
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
#[doc = "Enable Minor Loop Mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMLM_A {
    #[doc = "0: Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    _0 = 0,
    #[doc = "1: Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    _1 = 1,
}
impl From<EMLM_A> for bool {
    #[inline(always)]
    fn from(variant: EMLM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMLM`"]
pub type EMLM_R = crate::R<bool, EMLM_A>;
impl EMLM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMLM_A {
        match self.bits {
            false => EMLM_A::_0,
            true => EMLM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMLM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMLM_A::_1
    }
}
#[doc = "Write proxy for field `EMLM`"]
pub struct EMLM_W<'a> {
    w: &'a mut W,
}
impl<'a> EMLM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMLM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMLM_A::_0)
    }
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMLM_A::_1)
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
#[doc = "Error Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECX_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt."]
    _1 = 1,
}
impl From<ECX_A> for bool {
    #[inline(always)]
    fn from(variant: ECX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECX`"]
pub type ECX_R = crate::R<bool, ECX_A>;
impl ECX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECX_A {
        match self.bits {
            false => ECX_A::_0,
            true => ECX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECX_A::_1
    }
}
#[doc = "Write proxy for field `ECX`"]
pub struct ECX_W<'a> {
    w: &'a mut W,
}
impl<'a> ECX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECX_A::_0)
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECX_A::_1)
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
#[doc = "Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CX_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    _1 = 1,
}
impl From<CX_A> for bool {
    #[inline(always)]
    fn from(variant: CX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CX`"]
pub type CX_R = crate::R<bool, CX_A>;
impl CX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CX_A {
        match self.bits {
            false => CX_A::_0,
            true => CX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CX_A::_1
    }
}
#[doc = "Write proxy for field `CX`"]
pub struct CX_W<'a> {
    w: &'a mut W,
}
impl<'a> CX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CX_A::_0)
    }
    #[doc = "Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CX_A::_1)
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
impl R {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    pub fn edbg(&self) -> EDBG_R {
        EDBG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub fn erca(&self) -> ERCA_R {
        ERCA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    pub fn hoe(&self) -> HOE_R {
        HOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    pub fn clm(&self) -> CLM_R {
        CLM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    pub fn emlm(&self) -> EMLM_R {
        EMLM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    pub fn ecx(&self) -> ECX_R {
        ECX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    pub fn cx(&self) -> CX_R {
        CX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    pub fn edbg(&mut self) -> EDBG_W {
        EDBG_W { w: self }
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub fn erca(&mut self) -> ERCA_W {
        ERCA_W { w: self }
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    pub fn hoe(&mut self) -> HOE_W {
        HOE_W { w: self }
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    pub fn clm(&mut self) -> CLM_W {
        CLM_W { w: self }
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    pub fn emlm(&mut self) -> EMLM_W {
        EMLM_W { w: self }
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    pub fn ecx(&mut self) -> ECX_W {
        ECX_W { w: self }
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    pub fn cx(&mut self) -> CX_W {
        CX_W { w: self }
    }
}
