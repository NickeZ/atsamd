#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XOSCCTRL {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
pub struct XTALENR {
    bits: bool,
}
impl XTALENR {
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
pub struct RUNSTDBYR {
    bits: bool,
}
impl RUNSTDBYR {
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
pub struct ONDEMANDR {
    bits: bool,
}
impl ONDEMANDR {
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
pub struct LOWBUFGAINR {
    bits: bool,
}
impl LOWBUFGAINR {
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
pub struct IPTATR {
    bits: u8,
}
impl IPTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IMULTR {
    bits: u8,
}
impl IMULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENALCR {
    bits: bool,
}
impl ENALCR {
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
pub struct CFDENR {
    bits: bool,
}
impl CFDENR {
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
pub struct SWBENR {
    bits: bool,
}
impl SWBENR {
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
#[doc = "Possible values of the field `STARTUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPR {
    #[doc = "31 us"]
    CYCLE1,
    #[doc = "61 us"]
    CYCLE2,
    #[doc = "122 us"]
    CYCLE4,
    #[doc = "244 us"]
    CYCLE8,
    #[doc = "488 us"]
    CYCLE16,
    #[doc = "977 us"]
    CYCLE32,
    #[doc = "1953 us"]
    CYCLE64,
    #[doc = "3906 us"]
    CYCLE128,
    #[doc = "7813 us"]
    CYCLE256,
    #[doc = "15625 us"]
    CYCLE512,
    #[doc = "31250 us"]
    CYCLE1024,
    #[doc = "62500 us"]
    CYCLE2048,
    #[doc = "125000 us"]
    CYCLE4096,
    #[doc = "250000 us"]
    CYCLE8192,
    #[doc = "500000 us"]
    CYCLE16384,
    #[doc = "1000000 us"]
    CYCLE32768,
}
impl STARTUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTUPR::CYCLE1 => 0,
            STARTUPR::CYCLE2 => 1,
            STARTUPR::CYCLE4 => 2,
            STARTUPR::CYCLE8 => 3,
            STARTUPR::CYCLE16 => 4,
            STARTUPR::CYCLE32 => 5,
            STARTUPR::CYCLE64 => 6,
            STARTUPR::CYCLE128 => 7,
            STARTUPR::CYCLE256 => 8,
            STARTUPR::CYCLE512 => 9,
            STARTUPR::CYCLE1024 => 10,
            STARTUPR::CYCLE2048 => 11,
            STARTUPR::CYCLE4096 => 12,
            STARTUPR::CYCLE8192 => 13,
            STARTUPR::CYCLE16384 => 14,
            STARTUPR::CYCLE32768 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTUPR {
        match value {
            0 => STARTUPR::CYCLE1,
            1 => STARTUPR::CYCLE2,
            2 => STARTUPR::CYCLE4,
            3 => STARTUPR::CYCLE8,
            4 => STARTUPR::CYCLE16,
            5 => STARTUPR::CYCLE32,
            6 => STARTUPR::CYCLE64,
            7 => STARTUPR::CYCLE128,
            8 => STARTUPR::CYCLE256,
            9 => STARTUPR::CYCLE512,
            10 => STARTUPR::CYCLE1024,
            11 => STARTUPR::CYCLE2048,
            12 => STARTUPR::CYCLE4096,
            13 => STARTUPR::CYCLE8192,
            14 => STARTUPR::CYCLE16384,
            15 => STARTUPR::CYCLE32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE1`"]
    #[inline]
    pub fn is_cycle1(&self) -> bool {
        *self == STARTUPR::CYCLE1
    }
    #[doc = "Checks if the value of the field is `CYCLE2`"]
    #[inline]
    pub fn is_cycle2(&self) -> bool {
        *self == STARTUPR::CYCLE2
    }
    #[doc = "Checks if the value of the field is `CYCLE4`"]
    #[inline]
    pub fn is_cycle4(&self) -> bool {
        *self == STARTUPR::CYCLE4
    }
    #[doc = "Checks if the value of the field is `CYCLE8`"]
    #[inline]
    pub fn is_cycle8(&self) -> bool {
        *self == STARTUPR::CYCLE8
    }
    #[doc = "Checks if the value of the field is `CYCLE16`"]
    #[inline]
    pub fn is_cycle16(&self) -> bool {
        *self == STARTUPR::CYCLE16
    }
    #[doc = "Checks if the value of the field is `CYCLE32`"]
    #[inline]
    pub fn is_cycle32(&self) -> bool {
        *self == STARTUPR::CYCLE32
    }
    #[doc = "Checks if the value of the field is `CYCLE64`"]
    #[inline]
    pub fn is_cycle64(&self) -> bool {
        *self == STARTUPR::CYCLE64
    }
    #[doc = "Checks if the value of the field is `CYCLE128`"]
    #[inline]
    pub fn is_cycle128(&self) -> bool {
        *self == STARTUPR::CYCLE128
    }
    #[doc = "Checks if the value of the field is `CYCLE256`"]
    #[inline]
    pub fn is_cycle256(&self) -> bool {
        *self == STARTUPR::CYCLE256
    }
    #[doc = "Checks if the value of the field is `CYCLE512`"]
    #[inline]
    pub fn is_cycle512(&self) -> bool {
        *self == STARTUPR::CYCLE512
    }
    #[doc = "Checks if the value of the field is `CYCLE1024`"]
    #[inline]
    pub fn is_cycle1024(&self) -> bool {
        *self == STARTUPR::CYCLE1024
    }
    #[doc = "Checks if the value of the field is `CYCLE2048`"]
    #[inline]
    pub fn is_cycle2048(&self) -> bool {
        *self == STARTUPR::CYCLE2048
    }
    #[doc = "Checks if the value of the field is `CYCLE4096`"]
    #[inline]
    pub fn is_cycle4096(&self) -> bool {
        *self == STARTUPR::CYCLE4096
    }
    #[doc = "Checks if the value of the field is `CYCLE8192`"]
    #[inline]
    pub fn is_cycle8192(&self) -> bool {
        *self == STARTUPR::CYCLE8192
    }
    #[doc = "Checks if the value of the field is `CYCLE16384`"]
    #[inline]
    pub fn is_cycle16384(&self) -> bool {
        *self == STARTUPR::CYCLE16384
    }
    #[doc = "Checks if the value of the field is `CYCLE32768`"]
    #[inline]
    pub fn is_cycle32768(&self) -> bool {
        *self == STARTUPR::CYCLE32768
    }
}
#[doc = "Possible values of the field `CFDPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFDPRESCR {
    #[doc = "48 MHz"]
    DIV1,
    #[doc = "24 MHz"]
    DIV2,
    #[doc = "12 MHz"]
    DIV4,
    #[doc = "6 MHz"]
    DIV8,
    #[doc = "3 MHz"]
    DIV16,
    #[doc = "1.5 MHz"]
    DIV32,
    #[doc = "0.75 MHz"]
    DIV64,
    #[doc = "0.3125 MHz"]
    DIV128,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CFDPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFDPRESCR::DIV1 => 0,
            CFDPRESCR::DIV2 => 1,
            CFDPRESCR::DIV4 => 2,
            CFDPRESCR::DIV8 => 3,
            CFDPRESCR::DIV16 => 4,
            CFDPRESCR::DIV32 => 5,
            CFDPRESCR::DIV64 => 6,
            CFDPRESCR::DIV128 => 7,
            CFDPRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFDPRESCR {
        match value {
            0 => CFDPRESCR::DIV1,
            1 => CFDPRESCR::DIV2,
            2 => CFDPRESCR::DIV4,
            3 => CFDPRESCR::DIV8,
            4 => CFDPRESCR::DIV16,
            5 => CFDPRESCR::DIV32,
            6 => CFDPRESCR::DIV64,
            7 => CFDPRESCR::DIV128,
            i => CFDPRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == CFDPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == CFDPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == CFDPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == CFDPRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == CFDPRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == CFDPRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == CFDPRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == CFDPRESCR::DIV128
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
#[doc = r" Proxy"]
pub struct _XTALENW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALENW<'a> {
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
#[doc = r" Proxy"]
pub struct _RUNSTDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNSTDBYW<'a> {
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
#[doc = r" Proxy"]
pub struct _ONDEMANDW<'a> {
    w: &'a mut W,
}
impl<'a> _ONDEMANDW<'a> {
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
#[doc = r" Proxy"]
pub struct _LOWBUFGAINW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWBUFGAINW<'a> {
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
#[doc = r" Proxy"]
pub struct _IPTATW<'a> {
    w: &'a mut W,
}
impl<'a> _IPTATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IMULTW<'a> {
    w: &'a mut W,
}
impl<'a> _IMULTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENALCW<'a> {
    w: &'a mut W,
}
impl<'a> _ENALCW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CFDENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SWBENW<'a> {
    w: &'a mut W,
}
impl<'a> _SWBENW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STARTUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPW {
    #[doc = "31 us"]
    CYCLE1,
    #[doc = "61 us"]
    CYCLE2,
    #[doc = "122 us"]
    CYCLE4,
    #[doc = "244 us"]
    CYCLE8,
    #[doc = "488 us"]
    CYCLE16,
    #[doc = "977 us"]
    CYCLE32,
    #[doc = "1953 us"]
    CYCLE64,
    #[doc = "3906 us"]
    CYCLE128,
    #[doc = "7813 us"]
    CYCLE256,
    #[doc = "15625 us"]
    CYCLE512,
    #[doc = "31250 us"]
    CYCLE1024,
    #[doc = "62500 us"]
    CYCLE2048,
    #[doc = "125000 us"]
    CYCLE4096,
    #[doc = "250000 us"]
    CYCLE8192,
    #[doc = "500000 us"]
    CYCLE16384,
    #[doc = "1000000 us"]
    CYCLE32768,
}
impl STARTUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTUPW::CYCLE1 => 0,
            STARTUPW::CYCLE2 => 1,
            STARTUPW::CYCLE4 => 2,
            STARTUPW::CYCLE8 => 3,
            STARTUPW::CYCLE16 => 4,
            STARTUPW::CYCLE32 => 5,
            STARTUPW::CYCLE64 => 6,
            STARTUPW::CYCLE128 => 7,
            STARTUPW::CYCLE256 => 8,
            STARTUPW::CYCLE512 => 9,
            STARTUPW::CYCLE1024 => 10,
            STARTUPW::CYCLE2048 => 11,
            STARTUPW::CYCLE4096 => 12,
            STARTUPW::CYCLE8192 => 13,
            STARTUPW::CYCLE16384 => 14,
            STARTUPW::CYCLE32768 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTUPW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTUPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "31 us"]
    #[inline]
    pub fn cycle1(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE1)
    }
    #[doc = "61 us"]
    #[inline]
    pub fn cycle2(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE2)
    }
    #[doc = "122 us"]
    #[inline]
    pub fn cycle4(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE4)
    }
    #[doc = "244 us"]
    #[inline]
    pub fn cycle8(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE8)
    }
    #[doc = "488 us"]
    #[inline]
    pub fn cycle16(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE16)
    }
    #[doc = "977 us"]
    #[inline]
    pub fn cycle32(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE32)
    }
    #[doc = "1953 us"]
    #[inline]
    pub fn cycle64(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE64)
    }
    #[doc = "3906 us"]
    #[inline]
    pub fn cycle128(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE128)
    }
    #[doc = "7813 us"]
    #[inline]
    pub fn cycle256(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE256)
    }
    #[doc = "15625 us"]
    #[inline]
    pub fn cycle512(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE512)
    }
    #[doc = "31250 us"]
    #[inline]
    pub fn cycle1024(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE1024)
    }
    #[doc = "62500 us"]
    #[inline]
    pub fn cycle2048(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE2048)
    }
    #[doc = "125000 us"]
    #[inline]
    pub fn cycle4096(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE4096)
    }
    #[doc = "250000 us"]
    #[inline]
    pub fn cycle8192(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE8192)
    }
    #[doc = "500000 us"]
    #[inline]
    pub fn cycle16384(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE16384)
    }
    #[doc = "1000000 us"]
    #[inline]
    pub fn cycle32768(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE32768)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFDPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFDPRESCW {
    #[doc = "48 MHz"]
    DIV1,
    #[doc = "24 MHz"]
    DIV2,
    #[doc = "12 MHz"]
    DIV4,
    #[doc = "6 MHz"]
    DIV8,
    #[doc = "3 MHz"]
    DIV16,
    #[doc = "1.5 MHz"]
    DIV32,
    #[doc = "0.75 MHz"]
    DIV64,
    #[doc = "0.3125 MHz"]
    DIV128,
}
impl CFDPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFDPRESCW::DIV1 => 0,
            CFDPRESCW::DIV2 => 1,
            CFDPRESCW::DIV4 => 2,
            CFDPRESCW::DIV8 => 3,
            CFDPRESCW::DIV16 => 4,
            CFDPRESCW::DIV32 => 5,
            CFDPRESCW::DIV64 => 6,
            CFDPRESCW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFDPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _CFDPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFDPRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "48 MHz"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(CFDPRESCW::DIV1)
    }
    #[doc = "24 MHz"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(CFDPRESCW::DIV2)
    }
    #[doc = "12 MHz"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(CFDPRESCW::DIV4)
    }
    #[doc = "6 MHz"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(CFDPRESCW::DIV8)
    }
    #[doc = "3 MHz"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(CFDPRESCW::DIV16)
    }
    #[doc = "1.5 MHz"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(CFDPRESCW::DIV32)
    }
    #[doc = "0.75 MHz"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(CFDPRESCW::DIV64)
    }
    #[doc = "0.3125 MHz"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(CFDPRESCW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline]
    pub fn xtalen(&self) -> XTALENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XTALENR { bits }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline]
    pub fn ondemand(&self) -> ONDEMANDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ONDEMANDR { bits }
    }
    #[doc = "Bit 8 - Low Buffer Gain Enable"]
    #[inline]
    pub fn lowbufgain(&self) -> LOWBUFGAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOWBUFGAINR { bits }
    }
    #[doc = "Bits 9:10 - Oscillator Current Reference"]
    #[inline]
    pub fn iptat(&self) -> IPTATR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IPTATR { bits }
    }
    #[doc = "Bits 11:14 - Oscillator Current Multiplier"]
    #[inline]
    pub fn imult(&self) -> IMULTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMULTR { bits }
    }
    #[doc = "Bit 15 - Automatic Loop Control Enable"]
    #[inline]
    pub fn enalc(&self) -> ENALCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENALCR { bits }
    }
    #[doc = "Bit 16 - Clock Failure Detector Enable"]
    #[inline]
    pub fn cfden(&self) -> CFDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CFDENR { bits }
    }
    #[doc = "Bit 17 - Xosc Clock Switch Enable"]
    #[inline]
    pub fn swben(&self) -> SWBENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWBENR { bits }
    }
    #[doc = "Bits 20:23 - Start-Up Time"]
    #[inline]
    pub fn startup(&self) -> STARTUPR {
        STARTUPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Clock Failure Detector Prescaler"]
    #[inline]
    pub fn cfdpresc(&self) -> CFDPRESCR {
        CFDPRESCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline]
    pub fn xtalen(&mut self) -> _XTALENW {
        _XTALENW { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline]
    pub fn ondemand(&mut self) -> _ONDEMANDW {
        _ONDEMANDW { w: self }
    }
    #[doc = "Bit 8 - Low Buffer Gain Enable"]
    #[inline]
    pub fn lowbufgain(&mut self) -> _LOWBUFGAINW {
        _LOWBUFGAINW { w: self }
    }
    #[doc = "Bits 9:10 - Oscillator Current Reference"]
    #[inline]
    pub fn iptat(&mut self) -> _IPTATW {
        _IPTATW { w: self }
    }
    #[doc = "Bits 11:14 - Oscillator Current Multiplier"]
    #[inline]
    pub fn imult(&mut self) -> _IMULTW {
        _IMULTW { w: self }
    }
    #[doc = "Bit 15 - Automatic Loop Control Enable"]
    #[inline]
    pub fn enalc(&mut self) -> _ENALCW {
        _ENALCW { w: self }
    }
    #[doc = "Bit 16 - Clock Failure Detector Enable"]
    #[inline]
    pub fn cfden(&mut self) -> _CFDENW {
        _CFDENW { w: self }
    }
    #[doc = "Bit 17 - Xosc Clock Switch Enable"]
    #[inline]
    pub fn swben(&mut self) -> _SWBENW {
        _SWBENW { w: self }
    }
    #[doc = "Bits 20:23 - Start-Up Time"]
    #[inline]
    pub fn startup(&mut self) -> _STARTUPW {
        _STARTUPW { w: self }
    }
    #[doc = "Bits 24:27 - Clock Failure Detector Prescaler"]
    #[inline]
    pub fn cfdpresc(&mut self) -> _CFDPRESCW {
        _CFDPRESCW { w: self }
    }
}
