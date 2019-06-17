#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct READYR {
    bits: bool,
}
impl READYR {
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
pub struct PRMR {
    bits: bool,
}
impl PRMR {
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
pub struct LOADR {
    bits: bool,
}
impl LOADR {
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
pub struct SUSPR {
    bits: bool,
}
impl SUSPR {
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
pub struct AFIRSTR {
    bits: bool,
}
impl AFIRSTR {
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
pub struct BPDISR {
    bits: bool,
}
impl BPDISR {
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
#[doc = "Possible values of the field `BOOTPROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTPROTR {
    #[doc = "0 kbytes"]
    _0,
    #[doc = "8 kbytes"]
    _8,
    #[doc = "16 kbytes"]
    _16,
    #[doc = "24 kbytes"]
    _24,
    #[doc = "32 kbytes"]
    _32,
    #[doc = "40 kbytes"]
    _40,
    #[doc = "48 kbytes"]
    _48,
    #[doc = "56 kbytes"]
    _56,
    #[doc = "64 kbytes"]
    _64,
    #[doc = "72 kbytes"]
    _72,
    #[doc = "80 kbytes"]
    _80,
    #[doc = "88 kbytes"]
    _88,
    #[doc = "96 kbytes"]
    _96,
    #[doc = "104 kbytes"]
    _104,
    #[doc = "112 kbytes"]
    _112,
    #[doc = "120 kbytes"]
    _120,
}
impl BOOTPROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BOOTPROTR::_0 => 15,
            BOOTPROTR::_8 => 14,
            BOOTPROTR::_16 => 13,
            BOOTPROTR::_24 => 12,
            BOOTPROTR::_32 => 11,
            BOOTPROTR::_40 => 10,
            BOOTPROTR::_48 => 9,
            BOOTPROTR::_56 => 8,
            BOOTPROTR::_64 => 7,
            BOOTPROTR::_72 => 6,
            BOOTPROTR::_80 => 5,
            BOOTPROTR::_88 => 4,
            BOOTPROTR::_96 => 3,
            BOOTPROTR::_104 => 2,
            BOOTPROTR::_112 => 1,
            BOOTPROTR::_120 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BOOTPROTR {
        match value {
            15 => BOOTPROTR::_0,
            14 => BOOTPROTR::_8,
            13 => BOOTPROTR::_16,
            12 => BOOTPROTR::_24,
            11 => BOOTPROTR::_32,
            10 => BOOTPROTR::_40,
            9 => BOOTPROTR::_48,
            8 => BOOTPROTR::_56,
            7 => BOOTPROTR::_64,
            6 => BOOTPROTR::_72,
            5 => BOOTPROTR::_80,
            4 => BOOTPROTR::_88,
            3 => BOOTPROTR::_96,
            2 => BOOTPROTR::_104,
            1 => BOOTPROTR::_112,
            0 => BOOTPROTR::_120,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOOTPROTR::_0
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == BOOTPROTR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == BOOTPROTR::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == BOOTPROTR::_24
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == BOOTPROTR::_32
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline]
    pub fn is_40(&self) -> bool {
        *self == BOOTPROTR::_40
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline]
    pub fn is_48(&self) -> bool {
        *self == BOOTPROTR::_48
    }
    #[doc = "Checks if the value of the field is `_56`"]
    #[inline]
    pub fn is_56(&self) -> bool {
        *self == BOOTPROTR::_56
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == BOOTPROTR::_64
    }
    #[doc = "Checks if the value of the field is `_72`"]
    #[inline]
    pub fn is_72(&self) -> bool {
        *self == BOOTPROTR::_72
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline]
    pub fn is_80(&self) -> bool {
        *self == BOOTPROTR::_80
    }
    #[doc = "Checks if the value of the field is `_88`"]
    #[inline]
    pub fn is_88(&self) -> bool {
        *self == BOOTPROTR::_88
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline]
    pub fn is_96(&self) -> bool {
        *self == BOOTPROTR::_96
    }
    #[doc = "Checks if the value of the field is `_104`"]
    #[inline]
    pub fn is_104(&self) -> bool {
        *self == BOOTPROTR::_104
    }
    #[doc = "Checks if the value of the field is `_112`"]
    #[inline]
    pub fn is_112(&self) -> bool {
        *self == BOOTPROTR::_112
    }
    #[doc = "Checks if the value of the field is `_120`"]
    #[inline]
    pub fn is_120(&self) -> bool {
        *self == BOOTPROTR::_120
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Ready to accept a command"]
    #[inline]
    pub fn ready(&self) -> READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        READYR { bits }
    }
    #[doc = "Bit 1 - Power Reduction Mode"]
    #[inline]
    pub fn prm(&self) -> PRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PRMR { bits }
    }
    #[doc = "Bit 2 - NVM Page Buffer Active Loading"]
    #[inline]
    pub fn load(&self) -> LOADR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        LOADR { bits }
    }
    #[doc = "Bit 3 - NVM Write Or Erase Operation Is Suspended"]
    #[inline]
    pub fn susp(&self) -> SUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SUSPR { bits }
    }
    #[doc = "Bit 4 - BANKA First"]
    #[inline]
    pub fn afirst(&self) -> AFIRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        AFIRSTR { bits }
    }
    #[doc = "Bit 5 - Boot Loader Protection Disable"]
    #[inline]
    pub fn bpdis(&self) -> BPDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BPDISR { bits }
    }
    #[doc = "Bits 8:11 - Boot Loader Protection Size"]
    #[inline]
    pub fn bootprot(&self) -> BOOTPROTR {
        BOOTPROTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
