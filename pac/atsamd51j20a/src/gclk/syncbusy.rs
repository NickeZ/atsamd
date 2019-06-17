#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SYNCBUSY {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SWRSTR {
    bits: bool,
}
impl SWRSTR {
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
#[doc = "Possible values of the field `GENCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENCTRLR {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = "Generic clock generator 9"]
    GCLK9,
    #[doc = "Generic clock generator 10"]
    GCLK10,
    #[doc = "Generic clock generator 11"]
    GCLK11,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl GENCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            GENCTRLR::GCLK0 => 1,
            GENCTRLR::GCLK1 => 2,
            GENCTRLR::GCLK2 => 4,
            GENCTRLR::GCLK3 => 8,
            GENCTRLR::GCLK4 => 16,
            GENCTRLR::GCLK5 => 32,
            GENCTRLR::GCLK6 => 64,
            GENCTRLR::GCLK7 => 128,
            GENCTRLR::GCLK8 => 256,
            GENCTRLR::GCLK9 => 512,
            GENCTRLR::GCLK10 => 1024,
            GENCTRLR::GCLK11 => 2048,
            GENCTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> GENCTRLR {
        match value {
            1 => GENCTRLR::GCLK0,
            2 => GENCTRLR::GCLK1,
            4 => GENCTRLR::GCLK2,
            8 => GENCTRLR::GCLK3,
            16 => GENCTRLR::GCLK4,
            32 => GENCTRLR::GCLK5,
            64 => GENCTRLR::GCLK6,
            128 => GENCTRLR::GCLK7,
            256 => GENCTRLR::GCLK8,
            512 => GENCTRLR::GCLK9,
            1024 => GENCTRLR::GCLK10,
            2048 => GENCTRLR::GCLK11,
            i => GENCTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENCTRLR::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENCTRLR::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENCTRLR::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENCTRLR::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENCTRLR::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENCTRLR::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENCTRLR::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENCTRLR::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENCTRLR::GCLK8
    }
    #[doc = "Checks if the value of the field is `GCLK9`"]
    #[inline]
    pub fn is_gclk9(&self) -> bool {
        *self == GENCTRLR::GCLK9
    }
    #[doc = "Checks if the value of the field is `GCLK10`"]
    #[inline]
    pub fn is_gclk10(&self) -> bool {
        *self == GENCTRLR::GCLK10
    }
    #[doc = "Checks if the value of the field is `GCLK11`"]
    #[inline]
    pub fn is_gclk11(&self) -> bool {
        *self == GENCTRLR::GCLK11
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRSTR { bits }
    }
    #[doc = "Bits 2:13 - Generic Clock Generator Control n Synchronization Busy bits"]
    #[inline]
    pub fn genctrl(&self) -> GENCTRLR {
        GENCTRLR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
