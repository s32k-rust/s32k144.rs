#[doc = "Reader of register SASR"]
pub type R = crate::R<u32, super::SASR>;
#[doc = "Reader of field `RADDR`"]
pub type RADDR_R = crate::R<u16, u16>;
#[doc = "Address Not Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANV_A {
    #[doc = "0: RADDR is valid."]
    _0 = 0,
    #[doc = "1: RADDR is not valid."]
    _1 = 1,
}
impl From<ANV_A> for bool {
    #[inline(always)]
    fn from(variant: ANV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANV`"]
pub type ANV_R = crate::R<bool, ANV_A>;
impl ANV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANV_A {
        match self.bits {
            false => ANV_A::_0,
            true => ANV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANV_A::_1
    }
}
impl R {
    #[doc = "Bits 0:10 - Received Address"]
    #[inline(always)]
    pub fn raddr(&self) -> RADDR_R {
        RADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 14 - Address Not Valid"]
    #[inline(always)]
    pub fn anv(&self) -> ANV_R {
        ANV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
