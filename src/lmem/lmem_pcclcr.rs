#[doc = "Reader of register LMEM_PCCLCR"]
pub type R = crate::R<u32, super::LMEM_PCCLCR>;
#[doc = "Writer for register LMEM_PCCLCR"]
pub type W = crate::W<u32, super::LMEM_PCCLCR>;
#[doc = "Register LMEM_PCCLCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LMEM_PCCLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Initiate Cache Line Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LGO_A {
    #[doc = "0: Write: no effect. Read: no line command active."]
    _0 = 0,
    #[doc = "1: Write: initiate line command indicated by bits 27-24. Read: line command active."]
    _1 = 1,
}
impl From<LGO_A> for bool {
    #[inline(always)]
    fn from(variant: LGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LGO`"]
pub type LGO_R = crate::R<bool, LGO_A>;
impl LGO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LGO_A {
        match self.bits {
            false => LGO_A::_0,
            true => LGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LGO_A::_1
    }
}
#[doc = "Write proxy for field `LGO`"]
pub struct LGO_W<'a> {
    w: &'a mut W,
}
impl<'a> LGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LGO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LGO_A::_0)
    }
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LGO_A::_1)
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
#[doc = "Reader of field `CACHEADDR`"]
pub type CACHEADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CACHEADDR`"]
pub struct CACHEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 2)) | (((value as u32) & 0x0fff) << 2);
        self.w
    }
}
#[doc = "Way select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSEL_A {
    #[doc = "0: Way 0"]
    _0 = 0,
    #[doc = "1: Way 1"]
    _1 = 1,
}
impl From<WSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WSEL`"]
pub type WSEL_R = crate::R<bool, WSEL_A>;
impl WSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSEL_A {
        match self.bits {
            false => WSEL_A::_0,
            true => WSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WSEL_A::_1
    }
}
#[doc = "Write proxy for field `WSEL`"]
pub struct WSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Way 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSEL_A::_0)
    }
    #[doc = "Way 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Tag/Data Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDSEL_A {
    #[doc = "0: Data"]
    _0 = 0,
    #[doc = "1: Tag"]
    _1 = 1,
}
impl From<TDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDSEL`"]
pub type TDSEL_R = crate::R<bool, TDSEL_A>;
impl TDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDSEL_A {
        match self.bits {
            false => TDSEL_A::_0,
            true => TDSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDSEL_A::_1
    }
}
#[doc = "Write proxy for field `TDSEL`"]
pub struct TDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDSEL_A::_0)
    }
    #[doc = "Tag"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDSEL_A::_1)
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
#[doc = "Reader of field `LCIVB`"]
pub type LCIVB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCIVB`"]
pub struct LCIVB_W<'a> {
    w: &'a mut W,
}
impl<'a> LCIVB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LCIMB`"]
pub type LCIMB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCIMB`"]
pub struct LCIMB_W<'a> {
    w: &'a mut W,
}
impl<'a> LCIMB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `LCWAY`"]
pub type LCWAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCWAY`"]
pub struct LCWAY_W<'a> {
    w: &'a mut W,
}
impl<'a> LCWAY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Line Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCMD_A {
    #[doc = "0: Search and read or write"]
    _00 = 0,
    #[doc = "1: Invalidate"]
    _01 = 1,
    #[doc = "2: Push"]
    _10 = 2,
    #[doc = "3: Clear"]
    _11 = 3,
}
impl From<LCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: LCMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCMD`"]
pub type LCMD_R = crate::R<u8, LCMD_A>;
impl LCMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCMD_A {
        match self.bits {
            0 => LCMD_A::_00,
            1 => LCMD_A::_01,
            2 => LCMD_A::_10,
            3 => LCMD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LCMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LCMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LCMD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LCMD_A::_11
    }
}
#[doc = "Write proxy for field `LCMD`"]
pub struct LCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCMD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Search and read or write"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LCMD_A::_00)
    }
    #[doc = "Invalidate"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LCMD_A::_01)
    }
    #[doc = "Push"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LCMD_A::_10)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LCMD_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Line Address Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LADSEL_A {
    #[doc = "0: Cache address"]
    _0 = 0,
    #[doc = "1: Physical address"]
    _1 = 1,
}
impl From<LADSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LADSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LADSEL`"]
pub type LADSEL_R = crate::R<bool, LADSEL_A>;
impl LADSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LADSEL_A {
        match self.bits {
            false => LADSEL_A::_0,
            true => LADSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LADSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LADSEL_A::_1
    }
}
#[doc = "Write proxy for field `LADSEL`"]
pub struct LADSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LADSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LADSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cache address"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LADSEL_A::_0)
    }
    #[doc = "Physical address"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LADSEL_A::_1)
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
#[doc = "Line access type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LACC_A {
    #[doc = "0: Read"]
    _0 = 0,
    #[doc = "1: Write"]
    _1 = 1,
}
impl From<LACC_A> for bool {
    #[inline(always)]
    fn from(variant: LACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LACC`"]
pub type LACC_R = crate::R<bool, LACC_A>;
impl LACC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LACC_A {
        match self.bits {
            false => LACC_A::_0,
            true => LACC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LACC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LACC_A::_1
    }
}
#[doc = "Write proxy for field `LACC`"]
pub struct LACC_W<'a> {
    w: &'a mut W,
}
impl<'a> LACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LACC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LACC_A::_0)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LACC_A::_1)
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
impl R {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&self) -> LGO_R {
        LGO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:13 - Cache address"]
    #[inline(always)]
    pub fn cacheaddr(&self) -> CACHEADDR_R {
        CACHEADDR_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Way select"]
    #[inline(always)]
    pub fn wsel(&self) -> WSEL_R {
        WSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Tag/Data Select"]
    #[inline(always)]
    pub fn tdsel(&self) -> TDSEL_R {
        TDSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline(always)]
    pub fn lcivb(&self) -> LCIVB_R {
        LCIVB_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline(always)]
    pub fn lcimb(&self) -> LCIMB_R {
        LCIMB_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline(always)]
    pub fn lcway(&self) -> LCWAY_R {
        LCWAY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline(always)]
    pub fn lcmd(&self) -> LCMD_R {
        LCMD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline(always)]
    pub fn ladsel(&self) -> LADSEL_R {
        LADSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Line access type"]
    #[inline(always)]
    pub fn lacc(&self) -> LACC_R {
        LACC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&mut self) -> LGO_W {
        LGO_W { w: self }
    }
    #[doc = "Bits 2:13 - Cache address"]
    #[inline(always)]
    pub fn cacheaddr(&mut self) -> CACHEADDR_W {
        CACHEADDR_W { w: self }
    }
    #[doc = "Bit 14 - Way select"]
    #[inline(always)]
    pub fn wsel(&mut self) -> WSEL_W {
        WSEL_W { w: self }
    }
    #[doc = "Bit 16 - Tag/Data Select"]
    #[inline(always)]
    pub fn tdsel(&mut self) -> TDSEL_W {
        TDSEL_W { w: self }
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline(always)]
    pub fn lcivb(&mut self) -> LCIVB_W {
        LCIVB_W { w: self }
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline(always)]
    pub fn lcimb(&mut self) -> LCIMB_W {
        LCIMB_W { w: self }
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline(always)]
    pub fn lcway(&mut self) -> LCWAY_W {
        LCWAY_W { w: self }
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline(always)]
    pub fn lcmd(&mut self) -> LCMD_W {
        LCMD_W { w: self }
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline(always)]
    pub fn ladsel(&mut self) -> LADSEL_W {
        LADSEL_W { w: self }
    }
    #[doc = "Bit 27 - Line access type"]
    #[inline(always)]
    pub fn lacc(&mut self) -> LACC_W {
        LACC_W { w: self }
    }
}
