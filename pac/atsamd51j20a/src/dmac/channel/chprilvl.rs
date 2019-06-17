#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CHPRILVL {
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
#[doc = "Possible values of the field `PRILVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRILVLR {
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    LVL0,
    #[doc = "Channel Priority Level 1"]
    LVL1,
    #[doc = "Channel Priority Level 2"]
    LVL2,
    #[doc = "Channel Priority Level 3 (Highest Level)"]
    LVL3,
}
impl PRILVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRILVLR::LVL0 => 0,
            PRILVLR::LVL1 => 1,
            PRILVLR::LVL2 => 2,
            PRILVLR::LVL3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRILVLR {
        match value {
            0 => PRILVLR::LVL0,
            1 => PRILVLR::LVL1,
            2 => PRILVLR::LVL2,
            3 => PRILVLR::LVL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL0`"]
    #[inline]
    pub fn is_lvl0(&self) -> bool {
        *self == PRILVLR::LVL0
    }
    #[doc = "Checks if the value of the field is `LVL1`"]
    #[inline]
    pub fn is_lvl1(&self) -> bool {
        *self == PRILVLR::LVL1
    }
    #[doc = "Checks if the value of the field is `LVL2`"]
    #[inline]
    pub fn is_lvl2(&self) -> bool {
        *self == PRILVLR::LVL2
    }
    #[doc = "Checks if the value of the field is `LVL3`"]
    #[inline]
    pub fn is_lvl3(&self) -> bool {
        *self == PRILVLR::LVL3
    }
}
#[doc = "Values that can be written to the field `PRILVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRILVLW {
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    LVL0,
    #[doc = "Channel Priority Level 1"]
    LVL1,
    #[doc = "Channel Priority Level 2"]
    LVL2,
    #[doc = "Channel Priority Level 3 (Highest Level)"]
    LVL3,
}
impl PRILVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRILVLW::LVL0 => 0,
            PRILVLW::LVL1 => 1,
            PRILVLW::LVL2 => 2,
            PRILVLW::LVL3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRILVLW<'a> {
    w: &'a mut W,
}
impl<'a> _PRILVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRILVLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    #[inline]
    pub fn lvl0(self) -> &'a mut W {
        self.variant(PRILVLW::LVL0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline]
    pub fn lvl1(self) -> &'a mut W {
        self.variant(PRILVLW::LVL1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline]
    pub fn lvl2(self) -> &'a mut W {
        self.variant(PRILVLW::LVL2)
    }
    #[doc = "Channel Priority Level 3 (Highest Level)"]
    #[inline]
    pub fn lvl3(self) -> &'a mut W {
        self.variant(PRILVLW::LVL3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline]
    pub fn prilvl(&self) -> PRILVLR {
        PRILVLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline]
    pub fn prilvl(&mut self) -> _PRILVLW {
        _PRILVLW { w: self }
    }
}
