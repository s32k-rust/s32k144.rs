#[doc = "Reader of register PLAMC"]
pub type R = crate::R<u16, super::PLAMC>;
#[doc = "Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMC_A {
    #[doc = "0: A bus master connection to AXBS input port n is absent"]
    _0 = 0,
    #[doc = "1: A bus master connection to AXBS input port n is present"]
    _1 = 1,
}
impl From<AMC_A> for u8 {
    #[inline(always)]
    fn from(variant: AMC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AMC`"]
pub type AMC_R = crate::R<u8, AMC_A>;
impl AMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AMC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AMC_A::_0),
            1 => Val(AMC_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMC_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
    #[inline(always)]
    pub fn amc(&self) -> AMC_R {
        AMC_R::new((self.bits & 0xff) as u8)
    }
}
