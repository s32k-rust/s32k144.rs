#[doc = "Reader of register WMB1_CS"]
pub type R = crate::R<u32, super::WMB1_CS>;
#[doc = "Reader of field `DLC`"]
pub type DLC_R = crate::R<u8, u8>;
#[doc = "Remote Transmission Request Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_A {
    #[doc = "0: Frame is data one (not remote)"]
    _0 = 0,
    #[doc = "1: Frame is a remote one"]
    _1 = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTR`"]
pub type RTR_R = crate::R<bool, RTR_A>;
impl RTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::_0,
            true => RTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTR_A::_1
    }
}
#[doc = "ID Extended Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_A {
    #[doc = "0: Frame format is standard"]
    _0 = 0,
    #[doc = "1: Frame format is extended"]
    _1 = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDE`"]
pub type IDE_R = crate::R<bool, IDE_A>;
impl IDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::_0,
            true => IDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDE_A::_1
    }
}
#[doc = "Reader of field `SRR`"]
pub type SRR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 16:19 - Length of Data in Bytes"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Remote Transmission Request Bit"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ID Extended Bit"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Substitute Remote Request"]
    #[inline(always)]
    pub fn srr(&self) -> SRR_R {
        SRR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
