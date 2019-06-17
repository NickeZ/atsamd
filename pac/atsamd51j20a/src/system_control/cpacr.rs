#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPACR {
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
#[doc = "Possible values of the field `CP10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP10R {
    #[doc = "Access denied"]
    DENIED,
    #[doc = "Privileged access only"]
    PRIV,
    #[doc = "Full access"]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP10R::DENIED => 0,
            CP10R::PRIV => 1,
            CP10R::FULL => 3,
            CP10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP10R {
        match value {
            0 => CP10R::DENIED,
            1 => CP10R::PRIV,
            3 => CP10R::FULL,
            i => CP10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DENIED`"]
    #[inline]
    pub fn is_denied(&self) -> bool {
        *self == CP10R::DENIED
    }
    #[doc = "Checks if the value of the field is `PRIV`"]
    #[inline]
    pub fn is_priv_(&self) -> bool {
        *self == CP10R::PRIV
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == CP10R::FULL
    }
}
#[doc = "Possible values of the field `CP11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP11R {
    #[doc = "Access denied"]
    DENIED,
    #[doc = "Privileged access only"]
    PRIV,
    #[doc = "Full access"]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP11R::DENIED => 0,
            CP11R::PRIV => 1,
            CP11R::FULL => 3,
            CP11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP11R {
        match value {
            0 => CP11R::DENIED,
            1 => CP11R::PRIV,
            3 => CP11R::FULL,
            i => CP11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DENIED`"]
    #[inline]
    pub fn is_denied(&self) -> bool {
        *self == CP11R::DENIED
    }
    #[doc = "Checks if the value of the field is `PRIV`"]
    #[inline]
    pub fn is_priv_(&self) -> bool {
        *self == CP11R::PRIV
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == CP11R::FULL
    }
}
#[doc = "Values that can be written to the field `CP10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP10W {
    #[doc = "Access denied"]
    DENIED,
    #[doc = "Privileged access only"]
    PRIV,
    #[doc = "Full access"]
    FULL,
}
impl CP10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP10W::DENIED => 0,
            CP10W::PRIV => 1,
            CP10W::FULL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP10W<'a> {
    w: &'a mut W,
}
impl<'a> _CP10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied"]
    #[inline]
    pub fn denied(self) -> &'a mut W {
        self.variant(CP10W::DENIED)
    }
    #[doc = "Privileged access only"]
    #[inline]
    pub fn priv_(self) -> &'a mut W {
        self.variant(CP10W::PRIV)
    }
    #[doc = "Full access"]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(CP10W::FULL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP11W {
    #[doc = "Access denied"]
    DENIED,
    #[doc = "Privileged access only"]
    PRIV,
    #[doc = "Full access"]
    FULL,
}
impl CP11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP11W::DENIED => 0,
            CP11W::PRIV => 1,
            CP11W::FULL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP11W<'a> {
    w: &'a mut W,
}
impl<'a> _CP11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied"]
    #[inline]
    pub fn denied(self) -> &'a mut W {
        self.variant(CP11W::DENIED)
    }
    #[doc = "Privileged access only"]
    #[inline]
    pub fn priv_(self) -> &'a mut W {
        self.variant(CP11W::PRIV)
    }
    #[doc = "Full access"]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(CP11W::FULL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline]
    pub fn cp10(&self) -> CP10R {
        CP10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline]
    pub fn cp11(&self) -> CP11R {
        CP11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline]
    pub fn cp10(&mut self) -> _CP10W {
        _CP10W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline]
    pub fn cp11(&mut self) -> _CP11W {
        _CP11W { w: self }
    }
}
