#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLC {
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
pub struct ICSPACER {
    bits: u8,
}
impl ICSPACER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DATA32B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA32BR {
    #[doc = "Transaction from and to DATA register are 8-bit"]
    DATA_TRANS_8BIT,
    #[doc = "Transaction from and to DATA register are 32-bit"]
    DATA_TRANS_32BIT,
}
impl DATA32BR {
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
            DATA32BR::DATA_TRANS_8BIT => false,
            DATA32BR::DATA_TRANS_32BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA32BR {
        match value {
            false => DATA32BR::DATA_TRANS_8BIT,
            true => DATA32BR::DATA_TRANS_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_TRANS_8BIT`"]
    #[inline]
    pub fn is_data_trans_8bit(&self) -> bool {
        *self == DATA32BR::DATA_TRANS_8BIT
    }
    #[doc = "Checks if the value of the field is `DATA_TRANS_32BIT`"]
    #[inline]
    pub fn is_data_trans_32bit(&self) -> bool {
        *self == DATA32BR::DATA_TRANS_32BIT
    }
}
#[doc = r" Proxy"]
pub struct _ICSPACEW<'a> {
    w: &'a mut W,
}
impl<'a> _ICSPACEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA32B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA32BW {
    #[doc = "Transaction from and to DATA register are 8-bit"]
    DATA_TRANS_8BIT,
    #[doc = "Transaction from and to DATA register are 32-bit"]
    DATA_TRANS_32BIT,
}
impl DATA32BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA32BW::DATA_TRANS_8BIT => false,
            DATA32BW::DATA_TRANS_32BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA32BW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA32BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA32BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transaction from and to DATA register are 8-bit"]
    #[inline]
    pub fn data_trans_8bit(self) -> &'a mut W {
        self.variant(DATA32BW::DATA_TRANS_8BIT)
    }
    #[doc = "Transaction from and to DATA register are 32-bit"]
    #[inline]
    pub fn data_trans_32bit(self) -> &'a mut W {
        self.variant(DATA32BW::DATA_TRANS_32BIT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline]
    pub fn icspace(&self) -> ICSPACER {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ICSPACER { bits }
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline]
    pub fn data32b(&self) -> DATA32BR {
        DATA32BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline]
    pub fn icspace(&mut self) -> _ICSPACEW {
        _ICSPACEW { w: self }
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline]
    pub fn data32b(&mut self) -> _DATA32BW {
        _DATA32BW { w: self }
    }
}