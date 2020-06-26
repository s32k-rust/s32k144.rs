#[doc = "Reader of register PLASC"]
pub type R = crate::R<u16, super::PLASC>;
#[doc = "Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ASC_A {
    #[doc = "0: A bus slave connection to AXBS input port n is absent"]
    _0 = 0,
    #[doc = "1: A bus slave connection to AXBS input port n is present"]
    _1 = 1,
}
impl From<ASC_A> for u8 {
    #[inline(always)]
    fn from(variant: ASC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ASC`"]
pub type ASC_R = crate::R<u8, ASC_A>;
impl ASC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ASC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ASC_A::_0),
            1 => Val(ASC_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASC_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port."]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new((self.bits & 0xff) as u8)
    }
}
