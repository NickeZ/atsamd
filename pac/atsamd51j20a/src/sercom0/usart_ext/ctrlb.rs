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
    #[doc = "8 Bits"]
    _8_BIT,
    #[doc = "9 Bits"]
    _9_BIT,
    #[doc = "5 Bits"]
    _5_BIT,
    #[doc = "6 Bits"]
    _6_BIT,
    #[doc = "7 Bits"]
    _7_BIT,
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
            CHSIZER::_5_BIT => 5,
            CHSIZER::_6_BIT => 6,
            CHSIZER::_7_BIT => 7,
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
            5 => CHSIZER::_5_BIT,
            6 => CHSIZER::_6_BIT,
            7 => CHSIZER::_7_BIT,
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
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline]
    pub fn is_5_bit(&self) -> bool {
        *self == CHSIZER::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline]
    pub fn is_6_bit(&self) -> bool {
        *self == CHSIZER::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline]
    pub fn is_7_bit(&self) -> bool {
        *self == CHSIZER::_7_BIT
    }
}
#[doc = "Possible values of the field `SBMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBMODER {
    #[doc = "One Stop Bit"]
    _1_BIT,
    #[doc = "Two Stop Bits"]
    _2_BIT,
}
impl SBMODER {
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
            SBMODER::_1_BIT => false,
            SBMODER::_2_BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBMODER {
        match value {
            false => SBMODER::_1_BIT,
            true => SBMODER::_2_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_1_BIT`"]
    #[inline]
    pub fn is_1_bit(&self) -> bool {
        *self == SBMODER::_1_BIT
    }
    #[doc = "Checks if the value of the field is `_2_BIT`"]
    #[inline]
    pub fn is_2_bit(&self) -> bool {
        *self == SBMODER::_2_BIT
    }
}
#[doc = r" Value of the field"]
pub struct COLDENR {
    bits: bool,
}
impl COLDENR {
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
pub struct SFDER {
    bits: bool,
}
impl SFDER {
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
pub struct ENCR {
    bits: bool,
}
impl ENCR {
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
#[doc = "Possible values of the field `PMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMODER {
    #[doc = "Even Parity"]
    EVEN,
    #[doc = "Odd Parity"]
    ODD,
}
impl PMODER {
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
            PMODER::EVEN => false,
            PMODER::ODD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMODER {
        match value {
            false => PMODER::EVEN,
            true => PMODER::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == PMODER::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == PMODER::ODD
    }
}
#[doc = r" Value of the field"]
pub struct TXENR {
    bits: bool,
}
impl TXENR {
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
#[doc = r" Value of the field"]
pub struct LINCMDR {
    bits: u8,
}
impl LINCMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CHSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSIZEW {
    #[doc = "8 Bits"]
    _8_BIT,
    #[doc = "9 Bits"]
    _9_BIT,
    #[doc = "5 Bits"]
    _5_BIT,
    #[doc = "6 Bits"]
    _6_BIT,
    #[doc = "7 Bits"]
    _7_BIT,
}
impl CHSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHSIZEW::_8_BIT => 0,
            CHSIZEW::_9_BIT => 1,
            CHSIZEW::_5_BIT => 5,
            CHSIZEW::_6_BIT => 6,
            CHSIZEW::_7_BIT => 7,
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
    #[doc = "8 Bits"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHSIZEW::_8_BIT)
    }
    #[doc = "9 Bits"]
    #[inline]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(CHSIZEW::_9_BIT)
    }
    #[doc = "5 Bits"]
    #[inline]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(CHSIZEW::_5_BIT)
    }
    #[doc = "6 Bits"]
    #[inline]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(CHSIZEW::_6_BIT)
    }
    #[doc = "7 Bits"]
    #[inline]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(CHSIZEW::_7_BIT)
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
#[doc = "Values that can be written to the field `SBMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBMODEW {
    #[doc = "One Stop Bit"]
    _1_BIT,
    #[doc = "Two Stop Bits"]
    _2_BIT,
}
impl SBMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBMODEW::_1_BIT => false,
            SBMODEW::_2_BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SBMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One Stop Bit"]
    #[inline]
    pub fn _1_bit(self) -> &'a mut W {
        self.variant(SBMODEW::_1_BIT)
    }
    #[doc = "Two Stop Bits"]
    #[inline]
    pub fn _2_bit(self) -> &'a mut W {
        self.variant(SBMODEW::_2_BIT)
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
#[doc = r" Proxy"]
pub struct _COLDENW<'a> {
    w: &'a mut W,
}
impl<'a> _COLDENW<'a> {
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
pub struct _SFDEW<'a> {
    w: &'a mut W,
}
impl<'a> _SFDEW<'a> {
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
pub struct _ENCW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCW<'a> {
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
#[doc = "Values that can be written to the field `PMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMODEW {
    #[doc = "Even Parity"]
    EVEN,
    #[doc = "Odd Parity"]
    ODD,
}
impl PMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMODEW::EVEN => false,
            PMODEW::ODD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even Parity"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(PMODEW::EVEN)
    }
    #[doc = "Odd Parity"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(PMODEW::ODD)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXENW<'a> {
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
#[doc = r" Proxy"]
pub struct _LINCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _LINCMDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:2 - Character Size"]
    #[inline]
    pub fn chsize(&self) -> CHSIZER {
        CHSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline]
    pub fn sbmode(&self) -> SBMODER {
        SBMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline]
    pub fn colden(&self) -> COLDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COLDENR { bits }
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline]
    pub fn sfde(&self) -> SFDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFDER { bits }
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline]
    pub fn enc(&self) -> ENCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENCR { bits }
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline]
    pub fn pmode(&self) -> PMODER {
        PMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline]
    pub fn txen(&self) -> TXENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXENR { bits }
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
    #[doc = "Bits 24:25 - LIN Command"]
    #[inline]
    pub fn lincmd(&self) -> LINCMDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LINCMDR { bits }
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
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline]
    pub fn sbmode(&mut self) -> _SBMODEW {
        _SBMODEW { w: self }
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline]
    pub fn colden(&mut self) -> _COLDENW {
        _COLDENW { w: self }
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline]
    pub fn sfde(&mut self) -> _SFDEW {
        _SFDEW { w: self }
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline]
    pub fn enc(&mut self) -> _ENCW {
        _ENCW { w: self }
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline]
    pub fn pmode(&mut self) -> _PMODEW {
        _PMODEW { w: self }
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline]
    pub fn txen(&mut self) -> _TXENW {
        _TXENW { w: self }
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline]
    pub fn rxen(&mut self) -> _RXENW {
        _RXENW { w: self }
    }
    #[doc = "Bits 24:25 - LIN Command"]
    #[inline]
    pub fn lincmd(&mut self) -> _LINCMDW {
        _LINCMDW { w: self }
    }
}
