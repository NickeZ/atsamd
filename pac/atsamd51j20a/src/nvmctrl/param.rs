#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PARAM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NVMPR {
    bits: u16,
}
impl NVMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `PSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSZR {
    #[doc = "8 bytes"]
    _8,
    #[doc = "16 bytes"]
    _16,
    #[doc = "32 bytes"]
    _32,
    #[doc = "64 bytes"]
    _64,
    #[doc = "128 bytes"]
    _128,
    #[doc = "256 bytes"]
    _256,
    #[doc = "512 bytes"]
    _512,
    #[doc = "1024 bytes"]
    _1024,
}
impl PSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSZR::_8 => 0,
            PSZR::_16 => 1,
            PSZR::_32 => 2,
            PSZR::_64 => 3,
            PSZR::_128 => 4,
            PSZR::_256 => 5,
            PSZR::_512 => 6,
            PSZR::_1024 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSZR {
        match value {
            0 => PSZR::_8,
            1 => PSZR::_16,
            2 => PSZR::_32,
            3 => PSZR::_64,
            4 => PSZR::_128,
            5 => PSZR::_256,
            6 => PSZR::_512,
            7 => PSZR::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PSZR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PSZR::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == PSZR::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == PSZR::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == PSZR::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == PSZR::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == PSZR::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == PSZR::_1024
    }
}
#[doc = "Possible values of the field `SEE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEER {
    #[doc = "163840 bytes"]
    A,
    #[doc = "147456 bytes"]
    _9,
    #[doc = "131072 bytes"]
    _8,
    #[doc = "114688 bytes"]
    _7,
    #[doc = "98304 bytes"]
    _6,
    #[doc = "81920 bytes"]
    _5,
    #[doc = "65536 bytes"]
    _4,
    #[doc = "49152 bytes"]
    _3,
    #[doc = "32768 bytes"]
    _2,
    #[doc = "16384 bytes"]
    _1,
    #[doc = "0 bytes"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SEER {
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
            SEER::A => true,
            SEER::_9 => true,
            SEER::_8 => true,
            SEER::_7 => true,
            SEER::_6 => true,
            SEER::_5 => true,
            SEER::_4 => true,
            SEER::_3 => true,
            SEER::_2 => true,
            SEER::_1 => true,
            SEER::_0 => false,
            SEER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEER {
        match value {
            true => SEER::A,
            //true => SEER::_9,
            //true => SEER::_8,
            //true => SEER::_7,
            //true => SEER::_6,
            //true => SEER::_5,
            //true => SEER::_4,
            //true => SEER::_3,
            //true => SEER::_2,
            //true => SEER::_1,
            //false => SEER::_0,
            i => SEER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline]
    pub fn is_a(&self) -> bool {
        *self == SEER::A
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline]
    pub fn is_9(&self) -> bool {
        *self == SEER::_9
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == SEER::_8
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == SEER::_7
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == SEER::_6
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == SEER::_5
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == SEER::_4
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SEER::_3
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SEER::_2
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEER::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEER::_0
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - NVM Pages"]
    #[inline]
    pub fn nvmp(&self) -> NVMPR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NVMPR { bits }
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline]
    pub fn psz(&self) -> PSZR {
        PSZR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - SmartEEPROM Supported"]
    #[inline]
    pub fn see(&self) -> SEER {
        SEER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
