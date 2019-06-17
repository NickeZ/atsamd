#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::XOSC32K {
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
pub struct EN32KR {
    bits: bool,
}
impl EN32KR {
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
pub struct EN1KR {
    bits: bool,
}
impl EN1KR {
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
#[doc = "Possible values of the field `STARTUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPR {
    #[doc = "62.6 ms"]
    CYCLE2048,
    #[doc = "125 ms"]
    CYCLE4096,
    #[doc = "500 ms"]
    CYCLE16384,
    #[doc = "1000 ms"]
    CYCLE32768,
    #[doc = "2000 ms"]
    CYCLE65536,
    #[doc = "4000 ms"]
    CYCLE131072,
    #[doc = "8000 ms"]
    CYCLE262144,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STARTUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTUPR::CYCLE2048 => 0,
            STARTUPR::CYCLE4096 => 1,
            STARTUPR::CYCLE16384 => 2,
            STARTUPR::CYCLE32768 => 3,
            STARTUPR::CYCLE65536 => 4,
            STARTUPR::CYCLE131072 => 5,
            STARTUPR::CYCLE262144 => 6,
            STARTUPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTUPR {
        match value {
            0 => STARTUPR::CYCLE2048,
            1 => STARTUPR::CYCLE4096,
            2 => STARTUPR::CYCLE16384,
            3 => STARTUPR::CYCLE32768,
            4 => STARTUPR::CYCLE65536,
            5 => STARTUPR::CYCLE131072,
            6 => STARTUPR::CYCLE262144,
            i => STARTUPR::_Reserved(i),
        }
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
    #[doc = "Checks if the value of the field is `CYCLE65536`"]
    #[inline]
    pub fn is_cycle65536(&self) -> bool {
        *self == STARTUPR::CYCLE65536
    }
    #[doc = "Checks if the value of the field is `CYCLE131072`"]
    #[inline]
    pub fn is_cycle131072(&self) -> bool {
        *self == STARTUPR::CYCLE131072
    }
    #[doc = "Checks if the value of the field is `CYCLE262144`"]
    #[inline]
    pub fn is_cycle262144(&self) -> bool {
        *self == STARTUPR::CYCLE262144
    }
}
#[doc = r" Value of the field"]
pub struct WRTLOCKR {
    bits: bool,
}
impl WRTLOCKR {
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
#[doc = "Possible values of the field `CGM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGMR {
    #[doc = "Standard mode"]
    XT,
    #[doc = "High Speed mode"]
    HS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CGMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CGMR::XT => 1,
            CGMR::HS => 2,
            CGMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CGMR {
        match value {
            1 => CGMR::XT,
            2 => CGMR::HS,
            i => CGMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline]
    pub fn is_xt(&self) -> bool {
        *self == CGMR::XT
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline]
    pub fn is_hs(&self) -> bool {
        *self == CGMR::HS
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EN32KW<'a> {
    w: &'a mut W,
}
impl<'a> _EN32KW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EN1KW<'a> {
    w: &'a mut W,
}
impl<'a> _EN1KW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STARTUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPW {
    #[doc = "62.6 ms"]
    CYCLE2048,
    #[doc = "125 ms"]
    CYCLE4096,
    #[doc = "500 ms"]
    CYCLE16384,
    #[doc = "1000 ms"]
    CYCLE32768,
    #[doc = "2000 ms"]
    CYCLE65536,
    #[doc = "4000 ms"]
    CYCLE131072,
    #[doc = "8000 ms"]
    CYCLE262144,
}
impl STARTUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTUPW::CYCLE2048 => 0,
            STARTUPW::CYCLE4096 => 1,
            STARTUPW::CYCLE16384 => 2,
            STARTUPW::CYCLE32768 => 3,
            STARTUPW::CYCLE65536 => 4,
            STARTUPW::CYCLE131072 => 5,
            STARTUPW::CYCLE262144 => 6,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "62.6 ms"]
    #[inline]
    pub fn cycle2048(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE2048)
    }
    #[doc = "125 ms"]
    #[inline]
    pub fn cycle4096(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE4096)
    }
    #[doc = "500 ms"]
    #[inline]
    pub fn cycle16384(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE16384)
    }
    #[doc = "1000 ms"]
    #[inline]
    pub fn cycle32768(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE32768)
    }
    #[doc = "2000 ms"]
    #[inline]
    pub fn cycle65536(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE65536)
    }
    #[doc = "4000 ms"]
    #[inline]
    pub fn cycle131072(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE131072)
    }
    #[doc = "8000 ms"]
    #[inline]
    pub fn cycle262144(self) -> &'a mut W {
        self.variant(STARTUPW::CYCLE262144)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRTLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WRTLOCKW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CGM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGMW {
    #[doc = "Standard mode"]
    XT,
    #[doc = "High Speed mode"]
    HS,
}
impl CGMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CGMW::XT => 1,
            CGMW::HS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGMW<'a> {
    w: &'a mut W,
}
impl<'a> _CGMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard mode"]
    #[inline]
    pub fn xt(self) -> &'a mut W {
        self.variant(CGMW::XT)
    }
    #[doc = "High Speed mode"]
    #[inline]
    pub fn hs(self) -> &'a mut W {
        self.variant(CGMW::HS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline]
    pub fn xtalen(&self) -> XTALENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        XTALENR { bits }
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline]
    pub fn en32k(&self) -> EN32KR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        EN32KR { bits }
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline]
    pub fn en1k(&self) -> EN1KR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        EN1KR { bits }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline]
    pub fn ondemand(&self) -> ONDEMANDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ONDEMANDR { bits }
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline]
    pub fn startup(&self) -> STARTUPR {
        STARTUPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline]
    pub fn wrtlock(&self) -> WRTLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        WRTLOCKR { bits }
    }
    #[doc = "Bits 13:14 - Control Gain Mode"]
    #[inline]
    pub fn cgm(&self) -> CGMR {
        CGMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8320 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
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
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline]
    pub fn en32k(&mut self) -> _EN32KW {
        _EN32KW { w: self }
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline]
    pub fn en1k(&mut self) -> _EN1KW {
        _EN1KW { w: self }
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
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline]
    pub fn startup(&mut self) -> _STARTUPW {
        _STARTUPW { w: self }
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline]
    pub fn wrtlock(&mut self) -> _WRTLOCKW {
        _WRTLOCKW { w: self }
    }
    #[doc = "Bits 13:14 - Control Gain Mode"]
    #[inline]
    pub fn cgm(&mut self) -> _CGMW {
        _CGMW { w: self }
    }
}
