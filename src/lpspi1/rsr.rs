#[doc = "Reader of register RSR"]
pub type R = crate::R<u32, super::RSR>;
#[doc = "Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "0: Subsequent data word received after LPSPI_PCS assertion."]
    _0 = 0,
    #[doc = "1: First data word received after LPSPI_PCS assertion."]
    _1 = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, SOF_A>;
impl SOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::_0,
            true => SOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOF_A::_1
    }
}
#[doc = "RX FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTY_A {
    #[doc = "0: RX FIFO is not empty."]
    _0 = 0,
    #[doc = "1: RX FIFO is empty."]
    _1 = 1,
}
impl From<RXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEMPTY`"]
pub type RXEMPTY_R = crate::R<bool, RXEMPTY_A>;
impl RXEMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPTY_A {
        match self.bits {
            false => RXEMPTY_A::_0,
            true => RXEMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEMPTY_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
