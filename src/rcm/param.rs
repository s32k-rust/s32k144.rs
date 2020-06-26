#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Existence of SRS\\[WAKEUP\\]
status indication feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWAKEUP_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EWAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: EWAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EWAKEUP`"]
pub type EWAKEUP_R = crate::R<bool, EWAKEUP_A>;
impl EWAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWAKEUP_A {
        match self.bits {
            false => EWAKEUP_A::_0,
            true => EWAKEUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWAKEUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWAKEUP_A::_1
    }
}
#[doc = "Existence of SRS\\[LVD\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELVD_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ELVD_A> for bool {
    #[inline(always)]
    fn from(variant: ELVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELVD`"]
pub type ELVD_R = crate::R<bool, ELVD_A>;
impl ELVD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELVD_A {
        match self.bits {
            false => ELVD_A::_0,
            true => ELVD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELVD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELVD_A::_1
    }
}
#[doc = "Existence of SRS\\[LOC\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOC_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ELOC_A> for bool {
    #[inline(always)]
    fn from(variant: ELOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELOC`"]
pub type ELOC_R = crate::R<bool, ELOC_A>;
impl ELOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELOC_A {
        match self.bits {
            false => ELOC_A::_0,
            true => ELOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELOC_A::_1
    }
}
#[doc = "Existence of SRS\\[LOL\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOL_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ELOL_A> for bool {
    #[inline(always)]
    fn from(variant: ELOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELOL`"]
pub type ELOL_R = crate::R<bool, ELOL_A>;
impl ELOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELOL_A {
        match self.bits {
            false => ELOL_A::_0,
            true => ELOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELOL_A::_1
    }
}
#[doc = "Existence of SRS\\[WDOG\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWDOG_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EWDOG_A> for bool {
    #[inline(always)]
    fn from(variant: EWDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EWDOG`"]
pub type EWDOG_R = crate::R<bool, EWDOG_A>;
impl EWDOG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWDOG_A {
        match self.bits {
            false => EWDOG_A::_0,
            true => EWDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWDOG_A::_1
    }
}
#[doc = "Existence of SRS\\[PIN\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EPIN_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN`"]
pub type EPIN_R = crate::R<bool, EPIN_A>;
impl EPIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN_A {
        match self.bits {
            false => EPIN_A::_0,
            true => EPIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPIN_A::_1
    }
}
#[doc = "Existence of SRS\\[POR\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOR_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EPOR_A> for bool {
    #[inline(always)]
    fn from(variant: EPOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOR`"]
pub type EPOR_R = crate::R<bool, EPOR_A>;
impl EPOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOR_A {
        match self.bits {
            false => EPOR_A::_0,
            true => EPOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPOR_A::_1
    }
}
#[doc = "Existence of SRS\\[JTAG\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EJTAG_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EJTAG_A> for bool {
    #[inline(always)]
    fn from(variant: EJTAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EJTAG`"]
pub type EJTAG_R = crate::R<bool, EJTAG_A>;
impl EJTAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EJTAG_A {
        match self.bits {
            false => EJTAG_A::_0,
            true => EJTAG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EJTAG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EJTAG_A::_1
    }
}
#[doc = "Existence of SRS\\[LOCKUP\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOCKUP_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ELOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: ELOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELOCKUP`"]
pub type ELOCKUP_R = crate::R<bool, ELOCKUP_A>;
impl ELOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELOCKUP_A {
        match self.bits {
            false => ELOCKUP_A::_0,
            true => ELOCKUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELOCKUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELOCKUP_A::_1
    }
}
#[doc = "Existence of SRS\\[SW\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESW_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ESW_A> for bool {
    #[inline(always)]
    fn from(variant: ESW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ESW`"]
pub type ESW_R = crate::R<bool, ESW_A>;
impl ESW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESW_A {
        match self.bits {
            false => ESW_A::_0,
            true => ESW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESW_A::_1
    }
}
#[doc = "Existence of SRS\\[MDM_AP\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMDM_AP_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<EMDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: EMDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMDM_AP`"]
pub type EMDM_AP_R = crate::R<bool, EMDM_AP_A>;
impl EMDM_AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMDM_AP_A {
        match self.bits {
            false => EMDM_AP_A::_0,
            true => EMDM_AP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMDM_AP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMDM_AP_A::_1
    }
}
#[doc = "Existence of SRS\\[SACKERR\\]
status indication feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESACKERR_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ESACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: ESACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ESACKERR`"]
pub type ESACKERR_R = crate::R<bool, ESACKERR_A>;
impl ESACKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESACKERR_A {
        match self.bits {
            false => ESACKERR_A::_0,
            true => ESACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESACKERR_A::_1
    }
}
#[doc = "Existence of SRS\\[TAMPER\\]
status indication feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETAMPER_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ETAMPER_A> for bool {
    #[inline(always)]
    fn from(variant: ETAMPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETAMPER`"]
pub type ETAMPER_R = crate::R<bool, ETAMPER_A>;
impl ETAMPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETAMPER_A {
        match self.bits {
            false => ETAMPER_A::_0,
            true => ETAMPER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETAMPER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETAMPER_A::_1
    }
}
#[doc = "Existence of SRS\\[CORE1\\]
status indication feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECORE1_A {
    #[doc = "0: The feature is not available."]
    _0 = 0,
    #[doc = "1: The feature is available."]
    _1 = 1,
}
impl From<ECORE1_A> for bool {
    #[inline(always)]
    fn from(variant: ECORE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECORE1`"]
pub type ECORE1_R = crate::R<bool, ECORE1_A>;
impl ECORE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECORE1_A {
        match self.bits {
            false => ECORE1_A::_0,
            true => ECORE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECORE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECORE1_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Existence of SRS\\[WAKEUP\\]
status indication feature"]
    #[inline(always)]
    pub fn ewakeup(&self) -> EWAKEUP_R {
        EWAKEUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Existence of SRS\\[LVD\\]
status indication feature"]
    #[inline(always)]
    pub fn elvd(&self) -> ELVD_R {
        ELVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Existence of SRS\\[LOC\\]
status indication feature"]
    #[inline(always)]
    pub fn eloc(&self) -> ELOC_R {
        ELOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Existence of SRS\\[LOL\\]
status indication feature"]
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Existence of SRS\\[WDOG\\]
status indication feature"]
    #[inline(always)]
    pub fn ewdog(&self) -> EWDOG_R {
        EWDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Existence of SRS\\[PIN\\]
status indication feature"]
    #[inline(always)]
    pub fn epin(&self) -> EPIN_R {
        EPIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Existence of SRS\\[POR\\]
status indication feature"]
    #[inline(always)]
    pub fn epor(&self) -> EPOR_R {
        EPOR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Existence of SRS\\[JTAG\\]
status indication feature"]
    #[inline(always)]
    pub fn ejtag(&self) -> EJTAG_R {
        EJTAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Existence of SRS\\[LOCKUP\\]
status indication feature"]
    #[inline(always)]
    pub fn elockup(&self) -> ELOCKUP_R {
        ELOCKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Existence of SRS\\[SW\\]
status indication feature"]
    #[inline(always)]
    pub fn esw(&self) -> ESW_R {
        ESW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Existence of SRS\\[MDM_AP\\]
status indication feature"]
    #[inline(always)]
    pub fn emdm_ap(&self) -> EMDM_AP_R {
        EMDM_AP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Existence of SRS\\[SACKERR\\]
status indication feature"]
    #[inline(always)]
    pub fn esackerr(&self) -> ESACKERR_R {
        ESACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Existence of SRS\\[TAMPER\\]
status indication feature"]
    #[inline(always)]
    pub fn etamper(&self) -> ETAMPER_R {
        ETAMPER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Existence of SRS\\[CORE1\\]
status indication feature"]
    #[inline(always)]
    pub fn ecore1(&self) -> ECORE1_R {
        ECORE1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
