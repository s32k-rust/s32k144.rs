#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACRL {
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
#[doc = "Possible values of the field `TP7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP7R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP7R {
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
            TP7R::_0 => false,
            TP7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP7R {
        match value {
            false => TP7R::_0,
            true => TP7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP7R::_1
    }
}
#[doc = "Possible values of the field `WP7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP7R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP7R {
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
            WP7R::_0 => false,
            WP7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP7R {
        match value {
            false => WP7R::_0,
            true => WP7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP7R::_1
    }
}
#[doc = "Possible values of the field `SP7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP7R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP7R {
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
            SP7R::_0 => false,
            SP7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP7R {
        match value {
            false => SP7R::_0,
            true => SP7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP7R::_1
    }
}
#[doc = "Possible values of the field `TP6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP6R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP6R {
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
            TP6R::_0 => false,
            TP6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP6R {
        match value {
            false => TP6R::_0,
            true => TP6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP6R::_1
    }
}
#[doc = "Possible values of the field `WP6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP6R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP6R {
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
            WP6R::_0 => false,
            WP6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP6R {
        match value {
            false => WP6R::_0,
            true => WP6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP6R::_1
    }
}
#[doc = "Possible values of the field `SP6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP6R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP6R {
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
            SP6R::_0 => false,
            SP6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP6R {
        match value {
            false => SP6R::_0,
            true => SP6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP6R::_1
    }
}
#[doc = "Possible values of the field `TP5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP5R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP5R {
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
            TP5R::_0 => false,
            TP5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP5R {
        match value {
            false => TP5R::_0,
            true => TP5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP5R::_1
    }
}
#[doc = "Possible values of the field `WP5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP5R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP5R {
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
            WP5R::_0 => false,
            WP5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP5R {
        match value {
            false => WP5R::_0,
            true => WP5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP5R::_1
    }
}
#[doc = "Possible values of the field `SP5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP5R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP5R {
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
            SP5R::_0 => false,
            SP5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP5R {
        match value {
            false => SP5R::_0,
            true => SP5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP5R::_1
    }
}
#[doc = "Values that can be written to the field `TP7`"]
pub enum TP7W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP7W::_0 => false,
            TP7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP7W<'a> {
    w: &'a mut W,
}
impl<'a> _TP7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP7W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP7W::_1)
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
#[doc = "Values that can be written to the field `WP7`"]
pub enum WP7W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP7W::_0 => false,
            WP7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP7W<'a> {
    w: &'a mut W,
}
impl<'a> _WP7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP7W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP7W::_1)
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
#[doc = "Values that can be written to the field `SP7`"]
pub enum SP7W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP7W::_0 => false,
            SP7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP7W<'a> {
    w: &'a mut W,
}
impl<'a> _SP7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP7W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP7W::_1)
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
#[doc = "Values that can be written to the field `TP6`"]
pub enum TP6W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP6W::_0 => false,
            TP6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP6W<'a> {
    w: &'a mut W,
}
impl<'a> _TP6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP6W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP6W::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP6`"]
pub enum WP6W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP6W::_0 => false,
            WP6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP6W<'a> {
    w: &'a mut W,
}
impl<'a> _WP6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP6W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP6W::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SP6`"]
pub enum SP6W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP6W::_0 => false,
            SP6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP6W<'a> {
    w: &'a mut W,
}
impl<'a> _SP6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP6W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP6W::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TP5`"]
pub enum TP5W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP5W::_0 => false,
            TP5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP5W<'a> {
    w: &'a mut W,
}
impl<'a> _TP5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP5W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP5W::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP5`"]
pub enum WP5W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP5W::_0 => false,
            WP5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP5W<'a> {
    w: &'a mut W,
}
impl<'a> _WP5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP5W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP5W::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SP5`"]
pub enum SP5W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP5W::_0 => false,
            SP5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP5W<'a> {
    w: &'a mut W,
}
impl<'a> _SP5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP5W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP5W::_1)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - Trusted Protect"]
    #[inline]
    pub fn tp7(&self) -> TP7R {
        TP7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Write Protect"]
    #[inline]
    pub fn wp7(&self) -> WP7R {
        WP7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Supervisor Protect"]
    #[inline]
    pub fn sp7(&self) -> SP7R {
        SP7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline]
    pub fn tp6(&self) -> TP6R {
        TP6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline]
    pub fn wp6(&self) -> WP6R {
        WP6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline]
    pub fn sp6(&self) -> SP6R {
        SP6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline]
    pub fn tp5(&self) -> TP5R {
        TP5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline]
    pub fn wp5(&self) -> WP5R {
        WP5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline]
    pub fn sp5(&self) -> SP5R {
        SP5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1092 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Trusted Protect"]
    #[inline]
    pub fn tp7(&mut self) -> _TP7W {
        _TP7W { w: self }
    }
    #[doc = "Bit 1 - Write Protect"]
    #[inline]
    pub fn wp7(&mut self) -> _WP7W {
        _WP7W { w: self }
    }
    #[doc = "Bit 2 - Supervisor Protect"]
    #[inline]
    pub fn sp7(&mut self) -> _SP7W {
        _SP7W { w: self }
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline]
    pub fn tp6(&mut self) -> _TP6W {
        _TP6W { w: self }
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline]
    pub fn wp6(&mut self) -> _WP6W {
        _WP6W { w: self }
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline]
    pub fn sp6(&mut self) -> _SP6W {
        _SP6W { w: self }
    }
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline]
    pub fn tp5(&mut self) -> _TP5W {
        _TP5W { w: self }
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline]
    pub fn wp5(&mut self) -> _WP5W {
        _WP5W { w: self }
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline]
    pub fn sp5(&mut self) -> _SP5W {
        _SP5W { w: self }
    }
}
