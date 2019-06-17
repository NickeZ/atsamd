#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FIFO0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ETM0R {
    bits: u8,
}
impl ETM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETM1R {
    bits: u8,
}
impl ETM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETM2R {
    bits: u8,
}
impl ETM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETM_BYTECOUNTR {
    bits: u8,
}
impl ETM_BYTECOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETM_ATVALIDR {
    bits: bool,
}
impl ETM_ATVALIDR {
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
pub struct ITM_BYTECOUNTR {
    bits: u8,
}
impl ITM_BYTECOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ITM_ATVALIDR {
    bits: bool,
}
impl ITM_ATVALIDR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7"]
    #[inline]
    pub fn etm0(&self) -> ETM0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETM0R { bits }
    }
    #[doc = "Bits 8:15"]
    #[inline]
    pub fn etm1(&self) -> ETM1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETM1R { bits }
    }
    #[doc = "Bits 16:23"]
    #[inline]
    pub fn etm2(&self) -> ETM2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETM2R { bits }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn etm_bytecount(&self) -> ETM_BYTECOUNTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETM_BYTECOUNTR { bits }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn etm_atvalid(&self) -> ETM_ATVALIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETM_ATVALIDR { bits }
    }
    #[doc = "Bits 27:28"]
    #[inline]
    pub fn itm_bytecount(&self) -> ITM_BYTECOUNTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITM_BYTECOUNTR { bits }
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn itm_atvalid(&self) -> ITM_ATVALIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ITM_ATVALIDR { bits }
    }
}
