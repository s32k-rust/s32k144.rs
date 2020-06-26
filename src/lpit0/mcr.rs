#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Module Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_CEN_A {
    #[doc = "0: Peripheral clock to timers is disabled"]
    _0 = 0,
    #[doc = "1: Peripheral clock to timers is enabled"]
    _1 = 1,
}
impl From<M_CEN_A> for bool {
    #[inline(always)]
    fn from(variant: M_CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `M_CEN`"]
pub type M_CEN_R = crate::R<bool, M_CEN_A>;
impl M_CEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_CEN_A {
        match self.bits {
            false => M_CEN_A::_0,
            true => M_CEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M_CEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M_CEN_A::_1
    }
}
#[doc = "Write proxy for field `M_CEN`"]
pub struct M_CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> M_CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M_CEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral clock to timers is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M_CEN_A::_0)
    }
    #[doc = "Peripheral clock to timers is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M_CEN_A::_1)
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
#[doc = "Software Reset Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_A {
    #[doc = "0: Timer channels and registers are not reset"]
    _0 = 0,
    #[doc = "1: Timer channels and registers are reset"]
    _1 = 1,
}
impl From<SW_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SW_RST`"]
pub type SW_RST_R = crate::R<bool, SW_RST_A>;
impl SW_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_RST_A {
        match self.bits {
            false => SW_RST_A::_0,
            true => SW_RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SW_RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SW_RST_A::_1
    }
}
#[doc = "Write proxy for field `SW_RST`"]
pub struct SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer channels and registers are not reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SW_RST_A::_0)
    }
    #[doc = "Timer channels and registers are reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SW_RST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "DOZE Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZE_EN_A {
    #[doc = "0: Timer channels are stopped in DOZE mode"]
    _0 = 0,
    #[doc = "1: Timer channels continue to run in DOZE mode"]
    _1 = 1,
}
impl From<DOZE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOZE_EN`"]
pub type DOZE_EN_R = crate::R<bool, DOZE_EN_A>;
impl DOZE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZE_EN_A {
        match self.bits {
            false => DOZE_EN_A::_0,
            true => DOZE_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOZE_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOZE_EN_A::_1
    }
}
#[doc = "Write proxy for field `DOZE_EN`"]
pub struct DOZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer channels are stopped in DOZE mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZE_EN_A::_0)
    }
    #[doc = "Timer channels continue to run in DOZE mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZE_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Debug Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_EN_A {
    #[doc = "0: Timer channels are stopped in Debug mode"]
    _0 = 0,
    #[doc = "1: Timer channels continue to run in Debug mode"]
    _1 = 1,
}
impl From<DBG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_EN`"]
pub type DBG_EN_R = crate::R<bool, DBG_EN_A>;
impl DBG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_EN_A {
        match self.bits {
            false => DBG_EN_A::_0,
            true => DBG_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBG_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBG_EN_A::_1
    }
}
#[doc = "Write proxy for field `DBG_EN`"]
pub struct DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer channels are stopped in Debug mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBG_EN_A::_0)
    }
    #[doc = "Timer channels continue to run in Debug mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBG_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Module Clock Enable"]
    #[inline(always)]
    pub fn m_cen(&self) -> M_CEN_R {
        M_CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset Bit"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DOZE Mode Enable Bit"]
    #[inline(always)]
    pub fn doze_en(&self) -> DOZE_EN_R {
        DOZE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Enable Bit"]
    #[inline(always)]
    pub fn dbg_en(&self) -> DBG_EN_R {
        DBG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Clock Enable"]
    #[inline(always)]
    pub fn m_cen(&mut self) -> M_CEN_W {
        M_CEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset Bit"]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SW_RST_W {
        SW_RST_W { w: self }
    }
    #[doc = "Bit 2 - DOZE Mode Enable Bit"]
    #[inline(always)]
    pub fn doze_en(&mut self) -> DOZE_EN_W {
        DOZE_EN_W { w: self }
    }
    #[doc = "Bit 3 - Debug Enable Bit"]
    #[inline(always)]
    pub fn dbg_en(&mut self) -> DBG_EN_W {
        DBG_EN_W { w: self }
    }
}
