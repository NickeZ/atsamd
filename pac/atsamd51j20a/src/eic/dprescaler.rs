#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DPRESCALER {
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
#[doc = "Possible values of the field `PRESCALER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER0R {
    #[doc = "EIC clock divided by 2"]
    DIV2,
    #[doc = "EIC clock divided by 4"]
    DIV4,
    #[doc = "EIC clock divided by 8"]
    DIV8,
    #[doc = "EIC clock divided by 16"]
    DIV16,
    #[doc = "EIC clock divided by 32"]
    DIV32,
    #[doc = "EIC clock divided by 64"]
    DIV64,
    #[doc = "EIC clock divided by 128"]
    DIV128,
    #[doc = "EIC clock divided by 256"]
    DIV256,
}
impl PRESCALER0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALER0R::DIV2 => 0,
            PRESCALER0R::DIV4 => 1,
            PRESCALER0R::DIV8 => 2,
            PRESCALER0R::DIV16 => 3,
            PRESCALER0R::DIV32 => 4,
            PRESCALER0R::DIV64 => 5,
            PRESCALER0R::DIV128 => 6,
            PRESCALER0R::DIV256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALER0R {
        match value {
            0 => PRESCALER0R::DIV2,
            1 => PRESCALER0R::DIV4,
            2 => PRESCALER0R::DIV8,
            3 => PRESCALER0R::DIV16,
            4 => PRESCALER0R::DIV32,
            5 => PRESCALER0R::DIV64,
            6 => PRESCALER0R::DIV128,
            7 => PRESCALER0R::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER0R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER0R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER0R::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER0R::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER0R::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER0R::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER0R::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER0R::DIV256
    }
}
#[doc = "Possible values of the field `STATES0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATES0R {
    #[doc = "3 low frequency samples"]
    LFREQ3,
    #[doc = "7 low frequency samples"]
    LFREQ7,
}
impl STATES0R {
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
            STATES0R::LFREQ3 => false,
            STATES0R::LFREQ7 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATES0R {
        match value {
            false => STATES0R::LFREQ3,
            true => STATES0R::LFREQ7,
        }
    }
    #[doc = "Checks if the value of the field is `LFREQ3`"]
    #[inline]
    pub fn is_lfreq3(&self) -> bool {
        *self == STATES0R::LFREQ3
    }
    #[doc = "Checks if the value of the field is `LFREQ7`"]
    #[inline]
    pub fn is_lfreq7(&self) -> bool {
        *self == STATES0R::LFREQ7
    }
}
#[doc = "Possible values of the field `PRESCALER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER1R {
    #[doc = "EIC clock divided by 2"]
    DIV2,
    #[doc = "EIC clock divided by 4"]
    DIV4,
    #[doc = "EIC clock divided by 8"]
    DIV8,
    #[doc = "EIC clock divided by 16"]
    DIV16,
    #[doc = "EIC clock divided by 32"]
    DIV32,
    #[doc = "EIC clock divided by 64"]
    DIV64,
    #[doc = "EIC clock divided by 128"]
    DIV128,
    #[doc = "EIC clock divided by 256"]
    DIV256,
}
impl PRESCALER1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALER1R::DIV2 => 0,
            PRESCALER1R::DIV4 => 1,
            PRESCALER1R::DIV8 => 2,
            PRESCALER1R::DIV16 => 3,
            PRESCALER1R::DIV32 => 4,
            PRESCALER1R::DIV64 => 5,
            PRESCALER1R::DIV128 => 6,
            PRESCALER1R::DIV256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALER1R {
        match value {
            0 => PRESCALER1R::DIV2,
            1 => PRESCALER1R::DIV4,
            2 => PRESCALER1R::DIV8,
            3 => PRESCALER1R::DIV16,
            4 => PRESCALER1R::DIV32,
            5 => PRESCALER1R::DIV64,
            6 => PRESCALER1R::DIV128,
            7 => PRESCALER1R::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER1R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER1R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER1R::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER1R::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER1R::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER1R::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER1R::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER1R::DIV256
    }
}
#[doc = "Possible values of the field `STATES1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATES1R {
    #[doc = "3 low frequency samples"]
    LFREQ3,
    #[doc = "7 low frequency samples"]
    LFREQ7,
}
impl STATES1R {
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
            STATES1R::LFREQ3 => false,
            STATES1R::LFREQ7 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATES1R {
        match value {
            false => STATES1R::LFREQ3,
            true => STATES1R::LFREQ7,
        }
    }
    #[doc = "Checks if the value of the field is `LFREQ3`"]
    #[inline]
    pub fn is_lfreq3(&self) -> bool {
        *self == STATES1R::LFREQ3
    }
    #[doc = "Checks if the value of the field is `LFREQ7`"]
    #[inline]
    pub fn is_lfreq7(&self) -> bool {
        *self == STATES1R::LFREQ7
    }
}
#[doc = "Possible values of the field `TICKON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKONR {
    #[doc = "Clocked by GCLK"]
    CLK_GCLK_EIC,
    #[doc = "Clocked by Low Frequency Clock"]
    CLK_LFREQ,
}
impl TICKONR {
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
            TICKONR::CLK_GCLK_EIC => false,
            TICKONR::CLK_LFREQ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TICKONR {
        match value {
            false => TICKONR::CLK_GCLK_EIC,
            true => TICKONR::CLK_LFREQ,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_GCLK_EIC`"]
    #[inline]
    pub fn is_clk_gclk_eic(&self) -> bool {
        *self == TICKONR::CLK_GCLK_EIC
    }
    #[doc = "Checks if the value of the field is `CLK_LFREQ`"]
    #[inline]
    pub fn is_clk_lfreq(&self) -> bool {
        *self == TICKONR::CLK_LFREQ
    }
}
#[doc = "Values that can be written to the field `PRESCALER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER0W {
    #[doc = "EIC clock divided by 2"]
    DIV2,
    #[doc = "EIC clock divided by 4"]
    DIV4,
    #[doc = "EIC clock divided by 8"]
    DIV8,
    #[doc = "EIC clock divided by 16"]
    DIV16,
    #[doc = "EIC clock divided by 32"]
    DIV32,
    #[doc = "EIC clock divided by 64"]
    DIV64,
    #[doc = "EIC clock divided by 128"]
    DIV128,
    #[doc = "EIC clock divided by 256"]
    DIV256,
}
impl PRESCALER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALER0W::DIV2 => 0,
            PRESCALER0W::DIV4 => 1,
            PRESCALER0W::DIV8 => 2,
            PRESCALER0W::DIV16 => 3,
            PRESCALER0W::DIV32 => 4,
            PRESCALER0W::DIV64 => 5,
            PRESCALER0W::DIV128 => 6,
            PRESCALER0W::DIV256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALER0W<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALER0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EIC clock divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER0W::DIV2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER0W::DIV4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER0W::DIV8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER0W::DIV16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER0W::DIV32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER0W::DIV64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER0W::DIV128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER0W::DIV256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STATES0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATES0W {
    #[doc = "3 low frequency samples"]
    LFREQ3,
    #[doc = "7 low frequency samples"]
    LFREQ7,
}
impl STATES0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STATES0W::LFREQ3 => false,
            STATES0W::LFREQ7 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATES0W<'a> {
    w: &'a mut W,
}
impl<'a> _STATES0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATES0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "3 low frequency samples"]
    #[inline]
    pub fn lfreq3(self) -> &'a mut W {
        self.variant(STATES0W::LFREQ3)
    }
    #[doc = "7 low frequency samples"]
    #[inline]
    pub fn lfreq7(self) -> &'a mut W {
        self.variant(STATES0W::LFREQ7)
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
#[doc = "Values that can be written to the field `PRESCALER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER1W {
    #[doc = "EIC clock divided by 2"]
    DIV2,
    #[doc = "EIC clock divided by 4"]
    DIV4,
    #[doc = "EIC clock divided by 8"]
    DIV8,
    #[doc = "EIC clock divided by 16"]
    DIV16,
    #[doc = "EIC clock divided by 32"]
    DIV32,
    #[doc = "EIC clock divided by 64"]
    DIV64,
    #[doc = "EIC clock divided by 128"]
    DIV128,
    #[doc = "EIC clock divided by 256"]
    DIV256,
}
impl PRESCALER1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALER1W::DIV2 => 0,
            PRESCALER1W::DIV4 => 1,
            PRESCALER1W::DIV8 => 2,
            PRESCALER1W::DIV16 => 3,
            PRESCALER1W::DIV32 => 4,
            PRESCALER1W::DIV64 => 5,
            PRESCALER1W::DIV128 => 6,
            PRESCALER1W::DIV256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALER1W<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALER1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALER1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EIC clock divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER1W::DIV2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER1W::DIV4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER1W::DIV8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER1W::DIV16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER1W::DIV32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER1W::DIV64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER1W::DIV128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER1W::DIV256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STATES1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATES1W {
    #[doc = "3 low frequency samples"]
    LFREQ3,
    #[doc = "7 low frequency samples"]
    LFREQ7,
}
impl STATES1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STATES1W::LFREQ3 => false,
            STATES1W::LFREQ7 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATES1W<'a> {
    w: &'a mut W,
}
impl<'a> _STATES1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATES1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "3 low frequency samples"]
    #[inline]
    pub fn lfreq3(self) -> &'a mut W {
        self.variant(STATES1W::LFREQ3)
    }
    #[doc = "7 low frequency samples"]
    #[inline]
    pub fn lfreq7(self) -> &'a mut W {
        self.variant(STATES1W::LFREQ7)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TICKON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKONW {
    #[doc = "Clocked by GCLK"]
    CLK_GCLK_EIC,
    #[doc = "Clocked by Low Frequency Clock"]
    CLK_LFREQ,
}
impl TICKONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TICKONW::CLK_GCLK_EIC => false,
            TICKONW::CLK_LFREQ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TICKONW<'a> {
    w: &'a mut W,
}
impl<'a> _TICKONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TICKONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clocked by GCLK"]
    #[inline]
    pub fn clk_gclk_eic(self) -> &'a mut W {
        self.variant(TICKONW::CLK_GCLK_EIC)
    }
    #[doc = "Clocked by Low Frequency Clock"]
    #[inline]
    pub fn clk_lfreq(self) -> &'a mut W {
        self.variant(TICKONW::CLK_LFREQ)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline]
    pub fn prescaler0(&self) -> PRESCALER0R {
        PRESCALER0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline]
    pub fn states0(&self) -> STATES0R {
        STATES0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline]
    pub fn prescaler1(&self) -> PRESCALER1R {
        PRESCALER1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline]
    pub fn states1(&self) -> STATES1R {
        STATES1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline]
    pub fn tickon(&self) -> TICKONR {
        TICKONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline]
    pub fn prescaler0(&mut self) -> _PRESCALER0W {
        _PRESCALER0W { w: self }
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline]
    pub fn states0(&mut self) -> _STATES0W {
        _STATES0W { w: self }
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline]
    pub fn prescaler1(&mut self) -> _PRESCALER1W {
        _PRESCALER1W { w: self }
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline]
    pub fn states1(&mut self) -> _STATES1W {
        _STATES1W { w: self }
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline]
    pub fn tickon(&mut self) -> _TICKONW {
        _TICKONW { w: self }
    }
}
