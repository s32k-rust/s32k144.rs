#[doc = "Reader of register SETTEN"]
pub type R = crate::R<u32, super::SETTEN>;
#[doc = "Writer for register SETTEN"]
pub type W = crate::W<u32, super::SETTEN>;
#[doc = "Register SETTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SETTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set Timer 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_T_EN_0_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enables the Timer Channel 0"]
    _1 = 1,
}
impl From<SET_T_EN_0_A> for bool {
    #[inline(always)]
    fn from(variant: SET_T_EN_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SET_T_EN_0`"]
pub type SET_T_EN_0_R = crate::R<bool, SET_T_EN_0_A>;
impl SET_T_EN_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SET_T_EN_0_A {
        match self.bits {
            false => SET_T_EN_0_A::_0,
            true => SET_T_EN_0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SET_T_EN_0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SET_T_EN_0_A::_1
    }
}
#[doc = "Write proxy for field `SET_T_EN_0`"]
pub struct SET_T_EN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_T_EN_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_T_EN_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SET_T_EN_0_A::_0)
    }
    #[doc = "Enables the Timer Channel 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SET_T_EN_0_A::_1)
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
#[doc = "Set Timer 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_T_EN_1_A {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the Timer Channel 1"]
    _1 = 1,
}
impl From<SET_T_EN_1_A> for bool {
    #[inline(always)]
    fn from(variant: SET_T_EN_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SET_T_EN_1`"]
pub type SET_T_EN_1_R = crate::R<bool, SET_T_EN_1_A>;
impl SET_T_EN_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SET_T_EN_1_A {
        match self.bits {
            false => SET_T_EN_1_A::_0,
            true => SET_T_EN_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SET_T_EN_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SET_T_EN_1_A::_1
    }
}
#[doc = "Write proxy for field `SET_T_EN_1`"]
pub struct SET_T_EN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_T_EN_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_T_EN_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SET_T_EN_1_A::_0)
    }
    #[doc = "Enables the Timer Channel 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SET_T_EN_1_A::_1)
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
#[doc = "Set Timer 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_T_EN_2_A {
    #[doc = "0: No Effect"]
    _0 = 0,
    #[doc = "1: Enables the Timer Channel 2"]
    _1 = 1,
}
impl From<SET_T_EN_2_A> for bool {
    #[inline(always)]
    fn from(variant: SET_T_EN_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SET_T_EN_2`"]
pub type SET_T_EN_2_R = crate::R<bool, SET_T_EN_2_A>;
impl SET_T_EN_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SET_T_EN_2_A {
        match self.bits {
            false => SET_T_EN_2_A::_0,
            true => SET_T_EN_2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SET_T_EN_2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SET_T_EN_2_A::_1
    }
}
#[doc = "Write proxy for field `SET_T_EN_2`"]
pub struct SET_T_EN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_T_EN_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_T_EN_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SET_T_EN_2_A::_0)
    }
    #[doc = "Enables the Timer Channel 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SET_T_EN_2_A::_1)
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
#[doc = "Set Timer 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_T_EN_3_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enables the Timer Channel 3"]
    _1 = 1,
}
impl From<SET_T_EN_3_A> for bool {
    #[inline(always)]
    fn from(variant: SET_T_EN_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SET_T_EN_3`"]
pub type SET_T_EN_3_R = crate::R<bool, SET_T_EN_3_A>;
impl SET_T_EN_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SET_T_EN_3_A {
        match self.bits {
            false => SET_T_EN_3_A::_0,
            true => SET_T_EN_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SET_T_EN_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SET_T_EN_3_A::_1
    }
}
#[doc = "Write proxy for field `SET_T_EN_3`"]
pub struct SET_T_EN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_T_EN_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_T_EN_3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SET_T_EN_3_A::_0)
    }
    #[doc = "Enables the Timer Channel 3"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SET_T_EN_3_A::_1)
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
    #[doc = "Bit 0 - Set Timer 0 Enable"]
    #[inline(always)]
    pub fn set_t_en_0(&self) -> SET_T_EN_0_R {
        SET_T_EN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set Timer 1 Enable"]
    #[inline(always)]
    pub fn set_t_en_1(&self) -> SET_T_EN_1_R {
        SET_T_EN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set Timer 2 Enable"]
    #[inline(always)]
    pub fn set_t_en_2(&self) -> SET_T_EN_2_R {
        SET_T_EN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set Timer 3 Enable"]
    #[inline(always)]
    pub fn set_t_en_3(&self) -> SET_T_EN_3_R {
        SET_T_EN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set Timer 0 Enable"]
    #[inline(always)]
    pub fn set_t_en_0(&mut self) -> SET_T_EN_0_W {
        SET_T_EN_0_W { w: self }
    }
    #[doc = "Bit 1 - Set Timer 1 Enable"]
    #[inline(always)]
    pub fn set_t_en_1(&mut self) -> SET_T_EN_1_W {
        SET_T_EN_1_W { w: self }
    }
    #[doc = "Bit 2 - Set Timer 2 Enable"]
    #[inline(always)]
    pub fn set_t_en_2(&mut self) -> SET_T_EN_2_W {
        SET_T_EN_2_W { w: self }
    }
    #[doc = "Bit 3 - Set Timer 3 Enable"]
    #[inline(always)]
    pub fn set_t_en_3(&mut self) -> SET_T_EN_3_W {
        SET_T_EN_3_W { w: self }
    }
}
