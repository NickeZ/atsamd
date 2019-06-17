#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLB {
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
#[doc = "Possible values of the field `CHSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSIZER {
    #[doc = "8 bits"]
    _8_BIT,
    #[doc = "9 bits"]
    _9_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHSIZER::_8_BIT => 0,
            CHSIZER::_9_BIT => 1,
            CHSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHSIZER {
        match value {
            0 => CHSIZER::_8_BIT,
            1 => CHSIZER::_9_BIT,
            i => CHSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == CHSIZER::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline]
    pub fn is_9_bit(&self) -> bool {
        *self == CHSIZER::_9_BIT
    }
}
#[doc = r" Value of the field"]
pub struct PLOADENR {
    bits: bool,
}
impl PLOADENR {
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
pub struct SSDER {
    bits: bool,
}
impl SSDER {
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
pub struct MSSENR {
    bits: bool,
}
impl MSSENR {
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
#[doc = "Possible values of the field `AMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMODER {
    #[doc = "SPI Address mask "]
    MASK,
    #[doc = "Two unique Addressess"]
    _2_ADDRESSES,
    #[doc = "Address Range"]
    RANGE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AMODER::MASK => 0,
            AMODER::_2_ADDRESSES => 1,
            AMODER::RANGE => 2,
            AMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AMODER {
        match value {
            0 => AMODER::MASK,
            1 => AMODER::_2_ADDRESSES,
            2 => AMODER::RANGE,
            i => AMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline]
    pub fn is_mask(&self) -> bool {
        *self == AMODER::MASK
    }
    #[doc = "Checks if the value of the field is `_2_ADDRESSES`"]
    #[inline]
    pub fn is_2_addresses(&self) -> bool {
        *self == AMODER::_2_ADDRESSES
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline]
    pub fn is_range(&self) -> bool {
        *self == AMODER::RANGE
    }
}
#[doc = r" Value of the field"]
pub struct RXENR {
    bits: bool,
}
impl RXENR {
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
#[doc = "Values that can be written to the field `CHSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSIZEW {
    #[doc = "8 bits"]
    _8_BIT,
    #[doc = "9 bits"]
    _9_BIT,
}
impl CHSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHSIZEW::_8_BIT => 0,
            CHSIZEW::_9_BIT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHSIZEW::_8_BIT)
    }
    #[doc = "9 bits"]
    #[inline]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(CHSIZEW::_9_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLOADENW<'a> {
    w: &'a mut W,
}
impl<'a> _PLOADENW<'a> {
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
pub struct _SSDEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSDEW<'a> {
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
#[doc = r" Proxy"]
pub struct _MSSENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSSENW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMODEW {
    #[doc = "SPI Address mask "]
    MASK,
    #[doc = "Two unique Addressess"]
    _2_ADDRESSES,
    #[doc = "Address Range"]
    RANGE,
}
impl AMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AMODEW::MASK => 0,
            AMODEW::_2_ADDRESSES => 1,
            AMODEW::RANGE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _AMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SPI Address mask"]
    #[inline]
    pub fn mask(self) -> &'a mut W {
        self.variant(AMODEW::MASK)
    }
    #[doc = "Two unique Addressess"]
    #[inline]
    pub fn _2_addresses(self) -> &'a mut W {
        self.variant(AMODEW::_2_ADDRESSES)
    }
    #[doc = "Address Range"]
    #[inline]
    pub fn range(self) -> &'a mut W {
        self.variant(AMODEW::RANGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Character Size"]
    #[inline]
    pub fn chsize(&self) -> CHSIZER {
        CHSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline]
    pub fn ploaden(&self) -> PLOADENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLOADENR { bits }
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline]
    pub fn ssde(&self) -> SSDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SSDER { bits }
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline]
    pub fn mssen(&self) -> MSSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSSENR { bits }
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline]
    pub fn amode(&self) -> AMODER {
        AMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline]
    pub fn rxen(&self) -> RXENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXENR { bits }
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
    #[doc = "Bits 0:2 - Character Size"]
    #[inline]
    pub fn chsize(&mut self) -> _CHSIZEW {
        _CHSIZEW { w: self }
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline]
    pub fn ploaden(&mut self) -> _PLOADENW {
        _PLOADENW { w: self }
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline]
    pub fn ssde(&mut self) -> _SSDEW {
        _SSDEW { w: self }
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline]
    pub fn mssen(&mut self) -> _MSSENW {
        _MSSENW { w: self }
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline]
    pub fn amode(&mut self) -> _AMODEW {
        _AMODEW { w: self }
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline]
    pub fn rxen(&mut self) -> _RXENW {
        _RXENW { w: self }
    }
}
