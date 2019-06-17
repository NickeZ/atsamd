#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DPLLCTRLB {
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
#[doc = "Possible values of the field `FILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERR {
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    FILTER1,
    #[doc = "Bandwidth = 131Khz and Damping Factor = 1.08"]
    FILTER2,
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    FILTER3,
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    FILTER4,
    #[doc = "Bandwidth = 131Khz and Damping Factor = 0.56"]
    FILTER5,
    #[doc = "Bandwidth = 185Khz and Damping Factor = 0.79"]
    FILTER6,
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    FILTER7,
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    FILTER8,
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    FILTER9,
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    FILTER10,
    #[doc = "Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    FILTER11,
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    FILTER12,
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    FILTER13,
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    FILTER14,
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    FILTER15,
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    FILTER16,
}
impl FILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTERR::FILTER1 => 0,
            FILTERR::FILTER2 => 1,
            FILTERR::FILTER3 => 2,
            FILTERR::FILTER4 => 3,
            FILTERR::FILTER5 => 4,
            FILTERR::FILTER6 => 5,
            FILTERR::FILTER7 => 6,
            FILTERR::FILTER8 => 7,
            FILTERR::FILTER9 => 8,
            FILTERR::FILTER10 => 9,
            FILTERR::FILTER11 => 10,
            FILTERR::FILTER12 => 11,
            FILTERR::FILTER13 => 12,
            FILTERR::FILTER14 => 13,
            FILTERR::FILTER15 => 14,
            FILTERR::FILTER16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTERR {
        match value {
            0 => FILTERR::FILTER1,
            1 => FILTERR::FILTER2,
            2 => FILTERR::FILTER3,
            3 => FILTERR::FILTER4,
            4 => FILTERR::FILTER5,
            5 => FILTERR::FILTER6,
            6 => FILTERR::FILTER7,
            7 => FILTERR::FILTER8,
            8 => FILTERR::FILTER9,
            9 => FILTERR::FILTER10,
            10 => FILTERR::FILTER11,
            11 => FILTERR::FILTER12,
            12 => FILTERR::FILTER13,
            13 => FILTERR::FILTER14,
            14 => FILTERR::FILTER15,
            15 => FILTERR::FILTER16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER1`"]
    #[inline]
    pub fn is_filter1(&self) -> bool {
        *self == FILTERR::FILTER1
    }
    #[doc = "Checks if the value of the field is `FILTER2`"]
    #[inline]
    pub fn is_filter2(&self) -> bool {
        *self == FILTERR::FILTER2
    }
    #[doc = "Checks if the value of the field is `FILTER3`"]
    #[inline]
    pub fn is_filter3(&self) -> bool {
        *self == FILTERR::FILTER3
    }
    #[doc = "Checks if the value of the field is `FILTER4`"]
    #[inline]
    pub fn is_filter4(&self) -> bool {
        *self == FILTERR::FILTER4
    }
    #[doc = "Checks if the value of the field is `FILTER5`"]
    #[inline]
    pub fn is_filter5(&self) -> bool {
        *self == FILTERR::FILTER5
    }
    #[doc = "Checks if the value of the field is `FILTER6`"]
    #[inline]
    pub fn is_filter6(&self) -> bool {
        *self == FILTERR::FILTER6
    }
    #[doc = "Checks if the value of the field is `FILTER7`"]
    #[inline]
    pub fn is_filter7(&self) -> bool {
        *self == FILTERR::FILTER7
    }
    #[doc = "Checks if the value of the field is `FILTER8`"]
    #[inline]
    pub fn is_filter8(&self) -> bool {
        *self == FILTERR::FILTER8
    }
    #[doc = "Checks if the value of the field is `FILTER9`"]
    #[inline]
    pub fn is_filter9(&self) -> bool {
        *self == FILTERR::FILTER9
    }
    #[doc = "Checks if the value of the field is `FILTER10`"]
    #[inline]
    pub fn is_filter10(&self) -> bool {
        *self == FILTERR::FILTER10
    }
    #[doc = "Checks if the value of the field is `FILTER11`"]
    #[inline]
    pub fn is_filter11(&self) -> bool {
        *self == FILTERR::FILTER11
    }
    #[doc = "Checks if the value of the field is `FILTER12`"]
    #[inline]
    pub fn is_filter12(&self) -> bool {
        *self == FILTERR::FILTER12
    }
    #[doc = "Checks if the value of the field is `FILTER13`"]
    #[inline]
    pub fn is_filter13(&self) -> bool {
        *self == FILTERR::FILTER13
    }
    #[doc = "Checks if the value of the field is `FILTER14`"]
    #[inline]
    pub fn is_filter14(&self) -> bool {
        *self == FILTERR::FILTER14
    }
    #[doc = "Checks if the value of the field is `FILTER15`"]
    #[inline]
    pub fn is_filter15(&self) -> bool {
        *self == FILTERR::FILTER15
    }
    #[doc = "Checks if the value of the field is `FILTER16`"]
    #[inline]
    pub fn is_filter16(&self) -> bool {
        *self == FILTERR::FILTER16
    }
}
#[doc = r" Value of the field"]
pub struct WUFR {
    bits: bool,
}
impl WUFR {
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
#[doc = "Possible values of the field `REFCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCLKR {
    #[doc = "Dedicated GCLK clock reference"]
    GCLK,
    #[doc = "XOSC32K clock reference"]
    XOSC32,
    #[doc = "XOSC0 clock reference"]
    XOSC0,
    #[doc = "XOSC1 clock reference"]
    XOSC1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFCLKR::GCLK => 0,
            REFCLKR::XOSC32 => 1,
            REFCLKR::XOSC0 => 2,
            REFCLKR::XOSC1 => 3,
            REFCLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFCLKR {
        match value {
            0 => REFCLKR::GCLK,
            1 => REFCLKR::XOSC32,
            2 => REFCLKR::XOSC0,
            3 => REFCLKR::XOSC1,
            i => REFCLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline]
    pub fn is_gclk(&self) -> bool {
        *self == REFCLKR::GCLK
    }
    #[doc = "Checks if the value of the field is `XOSC32`"]
    #[inline]
    pub fn is_xosc32(&self) -> bool {
        *self == REFCLKR::XOSC32
    }
    #[doc = "Checks if the value of the field is `XOSC0`"]
    #[inline]
    pub fn is_xosc0(&self) -> bool {
        *self == REFCLKR::XOSC0
    }
    #[doc = "Checks if the value of the field is `XOSC1`"]
    #[inline]
    pub fn is_xosc1(&self) -> bool {
        *self == REFCLKR::XOSC1
    }
}
#[doc = "Possible values of the field `LTIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTIMER {
    #[doc = "No time-out. Automatic lock"]
    DEFAULT,
    #[doc = "Time-out if no lock within 800us"]
    _800US,
    #[doc = "Time-out if no lock within 900us"]
    _900US,
    #[doc = "Time-out if no lock within 1ms"]
    _1MS,
    #[doc = "Time-out if no lock within 1.1ms"]
    _1P1MS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LTIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LTIMER::DEFAULT => 0,
            LTIMER::_800US => 4,
            LTIMER::_900US => 5,
            LTIMER::_1MS => 6,
            LTIMER::_1P1MS => 7,
            LTIMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LTIMER {
        match value {
            0 => LTIMER::DEFAULT,
            4 => LTIMER::_800US,
            5 => LTIMER::_900US,
            6 => LTIMER::_1MS,
            7 => LTIMER::_1P1MS,
            i => LTIMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == LTIMER::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_800US`"]
    #[inline]
    pub fn is_800us(&self) -> bool {
        *self == LTIMER::_800US
    }
    #[doc = "Checks if the value of the field is `_900US`"]
    #[inline]
    pub fn is_900us(&self) -> bool {
        *self == LTIMER::_900US
    }
    #[doc = "Checks if the value of the field is `_1MS`"]
    #[inline]
    pub fn is_1ms(&self) -> bool {
        *self == LTIMER::_1MS
    }
    #[doc = "Checks if the value of the field is `_1P1MS`"]
    #[inline]
    pub fn is_1p1ms(&self) -> bool {
        *self == LTIMER::_1P1MS
    }
}
#[doc = r" Value of the field"]
pub struct LBYPASSR {
    bits: bool,
}
impl LBYPASSR {
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
#[doc = "Possible values of the field `DCOFILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOFILTERR {
    #[doc = "Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    FILTER1,
    #[doc = "Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    FILTER2,
    #[doc = "Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    FILTER3,
    #[doc = "Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    FILTER4,
    #[doc = "Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    FILTER5,
    #[doc = "Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    FILTER6,
    #[doc = "Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    FILTER7,
    #[doc = "Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    FILTER8,
}
impl DCOFILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOFILTERR::FILTER1 => 0,
            DCOFILTERR::FILTER2 => 1,
            DCOFILTERR::FILTER3 => 2,
            DCOFILTERR::FILTER4 => 3,
            DCOFILTERR::FILTER5 => 4,
            DCOFILTERR::FILTER6 => 5,
            DCOFILTERR::FILTER7 => 6,
            DCOFILTERR::FILTER8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOFILTERR {
        match value {
            0 => DCOFILTERR::FILTER1,
            1 => DCOFILTERR::FILTER2,
            2 => DCOFILTERR::FILTER3,
            3 => DCOFILTERR::FILTER4,
            4 => DCOFILTERR::FILTER5,
            5 => DCOFILTERR::FILTER6,
            6 => DCOFILTERR::FILTER7,
            7 => DCOFILTERR::FILTER8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER1`"]
    #[inline]
    pub fn is_filter1(&self) -> bool {
        *self == DCOFILTERR::FILTER1
    }
    #[doc = "Checks if the value of the field is `FILTER2`"]
    #[inline]
    pub fn is_filter2(&self) -> bool {
        *self == DCOFILTERR::FILTER2
    }
    #[doc = "Checks if the value of the field is `FILTER3`"]
    #[inline]
    pub fn is_filter3(&self) -> bool {
        *self == DCOFILTERR::FILTER3
    }
    #[doc = "Checks if the value of the field is `FILTER4`"]
    #[inline]
    pub fn is_filter4(&self) -> bool {
        *self == DCOFILTERR::FILTER4
    }
    #[doc = "Checks if the value of the field is `FILTER5`"]
    #[inline]
    pub fn is_filter5(&self) -> bool {
        *self == DCOFILTERR::FILTER5
    }
    #[doc = "Checks if the value of the field is `FILTER6`"]
    #[inline]
    pub fn is_filter6(&self) -> bool {
        *self == DCOFILTERR::FILTER6
    }
    #[doc = "Checks if the value of the field is `FILTER7`"]
    #[inline]
    pub fn is_filter7(&self) -> bool {
        *self == DCOFILTERR::FILTER7
    }
    #[doc = "Checks if the value of the field is `FILTER8`"]
    #[inline]
    pub fn is_filter8(&self) -> bool {
        *self == DCOFILTERR::FILTER8
    }
}
#[doc = r" Value of the field"]
pub struct DCOENR {
    bits: bool,
}
impl DCOENR {
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
pub struct DIVR {
    bits: u16,
}
impl DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERW {
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    FILTER1,
    #[doc = "Bandwidth = 131Khz and Damping Factor = 1.08"]
    FILTER2,
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    FILTER3,
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    FILTER4,
    #[doc = "Bandwidth = 131Khz and Damping Factor = 0.56"]
    FILTER5,
    #[doc = "Bandwidth = 185Khz and Damping Factor = 0.79"]
    FILTER6,
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    FILTER7,
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    FILTER8,
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    FILTER9,
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    FILTER10,
    #[doc = "Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    FILTER11,
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    FILTER12,
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    FILTER13,
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    FILTER14,
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    FILTER15,
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    FILTER16,
}
impl FILTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTERW::FILTER1 => 0,
            FILTERW::FILTER2 => 1,
            FILTERW::FILTER3 => 2,
            FILTERW::FILTER4 => 3,
            FILTERW::FILTER5 => 4,
            FILTERW::FILTER6 => 5,
            FILTERW::FILTER7 => 6,
            FILTERW::FILTER8 => 7,
            FILTERW::FILTER9 => 8,
            FILTERW::FILTER10 => 9,
            FILTERW::FILTER11 => 10,
            FILTERW::FILTER12 => 11,
            FILTERW::FILTER13 => 12,
            FILTERW::FILTER14 => 13,
            FILTERW::FILTER15 => 14,
            FILTERW::FILTER16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.76"]
    #[inline]
    pub fn filter1(self) -> &'a mut W {
        self.variant(FILTERW::FILTER1)
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 1.08"]
    #[inline]
    pub fn filter2(self) -> &'a mut W {
        self.variant(FILTERW::FILTER2)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.38"]
    #[inline]
    pub fn filter3(self) -> &'a mut W {
        self.variant(FILTERW::FILTER3)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.54"]
    #[inline]
    pub fn filter4(self) -> &'a mut W {
        self.variant(FILTERW::FILTER4)
    }
    #[doc = "Bandwidth = 131Khz and Damping Factor = 0.56"]
    #[inline]
    pub fn filter5(self) -> &'a mut W {
        self.variant(FILTERW::FILTER5)
    }
    #[doc = "Bandwidth = 185Khz and Damping Factor = 0.79"]
    #[inline]
    pub fn filter6(self) -> &'a mut W {
        self.variant(FILTERW::FILTER6)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 0.28"]
    #[inline]
    pub fn filter7(self) -> &'a mut W {
        self.variant(FILTERW::FILTER7)
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 0.39"]
    #[inline]
    pub fn filter8(self) -> &'a mut W {
        self.variant(FILTERW::FILTER8)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 1.49"]
    #[inline]
    pub fn filter9(self) -> &'a mut W {
        self.variant(FILTERW::FILTER9)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 2.11"]
    #[inline]
    pub fn filter10(self) -> &'a mut W {
        self.variant(FILTERW::FILTER10)
    }
    #[doc = "Bandwidth = 23.2Khz and Damping Factor = 0.75"]
    #[inline]
    pub fn filter11(self) -> &'a mut W {
        self.variant(FILTERW::FILTER11)
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 1.06"]
    #[inline]
    pub fn filter12(self) -> &'a mut W {
        self.variant(FILTERW::FILTER12)
    }
    #[doc = "Bandwidth = 65.6Khz and Damping Factor = 1.07"]
    #[inline]
    pub fn filter13(self) -> &'a mut W {
        self.variant(FILTERW::FILTER13)
    }
    #[doc = "Bandwidth = 92.7Khz and Damping Factor = 1.51"]
    #[inline]
    pub fn filter14(self) -> &'a mut W {
        self.variant(FILTERW::FILTER14)
    }
    #[doc = "Bandwidth = 32.8Khz and Damping Factor = 0.53"]
    #[inline]
    pub fn filter15(self) -> &'a mut W {
        self.variant(FILTERW::FILTER15)
    }
    #[doc = "Bandwidth = 46.4Khz and Damping Factor = 0.75"]
    #[inline]
    pub fn filter16(self) -> &'a mut W {
        self.variant(FILTERW::FILTER16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WUFW<'a> {
    w: &'a mut W,
}
impl<'a> _WUFW<'a> {
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
#[doc = "Values that can be written to the field `REFCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCLKW {
    #[doc = "Dedicated GCLK clock reference"]
    GCLK,
    #[doc = "XOSC32K clock reference"]
    XOSC32,
    #[doc = "XOSC0 clock reference"]
    XOSC0,
    #[doc = "XOSC1 clock reference"]
    XOSC1,
}
impl REFCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFCLKW::GCLK => 0,
            REFCLKW::XOSC32 => 1,
            REFCLKW::XOSC0 => 2,
            REFCLKW::XOSC1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _REFCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFCLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Dedicated GCLK clock reference"]
    #[inline]
    pub fn gclk(self) -> &'a mut W {
        self.variant(REFCLKW::GCLK)
    }
    #[doc = "XOSC32K clock reference"]
    #[inline]
    pub fn xosc32(self) -> &'a mut W {
        self.variant(REFCLKW::XOSC32)
    }
    #[doc = "XOSC0 clock reference"]
    #[inline]
    pub fn xosc0(self) -> &'a mut W {
        self.variant(REFCLKW::XOSC0)
    }
    #[doc = "XOSC1 clock reference"]
    #[inline]
    pub fn xosc1(self) -> &'a mut W {
        self.variant(REFCLKW::XOSC1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LTIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTIMEW {
    #[doc = "No time-out. Automatic lock"]
    DEFAULT,
    #[doc = "Time-out if no lock within 800us"]
    _800US,
    #[doc = "Time-out if no lock within 900us"]
    _900US,
    #[doc = "Time-out if no lock within 1ms"]
    _1MS,
    #[doc = "Time-out if no lock within 1.1ms"]
    _1P1MS,
}
impl LTIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LTIMEW::DEFAULT => 0,
            LTIMEW::_800US => 4,
            LTIMEW::_900US => 5,
            LTIMEW::_1MS => 6,
            LTIMEW::_1P1MS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LTIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _LTIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LTIMEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No time-out. Automatic lock"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(LTIMEW::DEFAULT)
    }
    #[doc = "Time-out if no lock within 800us"]
    #[inline]
    pub fn _800us(self) -> &'a mut W {
        self.variant(LTIMEW::_800US)
    }
    #[doc = "Time-out if no lock within 900us"]
    #[inline]
    pub fn _900us(self) -> &'a mut W {
        self.variant(LTIMEW::_900US)
    }
    #[doc = "Time-out if no lock within 1ms"]
    #[inline]
    pub fn _1ms(self) -> &'a mut W {
        self.variant(LTIMEW::_1MS)
    }
    #[doc = "Time-out if no lock within 1.1ms"]
    #[inline]
    pub fn _1p1ms(self) -> &'a mut W {
        self.variant(LTIMEW::_1P1MS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _LBYPASSW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOFILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOFILTERW {
    #[doc = "Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    FILTER1,
    #[doc = "Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    FILTER2,
    #[doc = "Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    FILTER3,
    #[doc = "Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    FILTER4,
    #[doc = "Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    FILTER5,
    #[doc = "Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    FILTER6,
    #[doc = "Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    FILTER7,
    #[doc = "Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    FILTER8,
}
impl DCOFILTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOFILTERW::FILTER1 => 0,
            DCOFILTERW::FILTER2 => 1,
            DCOFILTERW::FILTER3 => 2,
            DCOFILTERW::FILTER4 => 3,
            DCOFILTERW::FILTER5 => 4,
            DCOFILTERW::FILTER6 => 5,
            DCOFILTERW::FILTER7 => 6,
            DCOFILTERW::FILTER8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOFILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOFILTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOFILTERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Capacitor(pF) = 0.5 and Bandwidth Fn (MHz) = 3.21"]
    #[inline]
    pub fn filter1(self) -> &'a mut W {
        self.variant(DCOFILTERW::FILTER1)
    }
    #[doc = "Capacitor(pF) = 1 and Bandwidth Fn (MHz) = 1.6"]
    #[inline]
    pub fn filter2(self) -> &'a mut W {
        self.variant(DCOFILTERW::FILTER2)
    }
    #[doc = "Capacitor(pF) = 1.5 and Bandwidth Fn (MHz) = 1.1"]
    #[inline]
    pub fn filter3(self) -> &'a mut W {
        self.variant(DCOFILTERW::FILTER3)
    }
    #[doc = "Capacitor(pF) = 2 and Bandwidth Fn (MHz) = 0.8"]
    #[inline]
    pub fn filter4(self) -> &'a mut W {
        self.variant(DCOFILTERW::FILTER4)
    }
    #[doc = "Capacitor(pF) = 2.5 and Bandwidth Fn (MHz) = 0.64"]
    #[inline]
    pub fn filter5(self) -> &'a mut W {
        self.variant(DCOFILTERW::FILTER5)
    }
    #[doc = "Capacitor(pF) = 3 and Bandwidth Fn (MHz) = 0.55"]
    #[inline]
    pub fn filter6(self) -> &'a mut W {
        self.variant(DCOFILTERW::FILTER6)
    }
    #[doc = "Capacitor(pF) = 3.5 and Bandwidth Fn (MHz) = 0.45"]
    #[inline]
    pub fn filter7(self) -> &'a mut W {
        self.variant(DCOFILTERW::FILTER7)
    }
    #[doc = "Capacitor(pF) = 4 and Bandwidth Fn (MHz) = 0.4"]
    #[inline]
    pub fn filter8(self) -> &'a mut W {
        self.variant(DCOFILTERW::FILTER8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOENW<'a> {
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
pub struct _DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
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
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline]
    pub fn filter(&self) -> FILTERR {
        FILTERR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline]
    pub fn wuf(&self) -> WUFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WUFR { bits }
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline]
    pub fn refclk(&self) -> REFCLKR {
        REFCLKR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline]
    pub fn ltime(&self) -> LTIMER {
        LTIMER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline]
    pub fn lbypass(&self) -> LBYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBYPASSR { bits }
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline]
    pub fn dcofilter(&self) -> DCOFILTERR {
        DCOFILTERR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline]
    pub fn dcoen(&self) -> DCOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCOENR { bits }
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline]
    pub fn div(&self) -> DIVR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DIVR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline]
    pub fn filter(&mut self) -> _FILTERW {
        _FILTERW { w: self }
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline]
    pub fn wuf(&mut self) -> _WUFW {
        _WUFW { w: self }
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline]
    pub fn refclk(&mut self) -> _REFCLKW {
        _REFCLKW { w: self }
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline]
    pub fn ltime(&mut self) -> _LTIMEW {
        _LTIMEW { w: self }
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline]
    pub fn lbypass(&mut self) -> _LBYPASSW {
        _LBYPASSW { w: self }
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline]
    pub fn dcofilter(&mut self) -> _DCOFILTERW {
        _DCOFILTERW { w: self }
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline]
    pub fn dcoen(&mut self) -> _DCOENW {
        _DCOENW { w: self }
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline]
    pub fn div(&mut self) -> _DIVW {
        _DIVW { w: self }
    }
}
