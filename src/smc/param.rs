#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Existence of HSRUN feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHSRUN_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EHSRUN_A> for bool {
    #[inline(always)]
    fn from(variant: EHSRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EHSRUN`"]
pub type EHSRUN_R = crate::R<bool, EHSRUN_A>;
impl EHSRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EHSRUN_A {
        match self.bits {
            false => EHSRUN_A::_0,
            true => EHSRUN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EHSRUN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EHSRUN_A::_1
    }
}
#[doc = "Existence of LLS feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELLS_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ELLS_A> for bool {
    #[inline(always)]
    fn from(variant: ELLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELLS`"]
pub type ELLS_R = crate::R<bool, ELLS_A>;
impl ELLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELLS_A {
        match self.bits {
            false => ELLS_A::_0,
            true => ELLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELLS_A::_1
    }
}
#[doc = "Existence of LLS2 feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELLS2_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ELLS2_A> for bool {
    #[inline(always)]
    fn from(variant: ELLS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELLS2`"]
pub type ELLS2_R = crate::R<bool, ELLS2_A>;
impl ELLS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELLS2_A {
        match self.bits {
            false => ELLS2_A::_0,
            true => ELLS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELLS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELLS2_A::_1
    }
}
#[doc = "Existence of VLLS0 feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVLLS0_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EVLLS0_A> for bool {
    #[inline(always)]
    fn from(variant: EVLLS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVLLS0`"]
pub type EVLLS0_R = crate::R<bool, EVLLS0_A>;
impl EVLLS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVLLS0_A {
        match self.bits {
            false => EVLLS0_A::_0,
            true => EVLLS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EVLLS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EVLLS0_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Existence of HSRUN feature"]
    #[inline(always)]
    pub fn ehsrun(&self) -> EHSRUN_R {
        EHSRUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Existence of LLS feature"]
    #[inline(always)]
    pub fn ells(&self) -> ELLS_R {
        ELLS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Existence of LLS2 feature"]
    #[inline(always)]
    pub fn ells2(&self) -> ELLS2_R {
        ELLS2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Existence of VLLS0 feature"]
    #[inline(always)]
    pub fn evlls0(&self) -> EVLLS0_R {
        EVLLS0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
