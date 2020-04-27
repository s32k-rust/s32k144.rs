#[doc = "Reader of register ES"]
pub type R = crate::R<u32, super::ES>;
#[doc = "Destination Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBE_A {
    #[doc = "0: No destination bus error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a bus error on a destination write"]
    _1 = 1,
}
impl From<DBE_A> for bool {
    #[inline(always)]
    fn from(variant: DBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBE`"]
pub type DBE_R = crate::R<bool, DBE_A>;
impl DBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBE_A {
        match self.bits {
            false => DBE_A::_0,
            true => DBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBE_A::_1
    }
}
#[doc = "Source Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBE_A {
    #[doc = "0: No source bus error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a bus error on a source read"]
    _1 = 1,
}
impl From<SBE_A> for bool {
    #[inline(always)]
    fn from(variant: SBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBE`"]
pub type SBE_R = crate::R<bool, SBE_A>;
impl SBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBE_A {
        match self.bits {
            false => SBE_A::_0,
            true => SBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBE_A::_1
    }
}
#[doc = "Scatter/Gather Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGE_A {
    #[doc = "0: No scatter/gather configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\\[ESG\\]
is enabled. TCDn_DLASTSGA is not on a 32 byte boundary."]
    _1 = 1,
}
impl From<SGE_A> for bool {
    #[inline(always)]
    fn from(variant: SGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SGE`"]
pub type SGE_R = crate::R<bool, SGE_A>;
impl SGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGE_A {
        match self.bits {
            false => SGE_A::_0,
            true => SGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGE_A::_1
    }
}
#[doc = "NBYTES/CITER Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCE_A {
    #[doc = "0: No NBYTES/CITER configuration error"]
    _0 = 0,
}
impl From<NCE_A> for bool {
    #[inline(always)]
    fn from(variant: NCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NCE`"]
pub type NCE_R = crate::R<bool, NCE_A>;
impl NCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, NCE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(NCE_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCE_A::_0
    }
}
#[doc = "Destination Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOE_A {
    #[doc = "0: No destination offset configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    _1 = 1,
}
impl From<DOE_A> for bool {
    #[inline(always)]
    fn from(variant: DOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOE`"]
pub type DOE_R = crate::R<bool, DOE_A>;
impl DOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOE_A {
        match self.bits {
            false => DOE_A::_0,
            true => DOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOE_A::_1
    }
}
#[doc = "Destination Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAE_A {
    #[doc = "0: No destination address configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    _1 = 1,
}
impl From<DAE_A> for bool {
    #[inline(always)]
    fn from(variant: DAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAE`"]
pub type DAE_R = crate::R<bool, DAE_A>;
impl DAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAE_A {
        match self.bits {
            false => DAE_A::_0,
            true => DAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAE_A::_1
    }
}
#[doc = "Source Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOE_A {
    #[doc = "0: No source offset configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    _1 = 1,
}
impl From<SOE_A> for bool {
    #[inline(always)]
    fn from(variant: SOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOE`"]
pub type SOE_R = crate::R<bool, SOE_A>;
impl SOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOE_A {
        match self.bits {
            false => SOE_A::_0,
            true => SOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOE_A::_1
    }
}
#[doc = "Source Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAE_A {
    #[doc = "0: No source address configuration error."]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    _1 = 1,
}
impl From<SAE_A> for bool {
    #[inline(always)]
    fn from(variant: SAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAE`"]
pub type SAE_R = crate::R<bool, SAE_A>;
impl SAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAE_A {
        match self.bits {
            false => SAE_A::_0,
            true => SAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAE_A::_1
    }
}
#[doc = "Reader of field `ERRCHN`"]
pub type ERRCHN_R = crate::R<u8, u8>;
#[doc = "Channel Priority Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPE_A {
    #[doc = "0: No channel priority error"]
    _0 = 0,
}
impl From<CPE_A> for bool {
    #[inline(always)]
    fn from(variant: CPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPE`"]
pub type CPE_R = crate::R<bool, CPE_A>;
impl CPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CPE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CPE_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPE_A::_0
    }
}
#[doc = "Transfer Canceled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECX_A {
    #[doc = "0: No canceled transfers"]
    _0 = 0,
    #[doc = "1: The last recorded entry was a canceled transfer by the error cancel transfer input"]
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
#[doc = "VLD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLD_A {
    #[doc = "0: No ERR bits are set."]
    _0 = 0,
    #[doc = "1: At least one ERR bit is set indicating a valid error exists that has not been cleared."]
    _1 = 1,
}
impl From<VLD_A> for bool {
    #[inline(always)]
    fn from(variant: VLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VLD`"]
pub type VLD_R = crate::R<bool, VLD_A>;
impl VLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLD_A {
        match self.bits {
            false => VLD_A::_0,
            true => VLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VLD_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Destination Bus Error"]
    #[inline(always)]
    pub fn dbe(&self) -> DBE_R {
        DBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Source Bus Error"]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub fn sge(&self) -> SGE_R {
        SGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub fn nce(&self) -> NCE_R {
        NCE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Destination Offset Error"]
    #[inline(always)]
    pub fn doe(&self) -> DOE_R {
        DOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Address Error"]
    #[inline(always)]
    pub fn dae(&self) -> DAE_R {
        DAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source Offset Error"]
    #[inline(always)]
    pub fn soe(&self) -> SOE_R {
        SOE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Source Address Error"]
    #[inline(always)]
    pub fn sae(&self) -> SAE_R {
        SAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub fn errchn(&self) -> ERRCHN_R {
        ERRCHN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Channel Priority Error"]
    #[inline(always)]
    pub fn cpe(&self) -> CPE_R {
        CPE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transfer Canceled"]
    #[inline(always)]
    pub fn ecx(&self) -> ECX_R {
        ECX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - VLD"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
