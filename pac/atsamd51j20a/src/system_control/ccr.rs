#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = r" Value of the field"]
pub struct NONBASETHRDENAR {
    bits: bool,
}
impl NONBASETHRDENAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct USERSETMPENDR {
    bits: bool,
}
impl USERSETMPENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `UNALIGN_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRPR {
    #[doc = "Do not trap unaligned halfword and word accesses"]
    VALUE_0,
    #[doc = "Trap unaligned halfword and word accesses"]
    VALUE_1,
}
impl UNALIGN_TRPR {
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
            UNALIGN_TRPR::VALUE_0 => false,
            UNALIGN_TRPR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNALIGN_TRPR {
        match value {
            false => UNALIGN_TRPR::VALUE_0,
            true => UNALIGN_TRPR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == UNALIGN_TRPR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == UNALIGN_TRPR::VALUE_1
    }
}
#[doc = r" Value of the field"]
pub struct DIV_0_TRPR {
    bits: bool,
}
impl DIV_0_TRPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct BFHFNMIGNR {
    bits: bool,
}
impl BFHFNMIGNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `STKALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGNR {
    #[doc = "4-byte aligned"]
    VALUE_0,
    #[doc = "8-byte aligned"]
    VALUE_1,
}
impl STKALIGNR {
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
            STKALIGNR::VALUE_0 => false,
            STKALIGNR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STKALIGNR {
        match value {
            false => STKALIGNR::VALUE_0,
            true => STKALIGNR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == STKALIGNR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == STKALIGNR::VALUE_1
    }
}
#[doc = r" Proxy"]
pub struct _NONBASETHRDENAW<'a> {
    w: &'a mut W,
}
impl<'a> _NONBASETHRDENAW<'a> {
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
#[doc = r" Proxy"]
pub struct _USERSETMPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USERSETMPENDW<'a> {
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
#[doc = "Values that can be written to the field `UNALIGN_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRPW {
    #[doc = "Do not trap unaligned halfword and word accesses"]
    VALUE_0,
    #[doc = "Trap unaligned halfword and word accesses"]
    VALUE_1,
}
impl UNALIGN_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGN_TRPW::VALUE_0 => false,
            UNALIGN_TRPW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNALIGN_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGN_TRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNALIGN_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not trap unaligned halfword and word accesses"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::VALUE_0)
    }
    #[doc = "Trap unaligned halfword and word accesses"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::VALUE_1)
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
#[doc = r" Proxy"]
pub struct _DIV_0_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _DIV_0_TRPW<'a> {
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
#[doc = r" Proxy"]
pub struct _BFHFNMIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _BFHFNMIGNW<'a> {
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
#[doc = "Values that can be written to the field `STKALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGNW {
    #[doc = "4-byte aligned"]
    VALUE_0,
    #[doc = "8-byte aligned"]
    VALUE_1,
}
impl STKALIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STKALIGNW::VALUE_0 => false,
            STKALIGNW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STKALIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _STKALIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STKALIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4-byte aligned"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(STKALIGNW::VALUE_0)
    }
    #[doc = "8-byte aligned"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(STKALIGNW::VALUE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Indicates how processor enters Thread mode"]
    #[inline]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NONBASETHRDENAR { bits }
    }
    #[doc = "Bit 1 - Enables unprivileged software access to STIR register"]
    #[inline]
    pub fn usersetmpend(&self) -> USERSETMPENDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USERSETMPENDR { bits }
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline]
    pub fn unalign_trp(&self) -> UNALIGN_TRPR {
        UNALIGN_TRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables divide by 0 trap"]
    #[inline]
    pub fn div_0_trp(&self) -> DIV_0_TRPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIV_0_TRPR { bits }
    }
    #[doc = "Bit 8 - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
    #[inline]
    pub fn bfhfnmign(&self) -> BFHFNMIGNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BFHFNMIGNR { bits }
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline]
    pub fn stkalign(&self) -> STKALIGNR {
        STKALIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Indicates how processor enters Thread mode"]
    #[inline]
    pub fn nonbasethrdena(&mut self) -> _NONBASETHRDENAW {
        _NONBASETHRDENAW { w: self }
    }
    #[doc = "Bit 1 - Enables unprivileged software access to STIR register"]
    #[inline]
    pub fn usersetmpend(&mut self) -> _USERSETMPENDW {
        _USERSETMPENDW { w: self }
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline]
    pub fn unalign_trp(&mut self) -> _UNALIGN_TRPW {
        _UNALIGN_TRPW { w: self }
    }
    #[doc = "Bit 4 - Enables divide by 0 trap"]
    #[inline]
    pub fn div_0_trp(&mut self) -> _DIV_0_TRPW {
        _DIV_0_TRPW { w: self }
    }
    #[doc = "Bit 8 - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
    #[inline]
    pub fn bfhfnmign(&mut self) -> _BFHFNMIGNW {
        _BFHFNMIGNW { w: self }
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline]
    pub fn stkalign(&mut self) -> _STKALIGNW {
        _STKALIGNW { w: self }
    }
}
