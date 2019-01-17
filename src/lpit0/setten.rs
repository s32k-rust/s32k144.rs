#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SETTEN {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SET_T_EN_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_T_EN_0R {
    #[doc = "No effect"]
    _0,
    #[doc = "Enables the Timer Channel 0"]
    _1,
}
impl SET_T_EN_0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SET_T_EN_0R::_0 => false,
            SET_T_EN_0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SET_T_EN_0R {
        match value {
            false => SET_T_EN_0R::_0,
            true => SET_T_EN_0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SET_T_EN_0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SET_T_EN_0R::_1
    }
}
#[doc = "Possible values of the field `SET_T_EN_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_T_EN_1R {
    #[doc = "No Effect"]
    _0,
    #[doc = "Enables the Timer Channel 1"]
    _1,
}
impl SET_T_EN_1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SET_T_EN_1R::_0 => false,
            SET_T_EN_1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SET_T_EN_1R {
        match value {
            false => SET_T_EN_1R::_0,
            true => SET_T_EN_1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SET_T_EN_1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SET_T_EN_1R::_1
    }
}
#[doc = "Possible values of the field `SET_T_EN_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_T_EN_2R {
    #[doc = "No Effect"]
    _0,
    #[doc = "Enables the Timer Channel 2"]
    _1,
}
impl SET_T_EN_2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SET_T_EN_2R::_0 => false,
            SET_T_EN_2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SET_T_EN_2R {
        match value {
            false => SET_T_EN_2R::_0,
            true => SET_T_EN_2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SET_T_EN_2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SET_T_EN_2R::_1
    }
}
#[doc = "Possible values of the field `SET_T_EN_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_T_EN_3R {
    #[doc = "No effect"]
    _0,
    #[doc = "Enables the Timer Channel 3"]
    _1,
}
impl SET_T_EN_3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SET_T_EN_3R::_0 => false,
            SET_T_EN_3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SET_T_EN_3R {
        match value {
            false => SET_T_EN_3R::_0,
            true => SET_T_EN_3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SET_T_EN_3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SET_T_EN_3R::_1
    }
}
#[doc = "Values that can be written to the field `SET_T_EN_0`"]
pub enum SET_T_EN_0W {
    #[doc = "No effect"]
    _0,
    #[doc = "Enables the Timer Channel 0"]
    _1,
}
impl SET_T_EN_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SET_T_EN_0W::_0 => false,
            SET_T_EN_0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SET_T_EN_0W<'a> {
    w: &'a mut W,
}
impl<'a> _SET_T_EN_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SET_T_EN_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SET_T_EN_0W::_0)
    }
    #[doc = "Enables the Timer Channel 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SET_T_EN_0W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SET_T_EN_1`"]
pub enum SET_T_EN_1W {
    #[doc = "No Effect"]
    _0,
    #[doc = "Enables the Timer Channel 1"]
    _1,
}
impl SET_T_EN_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SET_T_EN_1W::_0 => false,
            SET_T_EN_1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SET_T_EN_1W<'a> {
    w: &'a mut W,
}
impl<'a> _SET_T_EN_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SET_T_EN_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SET_T_EN_1W::_0)
    }
    #[doc = "Enables the Timer Channel 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SET_T_EN_1W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SET_T_EN_2`"]
pub enum SET_T_EN_2W {
    #[doc = "No Effect"]
    _0,
    #[doc = "Enables the Timer Channel 2"]
    _1,
}
impl SET_T_EN_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SET_T_EN_2W::_0 => false,
            SET_T_EN_2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SET_T_EN_2W<'a> {
    w: &'a mut W,
}
impl<'a> _SET_T_EN_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SET_T_EN_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SET_T_EN_2W::_0)
    }
    #[doc = "Enables the Timer Channel 2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SET_T_EN_2W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SET_T_EN_3`"]
pub enum SET_T_EN_3W {
    #[doc = "No effect"]
    _0,
    #[doc = "Enables the Timer Channel 3"]
    _1,
}
impl SET_T_EN_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SET_T_EN_3W::_0 => false,
            SET_T_EN_3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SET_T_EN_3W<'a> {
    w: &'a mut W,
}
impl<'a> _SET_T_EN_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SET_T_EN_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SET_T_EN_3W::_0)
    }
    #[doc = "Enables the Timer Channel 3"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SET_T_EN_3W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Set Timer 0 Enable"]
    #[inline]
    pub fn set_t_en_0(&self) -> SET_T_EN_0R {
        SET_T_EN_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Set Timer 1 Enable"]
    #[inline]
    pub fn set_t_en_1(&self) -> SET_T_EN_1R {
        SET_T_EN_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Set Timer 2 Enable"]
    #[inline]
    pub fn set_t_en_2(&self) -> SET_T_EN_2R {
        SET_T_EN_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Set Timer 3 Enable"]
    #[inline]
    pub fn set_t_en_3(&self) -> SET_T_EN_3R {
        SET_T_EN_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Set Timer 0 Enable"]
    #[inline]
    pub fn set_t_en_0(&mut self) -> _SET_T_EN_0W {
        _SET_T_EN_0W { w: self }
    }
    #[doc = "Bit 1 - Set Timer 1 Enable"]
    #[inline]
    pub fn set_t_en_1(&mut self) -> _SET_T_EN_1W {
        _SET_T_EN_1W { w: self }
    }
    #[doc = "Bit 2 - Set Timer 2 Enable"]
    #[inline]
    pub fn set_t_en_2(&mut self) -> _SET_T_EN_2W {
        _SET_T_EN_2W { w: self }
    }
    #[doc = "Bit 3 - Set Timer 3 Enable"]
    #[inline]
    pub fn set_t_en_3(&mut self) -> _SET_T_EN_3W {
        _SET_T_EN_3W { w: self }
    }
}
