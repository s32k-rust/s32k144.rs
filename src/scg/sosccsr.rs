#[doc = "Reader of register SOSCCSR"]
pub type R = crate::R<u32, super::SOSCCSR>;
#[doc = "Writer for register SOSCCSR"]
pub type W = crate::W<u32, super::SOSCCSR>;
#[doc = "Register SOSCCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SOSCCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System OSC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCEN_A {
    #[doc = "0: System OSC is disabled"]
    _0 = 0,
    #[doc = "1: System OSC is enabled"]
    _1 = 1,
}
impl From<SOSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOSCEN`"]
pub type SOSCEN_R = crate::R<bool, SOSCEN_A>;
impl SOSCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCEN_A {
        match self.bits {
            false => SOSCEN_A::_0,
            true => SOSCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSCEN_A::_1
    }
}
#[doc = "Write proxy for field `SOSCEN`"]
pub struct SOSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "System OSC is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCEN_A::_0)
    }
    #[doc = "System OSC is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "System OSC Clock Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCCM_A {
    #[doc = "0: System OSC Clock Monitor is disabled"]
    _0 = 0,
    #[doc = "1: System OSC Clock Monitor is enabled"]
    _1 = 1,
}
impl From<SOSCCM_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOSCCM`"]
pub type SOSCCM_R = crate::R<bool, SOSCCM_A>;
impl SOSCCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCCM_A {
        match self.bits {
            false => SOSCCM_A::_0,
            true => SOSCCM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSCCM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSCCM_A::_1
    }
}
#[doc = "Write proxy for field `SOSCCM`"]
pub struct SOSCCM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "System OSC Clock Monitor is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCCM_A::_0)
    }
    #[doc = "System OSC Clock Monitor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCCM_A::_1)
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
#[doc = "System OSC Clock Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCCMRE_A {
    #[doc = "0: Clock Monitor generates interrupt when error detected"]
    _0 = 0,
    #[doc = "1: Clock Monitor generates reset when error detected"]
    _1 = 1,
}
impl From<SOSCCMRE_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCCMRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOSCCMRE`"]
pub type SOSCCMRE_R = crate::R<bool, SOSCCMRE_A>;
impl SOSCCMRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCCMRE_A {
        match self.bits {
            false => SOSCCMRE_A::_0,
            true => SOSCCMRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSCCMRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSCCMRE_A::_1
    }
}
#[doc = "Write proxy for field `SOSCCMRE`"]
pub struct SOSCCMRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCCMRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCCMRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock Monitor generates interrupt when error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCCMRE_A::_0)
    }
    #[doc = "Clock Monitor generates reset when error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCCMRE_A::_1)
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
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
    #[doc = "0: This Control Status Register can be written."]
    _0 = 0,
    #[doc = "1: This Control Status Register cannot be written."]
    _1 = 1,
}
impl From<LK_A> for bool {
    #[inline(always)]
    fn from(variant: LK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LK`"]
pub type LK_R = crate::R<bool, LK_A>;
impl LK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LK_A {
        match self.bits {
            false => LK_A::_0,
            true => LK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LK_A::_1
    }
}
#[doc = "Write proxy for field `LK`"]
pub struct LK_W<'a> {
    w: &'a mut W,
}
impl<'a> LK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This Control Status Register can be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LK_A::_0)
    }
    #[doc = "This Control Status Register cannot be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "System OSC Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCVLD_A {
    #[doc = "0: System OSC is not enabled or clock is not valid"]
    _0 = 0,
    #[doc = "1: System OSC is enabled and output clock is valid"]
    _1 = 1,
}
impl From<SOSCVLD_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOSCVLD`"]
pub type SOSCVLD_R = crate::R<bool, SOSCVLD_A>;
impl SOSCVLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCVLD_A {
        match self.bits {
            false => SOSCVLD_A::_0,
            true => SOSCVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSCVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSCVLD_A::_1
    }
}
#[doc = "System OSC Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCSEL_A {
    #[doc = "0: System OSC is not the system clock source"]
    _0 = 0,
    #[doc = "1: System OSC is the system clock source"]
    _1 = 1,
}
impl From<SOSCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOSCSEL`"]
pub type SOSCSEL_R = crate::R<bool, SOSCSEL_A>;
impl SOSCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCSEL_A {
        match self.bits {
            false => SOSCSEL_A::_0,
            true => SOSCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSCSEL_A::_1
    }
}
#[doc = "System OSC Clock Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCERR_A {
    #[doc = "0: System OSC Clock Monitor is disabled or has not detected an error"]
    _0 = 0,
    #[doc = "1: System OSC Clock Monitor is enabled and detected an error"]
    _1 = 1,
}
impl From<SOSCERR_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOSCERR`"]
pub type SOSCERR_R = crate::R<bool, SOSCERR_A>;
impl SOSCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCERR_A {
        match self.bits {
            false => SOSCERR_A::_0,
            true => SOSCERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSCERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSCERR_A::_1
    }
}
#[doc = "Write proxy for field `SOSCERR`"]
pub struct SOSCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "System OSC Clock Monitor is disabled or has not detected an error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCERR_A::_0)
    }
    #[doc = "System OSC Clock Monitor is enabled and detected an error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCERR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - System OSC Enable"]
    #[inline(always)]
    pub fn soscen(&self) -> SOSCEN_R {
        SOSCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - System OSC Clock Monitor"]
    #[inline(always)]
    pub fn sosccm(&self) -> SOSCCM_R {
        SOSCCM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - System OSC Clock Monitor Reset Enable"]
    #[inline(always)]
    pub fn sosccmre(&self) -> SOSCCMRE_R {
        SOSCCMRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - System OSC Valid"]
    #[inline(always)]
    pub fn soscvld(&self) -> SOSCVLD_R {
        SOSCVLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - System OSC Selected"]
    #[inline(always)]
    pub fn soscsel(&self) -> SOSCSEL_R {
        SOSCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - System OSC Clock Error"]
    #[inline(always)]
    pub fn soscerr(&self) -> SOSCERR_R {
        SOSCERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System OSC Enable"]
    #[inline(always)]
    pub fn soscen(&mut self) -> SOSCEN_W {
        SOSCEN_W { w: self }
    }
    #[doc = "Bit 16 - System OSC Clock Monitor"]
    #[inline(always)]
    pub fn sosccm(&mut self) -> SOSCCM_W {
        SOSCCM_W { w: self }
    }
    #[doc = "Bit 17 - System OSC Clock Monitor Reset Enable"]
    #[inline(always)]
    pub fn sosccmre(&mut self) -> SOSCCMRE_W {
        SOSCCMRE_W { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline(always)]
    pub fn lk(&mut self) -> LK_W {
        LK_W { w: self }
    }
    #[doc = "Bit 26 - System OSC Clock Error"]
    #[inline(always)]
    pub fn soscerr(&mut self) -> SOSCERR_W {
        SOSCERR_W { w: self }
    }
}
