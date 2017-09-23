#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PACRD {
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
#[doc = "Possible values of the field `TP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP1R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP1R {
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
            TP1R::_0 => false,
            TP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP1R {
        match value {
            false => TP1R::_0,
            true => TP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP1R::_1
    }
}
#[doc = "Possible values of the field `WP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP1R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP1R {
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
            WP1R::_0 => false,
            WP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP1R {
        match value {
            false => WP1R::_0,
            true => WP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP1R::_1
    }
}
#[doc = "Possible values of the field `SP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP1R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP1R {
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
            SP1R::_0 => false,
            SP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP1R {
        match value {
            false => SP1R::_0,
            true => SP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP1R::_1
    }
}
#[doc = "Possible values of the field `TP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP0R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP0R {
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
            TP0R::_0 => false,
            TP0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP0R {
        match value {
            false => TP0R::_0,
            true => TP0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP0R::_1
    }
}
#[doc = "Possible values of the field `WP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP0R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP0R {
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
            WP0R::_0 => false,
            WP0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP0R {
        match value {
            false => WP0R::_0,
            true => WP0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP0R::_1
    }
}
#[doc = "Possible values of the field `SP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP0R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP0R {
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
            SP0R::_0 => false,
            SP0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP0R {
        match value {
            false => SP0R::_0,
            true => SP0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP0R::_1
    }
}
#[doc = "Values that can be written to the field `TP1`"]
pub enum TP1W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP1W::_0 => false,
            TP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP1W<'a> {
    w: &'a mut W,
}
impl<'a> _TP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP1W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP1W::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP1`"]
pub enum WP1W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP1W::_0 => false,
            WP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP1W<'a> {
    w: &'a mut W,
}
impl<'a> _WP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP1W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP1W::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SP1`"]
pub enum SP1W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP1W::_0 => false,
            SP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP1W<'a> {
    w: &'a mut W,
}
impl<'a> _SP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP1W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP1W::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TP0`"]
pub enum TP0W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP0W::_0 => false,
            TP0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP0W<'a> {
    w: &'a mut W,
}
impl<'a> _TP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP0W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP0W::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP0`"]
pub enum WP0W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP0W::_0 => false,
            WP0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP0W<'a> {
    w: &'a mut W,
}
impl<'a> _WP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP0W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP0W::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SP0`"]
pub enum SP0W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP0W::_0 => false,
            SP0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP0W<'a> {
    w: &'a mut W,
}
impl<'a> _SP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP0W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP0W::_1)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline]
    pub fn tp1(&self) -> TP1R {
        TP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline]
    pub fn wp1(&self) -> WP1R {
        WP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline]
    pub fn sp1(&self) -> SP1R {
        SP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline]
    pub fn tp0(&self) -> TP0R {
        TP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline]
    pub fn wp0(&self) -> WP0R {
        WP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline]
    pub fn sp0(&self) -> SP0R {
        SP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1140850688 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline]
    pub fn tp1(&mut self) -> _TP1W {
        _TP1W { w: self }
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline]
    pub fn wp1(&mut self) -> _WP1W {
        _WP1W { w: self }
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline]
    pub fn sp1(&mut self) -> _SP1W {
        _SP1W { w: self }
    }
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline]
    pub fn tp0(&mut self) -> _TP0W {
        _TP0W { w: self }
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline]
    pub fn wp0(&mut self) -> _WP0W {
        _WP0W { w: self }
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline]
    pub fn sp0(&mut self) -> _SP0W {
        _SP0W { w: self }
    }
}
