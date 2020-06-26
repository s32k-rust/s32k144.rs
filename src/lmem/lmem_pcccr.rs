#[doc = "Reader of register LMEM_PCCCR"]
pub type R = crate::R<u32, super::LMEM_PCCCR>;
#[doc = "Writer for register LMEM_PCCCR"]
pub type W = crate::W<u32, super::LMEM_PCCCR>;
#[doc = "Register LMEM_PCCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LMEM_PCCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCACHE_A {
    #[doc = "0: Cache disabled"]
    _0 = 0,
    #[doc = "1: Cache enabled"]
    _1 = 1,
}
impl From<ENCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: ENCACHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENCACHE`"]
pub type ENCACHE_R = crate::R<bool, ENCACHE_A>;
impl ENCACHE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCACHE_A {
        match self.bits {
            false => ENCACHE_A::_0,
            true => ENCACHE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENCACHE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENCACHE_A::_1
    }
}
#[doc = "Write proxy for field `ENCACHE`"]
pub struct ENCACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCACHE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENCACHE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cache disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENCACHE_A::_0)
    }
    #[doc = "Cache enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENCACHE_A::_1)
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
#[doc = "Reader of field `PCCR2`"]
pub type PCCR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCCR2`"]
pub struct PCCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCR2_W<'a> {
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
#[doc = "Reader of field `PCCR3`"]
pub type PCCR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCCR3`"]
pub struct PCCR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCR3_W<'a> {
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
#[doc = "Invalidate Way 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW0_A {
    #[doc = "0: No operation"]
    _0 = 0,
    #[doc = "1: When setting the GO bit, invalidate all lines in way 0."]
    _1 = 1,
}
impl From<INVW0_A> for bool {
    #[inline(always)]
    fn from(variant: INVW0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVW0`"]
pub type INVW0_R = crate::R<bool, INVW0_A>;
impl INVW0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVW0_A {
        match self.bits {
            false => INVW0_A::_0,
            true => INVW0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVW0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVW0_A::_1
    }
}
#[doc = "Write proxy for field `INVW0`"]
pub struct INVW0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVW0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVW0_A::_0)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVW0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Push Way 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHW0_A {
    #[doc = "0: No operation"]
    _0 = 0,
    #[doc = "1: When setting the GO bit, push all modified lines in way 0"]
    _1 = 1,
}
impl From<PUSHW0_A> for bool {
    #[inline(always)]
    fn from(variant: PUSHW0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PUSHW0`"]
pub type PUSHW0_R = crate::R<bool, PUSHW0_A>;
impl PUSHW0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUSHW0_A {
        match self.bits {
            false => PUSHW0_A::_0,
            true => PUSHW0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PUSHW0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PUSHW0_A::_1
    }
}
#[doc = "Write proxy for field `PUSHW0`"]
pub struct PUSHW0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSHW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUSHW0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUSHW0_A::_0)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUSHW0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Invalidate Way 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW1_A {
    #[doc = "0: No operation"]
    _0 = 0,
    #[doc = "1: When setting the GO bit, invalidate all lines in way 1"]
    _1 = 1,
}
impl From<INVW1_A> for bool {
    #[inline(always)]
    fn from(variant: INVW1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVW1`"]
pub type INVW1_R = crate::R<bool, INVW1_A>;
impl INVW1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVW1_A {
        match self.bits {
            false => INVW1_A::_0,
            true => INVW1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVW1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVW1_A::_1
    }
}
#[doc = "Write proxy for field `INVW1`"]
pub struct INVW1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVW1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVW1_A::_0)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVW1_A::_1)
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
#[doc = "Push Way 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHW1_A {
    #[doc = "0: No operation"]
    _0 = 0,
    #[doc = "1: When setting the GO bit, push all modified lines in way 1"]
    _1 = 1,
}
impl From<PUSHW1_A> for bool {
    #[inline(always)]
    fn from(variant: PUSHW1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PUSHW1`"]
pub type PUSHW1_R = crate::R<bool, PUSHW1_A>;
impl PUSHW1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUSHW1_A {
        match self.bits {
            false => PUSHW1_A::_0,
            true => PUSHW1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PUSHW1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PUSHW1_A::_1
    }
}
#[doc = "Write proxy for field `PUSHW1`"]
pub struct PUSHW1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSHW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUSHW1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUSHW1_A::_0)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUSHW1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Initiate Cache Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GO_A {
    #[doc = "0: Write: no effect. Read: no cache command active."]
    _0 = 0,
    #[doc = "1: Write: initiate command indicated by bits 27-24. Read: cache command active."]
    _1 = 1,
}
impl From<GO_A> for bool {
    #[inline(always)]
    fn from(variant: GO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GO`"]
pub type GO_R = crate::R<bool, GO_A>;
impl GO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GO_A {
        match self.bits {
            false => GO_A::_0,
            true => GO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GO_A::_1
    }
}
#[doc = "Write proxy for field `GO`"]
pub struct GO_W<'a> {
    w: &'a mut W,
}
impl<'a> GO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write: no effect. Read: no cache command active."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GO_A::_0)
    }
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn encache(&self) -> ENCACHE_R {
        ENCACHE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Forces all cacheable spaces to write through"]
    #[inline(always)]
    pub fn pccr2(&self) -> PCCR2_R {
        PCCR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Forces no allocation on cache misses (must also have PCCR2 asserted)"]
    #[inline(always)]
    pub fn pccr3(&self) -> PCCR3_R {
        PCCR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline(always)]
    pub fn invw0(&self) -> INVW0_R {
        INVW0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline(always)]
    pub fn pushw0(&self) -> PUSHW0_R {
        PUSHW0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline(always)]
    pub fn invw1(&self) -> INVW1_R {
        INVW1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline(always)]
    pub fn pushw1(&self) -> PUSHW1_R {
        PUSHW1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline(always)]
    pub fn go(&self) -> GO_R {
        GO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn encache(&mut self) -> ENCACHE_W {
        ENCACHE_W { w: self }
    }
    #[doc = "Bit 2 - Forces all cacheable spaces to write through"]
    #[inline(always)]
    pub fn pccr2(&mut self) -> PCCR2_W {
        PCCR2_W { w: self }
    }
    #[doc = "Bit 3 - Forces no allocation on cache misses (must also have PCCR2 asserted)"]
    #[inline(always)]
    pub fn pccr3(&mut self) -> PCCR3_W {
        PCCR3_W { w: self }
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline(always)]
    pub fn invw0(&mut self) -> INVW0_W {
        INVW0_W { w: self }
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline(always)]
    pub fn pushw0(&mut self) -> PUSHW0_W {
        PUSHW0_W { w: self }
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline(always)]
    pub fn invw1(&mut self) -> INVW1_W {
        INVW1_W { w: self }
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline(always)]
    pub fn pushw1(&mut self) -> PUSHW1_W {
        PUSHW1_W { w: self }
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline(always)]
    pub fn go(&mut self) -> GO_W {
        GO_W { w: self }
    }
}
