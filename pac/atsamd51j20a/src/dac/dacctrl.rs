#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::DACCTRL {
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
pub struct LEFTADJR {
    bits: bool,
}
impl LEFTADJR {
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
#[doc = "Possible values of the field `CCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCTRLR {
    #[doc = "100kSPS"]
    CC100K,
    #[doc = "500kSPS"]
    CC1M,
    #[doc = "1MSPS"]
    CC12M,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CCTRLR::CC100K => 0,
            CCTRLR::CC1M => 1,
            CCTRLR::CC12M => 2,
            CCTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CCTRLR {
        match value {
            0 => CCTRLR::CC100K,
            1 => CCTRLR::CC1M,
            2 => CCTRLR::CC12M,
            i => CCTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CC100K`"]
    #[inline]
    pub fn is_cc100k(&self) -> bool {
        *self == CCTRLR::CC100K
    }
    #[doc = "Checks if the value of the field is `CC1M`"]
    #[inline]
    pub fn is_cc1m(&self) -> bool {
        *self == CCTRLR::CC1M
    }
    #[doc = "Checks if the value of the field is `CC12M`"]
    #[inline]
    pub fn is_cc12m(&self) -> bool {
        *self == CCTRLR::CC12M
    }
}
#[doc = r" Value of the field"]
pub struct FEXTR {
    bits: bool,
}
impl FEXTR {
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
pub struct DITHERR {
    bits: bool,
}
impl DITHERR {
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
#[doc = "Possible values of the field `REFRESH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFRESHR {
    #[doc = "Do not Refresh"]
    REFRESH_0,
    #[doc = "Refresh every 30 us"]
    REFRESH_1,
    #[doc = "Refresh every 60 us"]
    REFRESH_2,
    #[doc = "Refresh every 90 us"]
    REFRESH_3,
    #[doc = "Refresh every 120 us"]
    REFRESH_4,
    #[doc = "Refresh every 150 us"]
    REFRESH_5,
    #[doc = "Refresh every 180 us"]
    REFRESH_6,
    #[doc = "Refresh every 210 us"]
    REFRESH_7,
    #[doc = "Refresh every 240 us"]
    REFRESH_8,
    #[doc = "Refresh every 270 us"]
    REFRESH_9,
    #[doc = "Refresh every 300 us"]
    REFRESH_10,
    #[doc = "Refresh every 330 us"]
    REFRESH_11,
    #[doc = "Refresh every 360 us"]
    REFRESH_12,
    #[doc = "Refresh every 390 us"]
    REFRESH_13,
    #[doc = "Refresh every 420 us"]
    REFRESH_14,
    #[doc = "Refresh every 450 us"]
    REFRESH_15,
}
impl REFRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFRESHR::REFRESH_0 => 0,
            REFRESHR::REFRESH_1 => 1,
            REFRESHR::REFRESH_2 => 2,
            REFRESHR::REFRESH_3 => 3,
            REFRESHR::REFRESH_4 => 4,
            REFRESHR::REFRESH_5 => 5,
            REFRESHR::REFRESH_6 => 6,
            REFRESHR::REFRESH_7 => 7,
            REFRESHR::REFRESH_8 => 8,
            REFRESHR::REFRESH_9 => 9,
            REFRESHR::REFRESH_10 => 10,
            REFRESHR::REFRESH_11 => 11,
            REFRESHR::REFRESH_12 => 12,
            REFRESHR::REFRESH_13 => 13,
            REFRESHR::REFRESH_14 => 14,
            REFRESHR::REFRESH_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFRESHR {
        match value {
            0 => REFRESHR::REFRESH_0,
            1 => REFRESHR::REFRESH_1,
            2 => REFRESHR::REFRESH_2,
            3 => REFRESHR::REFRESH_3,
            4 => REFRESHR::REFRESH_4,
            5 => REFRESHR::REFRESH_5,
            6 => REFRESHR::REFRESH_6,
            7 => REFRESHR::REFRESH_7,
            8 => REFRESHR::REFRESH_8,
            9 => REFRESHR::REFRESH_9,
            10 => REFRESHR::REFRESH_10,
            11 => REFRESHR::REFRESH_11,
            12 => REFRESHR::REFRESH_12,
            13 => REFRESHR::REFRESH_13,
            14 => REFRESHR::REFRESH_14,
            15 => REFRESHR::REFRESH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFRESH_0`"]
    #[inline]
    pub fn is_refresh_0(&self) -> bool {
        *self == REFRESHR::REFRESH_0
    }
    #[doc = "Checks if the value of the field is `REFRESH_1`"]
    #[inline]
    pub fn is_refresh_1(&self) -> bool {
        *self == REFRESHR::REFRESH_1
    }
    #[doc = "Checks if the value of the field is `REFRESH_2`"]
    #[inline]
    pub fn is_refresh_2(&self) -> bool {
        *self == REFRESHR::REFRESH_2
    }
    #[doc = "Checks if the value of the field is `REFRESH_3`"]
    #[inline]
    pub fn is_refresh_3(&self) -> bool {
        *self == REFRESHR::REFRESH_3
    }
    #[doc = "Checks if the value of the field is `REFRESH_4`"]
    #[inline]
    pub fn is_refresh_4(&self) -> bool {
        *self == REFRESHR::REFRESH_4
    }
    #[doc = "Checks if the value of the field is `REFRESH_5`"]
    #[inline]
    pub fn is_refresh_5(&self) -> bool {
        *self == REFRESHR::REFRESH_5
    }
    #[doc = "Checks if the value of the field is `REFRESH_6`"]
    #[inline]
    pub fn is_refresh_6(&self) -> bool {
        *self == REFRESHR::REFRESH_6
    }
    #[doc = "Checks if the value of the field is `REFRESH_7`"]
    #[inline]
    pub fn is_refresh_7(&self) -> bool {
        *self == REFRESHR::REFRESH_7
    }
    #[doc = "Checks if the value of the field is `REFRESH_8`"]
    #[inline]
    pub fn is_refresh_8(&self) -> bool {
        *self == REFRESHR::REFRESH_8
    }
    #[doc = "Checks if the value of the field is `REFRESH_9`"]
    #[inline]
    pub fn is_refresh_9(&self) -> bool {
        *self == REFRESHR::REFRESH_9
    }
    #[doc = "Checks if the value of the field is `REFRESH_10`"]
    #[inline]
    pub fn is_refresh_10(&self) -> bool {
        *self == REFRESHR::REFRESH_10
    }
    #[doc = "Checks if the value of the field is `REFRESH_11`"]
    #[inline]
    pub fn is_refresh_11(&self) -> bool {
        *self == REFRESHR::REFRESH_11
    }
    #[doc = "Checks if the value of the field is `REFRESH_12`"]
    #[inline]
    pub fn is_refresh_12(&self) -> bool {
        *self == REFRESHR::REFRESH_12
    }
    #[doc = "Checks if the value of the field is `REFRESH_13`"]
    #[inline]
    pub fn is_refresh_13(&self) -> bool {
        *self == REFRESHR::REFRESH_13
    }
    #[doc = "Checks if the value of the field is `REFRESH_14`"]
    #[inline]
    pub fn is_refresh_14(&self) -> bool {
        *self == REFRESHR::REFRESH_14
    }
    #[doc = "Checks if the value of the field is `REFRESH_15`"]
    #[inline]
    pub fn is_refresh_15(&self) -> bool {
        *self == REFRESHR::REFRESH_15
    }
}
#[doc = "Possible values of the field `OSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSRR {
    #[doc = "No Over Sampling"]
    OSR_1,
    #[doc = "2x Over Sampling Ratio"]
    OSR_2,
    #[doc = "4x Over Sampling Ratio"]
    OSR_4,
    #[doc = "8x Over Sampling Ratio"]
    OSR_8,
    #[doc = "16x Over Sampling Ratio"]
    OSR_16,
    #[doc = "32x Over Sampling Ratio"]
    OSR_32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSRR::OSR_1 => 0,
            OSRR::OSR_2 => 1,
            OSRR::OSR_4 => 2,
            OSRR::OSR_8 => 3,
            OSRR::OSR_16 => 4,
            OSRR::OSR_32 => 5,
            OSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSRR {
        match value {
            0 => OSRR::OSR_1,
            1 => OSRR::OSR_2,
            2 => OSRR::OSR_4,
            3 => OSRR::OSR_8,
            4 => OSRR::OSR_16,
            5 => OSRR::OSR_32,
            i => OSRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OSR_1`"]
    #[inline]
    pub fn is_osr_1(&self) -> bool {
        *self == OSRR::OSR_1
    }
    #[doc = "Checks if the value of the field is `OSR_2`"]
    #[inline]
    pub fn is_osr_2(&self) -> bool {
        *self == OSRR::OSR_2
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline]
    pub fn is_osr_4(&self) -> bool {
        *self == OSRR::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline]
    pub fn is_osr_8(&self) -> bool {
        *self == OSRR::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline]
    pub fn is_osr_16(&self) -> bool {
        *self == OSRR::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_32`"]
    #[inline]
    pub fn is_osr_32(&self) -> bool {
        *self == OSRR::OSR_32
    }
}
#[doc = r" Proxy"]
pub struct _LEFTADJW<'a> {
    w: &'a mut W,
}
impl<'a> _LEFTADJW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
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
#[doc = "Values that can be written to the field `CCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCTRLW {
    #[doc = "100kSPS"]
    CC100K,
    #[doc = "500kSPS"]
    CC1M,
    #[doc = "1MSPS"]
    CC12M,
}
impl CCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CCTRLW::CC100K => 0,
            CCTRLW::CC1M => 1,
            CCTRLW::CC12M => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "100kSPS"]
    #[inline]
    pub fn cc100k(self) -> &'a mut W {
        self.variant(CCTRLW::CC100K)
    }
    #[doc = "500kSPS"]
    #[inline]
    pub fn cc1m(self) -> &'a mut W {
        self.variant(CCTRLW::CC1M)
    }
    #[doc = "1MSPS"]
    #[inline]
    pub fn cc12m(self) -> &'a mut W {
        self.variant(CCTRLW::CC12M)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _FEXTW<'a> {
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
pub struct _DITHERW<'a> {
    w: &'a mut W,
}
impl<'a> _DITHERW<'a> {
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
#[doc = "Values that can be written to the field `REFRESH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFRESHW {
    #[doc = "Do not Refresh"]
    REFRESH_0,
    #[doc = "Refresh every 30 us"]
    REFRESH_1,
    #[doc = "Refresh every 60 us"]
    REFRESH_2,
    #[doc = "Refresh every 90 us"]
    REFRESH_3,
    #[doc = "Refresh every 120 us"]
    REFRESH_4,
    #[doc = "Refresh every 150 us"]
    REFRESH_5,
    #[doc = "Refresh every 180 us"]
    REFRESH_6,
    #[doc = "Refresh every 210 us"]
    REFRESH_7,
    #[doc = "Refresh every 240 us"]
    REFRESH_8,
    #[doc = "Refresh every 270 us"]
    REFRESH_9,
    #[doc = "Refresh every 300 us"]
    REFRESH_10,
    #[doc = "Refresh every 330 us"]
    REFRESH_11,
    #[doc = "Refresh every 360 us"]
    REFRESH_12,
    #[doc = "Refresh every 390 us"]
    REFRESH_13,
    #[doc = "Refresh every 420 us"]
    REFRESH_14,
    #[doc = "Refresh every 450 us"]
    REFRESH_15,
}
impl REFRESHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFRESHW::REFRESH_0 => 0,
            REFRESHW::REFRESH_1 => 1,
            REFRESHW::REFRESH_2 => 2,
            REFRESHW::REFRESH_3 => 3,
            REFRESHW::REFRESH_4 => 4,
            REFRESHW::REFRESH_5 => 5,
            REFRESHW::REFRESH_6 => 6,
            REFRESHW::REFRESH_7 => 7,
            REFRESHW::REFRESH_8 => 8,
            REFRESHW::REFRESH_9 => 9,
            REFRESHW::REFRESH_10 => 10,
            REFRESHW::REFRESH_11 => 11,
            REFRESHW::REFRESH_12 => 12,
            REFRESHW::REFRESH_13 => 13,
            REFRESHW::REFRESH_14 => 14,
            REFRESHW::REFRESH_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _REFRESHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFRESHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do not Refresh"]
    #[inline]
    pub fn refresh_0(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_0)
    }
    #[doc = "Refresh every 30 us"]
    #[inline]
    pub fn refresh_1(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_1)
    }
    #[doc = "Refresh every 60 us"]
    #[inline]
    pub fn refresh_2(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_2)
    }
    #[doc = "Refresh every 90 us"]
    #[inline]
    pub fn refresh_3(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_3)
    }
    #[doc = "Refresh every 120 us"]
    #[inline]
    pub fn refresh_4(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_4)
    }
    #[doc = "Refresh every 150 us"]
    #[inline]
    pub fn refresh_5(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_5)
    }
    #[doc = "Refresh every 180 us"]
    #[inline]
    pub fn refresh_6(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_6)
    }
    #[doc = "Refresh every 210 us"]
    #[inline]
    pub fn refresh_7(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_7)
    }
    #[doc = "Refresh every 240 us"]
    #[inline]
    pub fn refresh_8(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_8)
    }
    #[doc = "Refresh every 270 us"]
    #[inline]
    pub fn refresh_9(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_9)
    }
    #[doc = "Refresh every 300 us"]
    #[inline]
    pub fn refresh_10(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_10)
    }
    #[doc = "Refresh every 330 us"]
    #[inline]
    pub fn refresh_11(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_11)
    }
    #[doc = "Refresh every 360 us"]
    #[inline]
    pub fn refresh_12(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_12)
    }
    #[doc = "Refresh every 390 us"]
    #[inline]
    pub fn refresh_13(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_13)
    }
    #[doc = "Refresh every 420 us"]
    #[inline]
    pub fn refresh_14(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_14)
    }
    #[doc = "Refresh every 450 us"]
    #[inline]
    pub fn refresh_15(self) -> &'a mut W {
        self.variant(REFRESHW::REFRESH_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSRW {
    #[doc = "No Over Sampling"]
    OSR_1,
    #[doc = "2x Over Sampling Ratio"]
    OSR_2,
    #[doc = "4x Over Sampling Ratio"]
    OSR_4,
    #[doc = "8x Over Sampling Ratio"]
    OSR_8,
    #[doc = "16x Over Sampling Ratio"]
    OSR_16,
    #[doc = "32x Over Sampling Ratio"]
    OSR_32,
}
impl OSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSRW::OSR_1 => 0,
            OSRW::OSR_2 => 1,
            OSRW::OSR_4 => 2,
            OSRW::OSR_8 => 3,
            OSRW::OSR_16 => 4,
            OSRW::OSR_32 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSRW<'a> {
    w: &'a mut W,
}
impl<'a> _OSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Over Sampling"]
    #[inline]
    pub fn osr_1(self) -> &'a mut W {
        self.variant(OSRW::OSR_1)
    }
    #[doc = "2x Over Sampling Ratio"]
    #[inline]
    pub fn osr_2(self) -> &'a mut W {
        self.variant(OSRW::OSR_2)
    }
    #[doc = "4x Over Sampling Ratio"]
    #[inline]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSRW::OSR_4)
    }
    #[doc = "8x Over Sampling Ratio"]
    #[inline]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSRW::OSR_8)
    }
    #[doc = "16x Over Sampling Ratio"]
    #[inline]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSRW::OSR_16)
    }
    #[doc = "32x Over Sampling Ratio"]
    #[inline]
    pub fn osr_32(self) -> &'a mut W {
        self.variant(OSRW::OSR_32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline]
    pub fn leftadj(&self) -> LEFTADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        LEFTADJR { bits }
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline]
    pub fn cctrl(&self) -> CCTRLR {
        CCTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 5 - Standalone Filter"]
    #[inline]
    pub fn fext(&self) -> FEXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        FEXTR { bits }
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
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline]
    pub fn dither(&self) -> DITHERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DITHERR { bits }
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline]
    pub fn refresh(&self) -> REFRESHR {
        REFRESHR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 13:15 - Sampling Rate"]
    #[inline]
    pub fn osr(&self) -> OSRR {
        OSRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline]
    pub fn leftadj(&mut self) -> _LEFTADJW {
        _LEFTADJW { w: self }
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline]
    pub fn cctrl(&mut self) -> _CCTRLW {
        _CCTRLW { w: self }
    }
    #[doc = "Bit 5 - Standalone Filter"]
    #[inline]
    pub fn fext(&mut self) -> _FEXTW {
        _FEXTW { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline]
    pub fn dither(&mut self) -> _DITHERW {
        _DITHERW { w: self }
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline]
    pub fn refresh(&mut self) -> _REFRESHW {
        _REFRESHW { w: self }
    }
    #[doc = "Bits 13:15 - Sampling Rate"]
    #[inline]
    pub fn osr(&mut self) -> _OSRW {
        _OSRW { w: self }
    }
}
