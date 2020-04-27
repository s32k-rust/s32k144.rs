#[doc = "Reader of register CPCR"]
pub type R = crate::R<u32, super::CPCR>;
#[doc = "Writer for register CPCR"]
pub type W = crate::W<u32, super::CPCR>;
#[doc = "Register CPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AXBS Halt State Machine Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HLT_FSM_ST_A {
    #[doc = "0: Waiting for request"]
    _00 = 0,
    #[doc = "1: Waiting for platform idle"]
    _01 = 1,
    #[doc = "3: Platform stalled"]
    _11 = 3,
    #[doc = "2: Unused state"]
    _10 = 2,
}
impl From<HLT_FSM_ST_A> for u8 {
    #[inline(always)]
    fn from(variant: HLT_FSM_ST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HLT_FSM_ST`"]
pub type HLT_FSM_ST_R = crate::R<u8, HLT_FSM_ST_A>;
impl HLT_FSM_ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HLT_FSM_ST_A {
        match self.bits {
            0 => HLT_FSM_ST_A::_00,
            1 => HLT_FSM_ST_A::_01,
            3 => HLT_FSM_ST_A::_11,
            2 => HLT_FSM_ST_A::_10,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == HLT_FSM_ST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == HLT_FSM_ST_A::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == HLT_FSM_ST_A::_11
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == HLT_FSM_ST_A::_10
    }
}
#[doc = "AXBS Halt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_HLT_REQ_A {
    #[doc = "0: AXBS is not receiving halt request"]
    _0 = 0,
    #[doc = "1: AXBS is receiving halt request"]
    _1 = 1,
}
impl From<AXBS_HLT_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_HLT_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_HLT_REQ`"]
pub type AXBS_HLT_REQ_R = crate::R<bool, AXBS_HLT_REQ_A>;
impl AXBS_HLT_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_HLT_REQ_A {
        match self.bits {
            false => AXBS_HLT_REQ_A::_0,
            true => AXBS_HLT_REQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AXBS_HLT_REQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AXBS_HLT_REQ_A::_1
    }
}
#[doc = "AXBS Halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_HLTD_A {
    #[doc = "0: AXBS is not currently halted"]
    _0 = 0,
    #[doc = "1: AXBS is currently halted"]
    _1 = 1,
}
impl From<AXBS_HLTD_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_HLTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_HLTD`"]
pub type AXBS_HLTD_R = crate::R<bool, AXBS_HLTD_A>;
impl AXBS_HLTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_HLTD_A {
        match self.bits {
            false => AXBS_HLTD_A::_0,
            true => AXBS_HLTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AXBS_HLTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AXBS_HLTD_A::_1
    }
}
#[doc = "Flash Memory Controller Program Flash Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMC_PF_IDLE_A {
    #[doc = "0: FMC program flash is not idle"]
    _0 = 0,
    #[doc = "1: FMC program flash is currently idle"]
    _1 = 1,
}
impl From<FMC_PF_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: FMC_PF_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FMC_PF_IDLE`"]
pub type FMC_PF_IDLE_R = crate::R<bool, FMC_PF_IDLE_A>;
impl FMC_PF_IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMC_PF_IDLE_A {
        match self.bits {
            false => FMC_PF_IDLE_A::_0,
            true => FMC_PF_IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FMC_PF_IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FMC_PF_IDLE_A::_1
    }
}
#[doc = "Peripheral Bridge Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBRIDGE_IDLE_A {
    #[doc = "0: PBRIDGE is not idle"]
    _0 = 0,
    #[doc = "1: PBRIDGE is currently idle"]
    _1 = 1,
}
impl From<PBRIDGE_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: PBRIDGE_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PBRIDGE_IDLE`"]
pub type PBRIDGE_IDLE_R = crate::R<bool, PBRIDGE_IDLE_A>;
impl PBRIDGE_IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBRIDGE_IDLE_A {
        match self.bits {
            false => PBRIDGE_IDLE_A::_0,
            true => PBRIDGE_IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PBRIDGE_IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PBRIDGE_IDLE_A::_1
    }
}
#[doc = "Crossbar Round-robin Arbitration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBRR_A {
    #[doc = "0: Fixed-priority arbitration"]
    _0 = 0,
    #[doc = "1: Round-robin arbitration"]
    _1 = 1,
}
impl From<CBRR_A> for bool {
    #[inline(always)]
    fn from(variant: CBRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CBRR`"]
pub type CBRR_R = crate::R<bool, CBRR_A>;
impl CBRR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBRR_A {
        match self.bits {
            false => CBRR_A::_0,
            true => CBRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CBRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CBRR_A::_1
    }
}
#[doc = "Write proxy for field `CBRR`"]
pub struct CBRR_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBRR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fixed-priority arbitration"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CBRR_A::_0)
    }
    #[doc = "Round-robin arbitration"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CBRR_A::_1)
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
#[doc = "SRAM_U Arbitration Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMUAP_A {
    #[doc = "0: Round robin"]
    _00 = 0,
    #[doc = "1: Special round robin (favors SRAM backdoor accesses over the processor)"]
    _01 = 1,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    _10 = 2,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    _11 = 3,
}
impl From<SRAMUAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMUAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMUAP`"]
pub type SRAMUAP_R = crate::R<u8, SRAMUAP_A>;
impl SRAMUAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMUAP_A {
        match self.bits {
            0 => SRAMUAP_A::_00,
            1 => SRAMUAP_A::_01,
            2 => SRAMUAP_A::_10,
            3 => SRAMUAP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SRAMUAP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SRAMUAP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SRAMUAP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SRAMUAP_A::_11
    }
}
#[doc = "Write proxy for field `SRAMUAP`"]
pub struct SRAMUAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMUAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMUAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_00)
    }
    #[doc = "Special round robin (favors SRAM backdoor accesses over the processor)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SRAMUWP`"]
pub type SRAMUWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMUWP`"]
pub struct SRAMUWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMUWP_W<'a> {
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
#[doc = "SRAM_L Arbitration Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMLAP_A {
    #[doc = "0: Round robin"]
    _00 = 0,
    #[doc = "1: Special round robin (favors SRAM backdoor accesses over the processor)"]
    _01 = 1,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    _10 = 2,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    _11 = 3,
}
impl From<SRAMLAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMLAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMLAP`"]
pub type SRAMLAP_R = crate::R<u8, SRAMLAP_A>;
impl SRAMLAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMLAP_A {
        match self.bits {
            0 => SRAMLAP_A::_00,
            1 => SRAMLAP_A::_01,
            2 => SRAMLAP_A::_10,
            3 => SRAMLAP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SRAMLAP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SRAMLAP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SRAMLAP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SRAMLAP_A::_11
    }
}
#[doc = "Write proxy for field `SRAMLAP`"]
pub struct SRAMLAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMLAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_00)
    }
    #[doc = "Special round robin (favors SRAM backdoor accesses over the processor)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `SRAMLWP`"]
pub type SRAMLWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMLWP`"]
pub struct SRAMLWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLWP_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - AXBS Halt State Machine Status"]
    #[inline(always)]
    pub fn hlt_fsm_st(&self) -> HLT_FSM_ST_R {
        HLT_FSM_ST_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - AXBS Halt Request"]
    #[inline(always)]
    pub fn axbs_hlt_req(&self) -> AXBS_HLT_REQ_R {
        AXBS_HLT_REQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXBS Halted"]
    #[inline(always)]
    pub fn axbs_hltd(&self) -> AXBS_HLTD_R {
        AXBS_HLTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Memory Controller Program Flash Idle"]
    #[inline(always)]
    pub fn fmc_pf_idle(&self) -> FMC_PF_IDLE_R {
        FMC_PF_IDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral Bridge Idle"]
    #[inline(always)]
    pub fn pbridge_idle(&self) -> PBRIDGE_IDLE_R {
        PBRIDGE_IDLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Crossbar Round-robin Arbitration Enable"]
    #[inline(always)]
    pub fn cbrr(&self) -> CBRR_R {
        CBRR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - SRAM_U Arbitration Priority"]
    #[inline(always)]
    pub fn sramuap(&self) -> SRAMUAP_R {
        SRAMUAP_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - SRAM_U Write Protect"]
    #[inline(always)]
    pub fn sramuwp(&self) -> SRAMUWP_R {
        SRAMUWP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - SRAM_L Arbitration Priority"]
    #[inline(always)]
    pub fn sramlap(&self) -> SRAMLAP_R {
        SRAMLAP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline(always)]
    pub fn sramlwp(&self) -> SRAMLWP_R {
        SRAMLWP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Crossbar Round-robin Arbitration Enable"]
    #[inline(always)]
    pub fn cbrr(&mut self) -> CBRR_W {
        CBRR_W { w: self }
    }
    #[doc = "Bits 24:25 - SRAM_U Arbitration Priority"]
    #[inline(always)]
    pub fn sramuap(&mut self) -> SRAMUAP_W {
        SRAMUAP_W { w: self }
    }
    #[doc = "Bit 26 - SRAM_U Write Protect"]
    #[inline(always)]
    pub fn sramuwp(&mut self) -> SRAMUWP_W {
        SRAMUWP_W { w: self }
    }
    #[doc = "Bits 28:29 - SRAM_L Arbitration Priority"]
    #[inline(always)]
    pub fn sramlap(&mut self) -> SRAMLAP_W {
        SRAMLAP_W { w: self }
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline(always)]
    pub fn sramlwp(&mut self) -> SRAMLWP_W {
        SRAMLWP_W { w: self }
    }
}
