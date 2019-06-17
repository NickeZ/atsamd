#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLA {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "USART with external clock"]
    USART_EXT_CLK,
    #[doc = "USART with internal clock"]
    USART_INT_CLK,
    #[doc = "SPI in slave operation"]
    SPI_SLAVE,
    #[doc = "SPI in master operation"]
    SPI_MASTER,
    #[doc = "I2C slave operation"]
    I2C_SLAVE,
    #[doc = "I2C master operation"]
    I2C_MASTER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::USART_EXT_CLK => 0,
            MODER::USART_INT_CLK => 1,
            MODER::SPI_SLAVE => 2,
            MODER::SPI_MASTER => 3,
            MODER::I2C_SLAVE => 4,
            MODER::I2C_MASTER => 5,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::USART_EXT_CLK,
            1 => MODER::USART_INT_CLK,
            2 => MODER::SPI_SLAVE,
            3 => MODER::SPI_MASTER,
            4 => MODER::I2C_SLAVE,
            5 => MODER::I2C_MASTER,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODER::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODER::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODER::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline]
    pub fn is_spi_master(&self) -> bool {
        *self == MODER::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODER::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODER::I2C_MASTER
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
pub struct PINOUTR {
    bits: bool,
}
impl PINOUTR {
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
#[doc = "Possible values of the field `SDAHOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDAHOLDR {
    #[doc = "Disabled"]
    DISABLE,
    #[doc = "50-100ns hold time"]
    _75NS,
    #[doc = "300-600ns hold time"]
    _450NS,
    #[doc = "400-800ns hold time"]
    _600NS,
}
impl SDAHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDAHOLDR::DISABLE => 0,
            SDAHOLDR::_75NS => 1,
            SDAHOLDR::_450NS => 2,
            SDAHOLDR::_600NS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDAHOLDR {
        match value {
            0 => SDAHOLDR::DISABLE,
            1 => SDAHOLDR::_75NS,
            2 => SDAHOLDR::_450NS,
            3 => SDAHOLDR::_600NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SDAHOLDR::DISABLE
    }
    #[doc = "Checks if the value of the field is `_75NS`"]
    #[inline]
    pub fn is_75ns(&self) -> bool {
        *self == SDAHOLDR::_75NS
    }
    #[doc = "Checks if the value of the field is `_450NS`"]
    #[inline]
    pub fn is_450ns(&self) -> bool {
        *self == SDAHOLDR::_450NS
    }
    #[doc = "Checks if the value of the field is `_600NS`"]
    #[inline]
    pub fn is_600ns(&self) -> bool {
        *self == SDAHOLDR::_600NS
    }
}
#[doc = r" Value of the field"]
pub struct MEXTTOENR {
    bits: bool,
}
impl MEXTTOENR {
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
pub struct SEXTTOENR {
    bits: bool,
}
impl SEXTTOENR {
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
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "Standard Mode(Sm) Upto 100kHz and Fast Mode(Fm) Upto 400kHz "]
    STANDARD_AND_FAST_MODE,
    #[doc = "Fast-mode Plus Upto 1MHz"]
    FASTPLUS_MODE,
    #[doc = "High-speed mode Upto 3.4MHz"]
    HIGH_SPEED_MODE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::STANDARD_AND_FAST_MODE => 0,
            SPEEDR::FASTPLUS_MODE => 1,
            SPEEDR::HIGH_SPEED_MODE => 2,
            SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::STANDARD_AND_FAST_MODE,
            1 => SPEEDR::FASTPLUS_MODE,
            2 => SPEEDR::HIGH_SPEED_MODE,
            i => SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_AND_FAST_MODE`"]
    #[inline]
    pub fn is_standard_and_fast_mode(&self) -> bool {
        *self == SPEEDR::STANDARD_AND_FAST_MODE
    }
    #[doc = "Checks if the value of the field is `FASTPLUS_MODE`"]
    #[inline]
    pub fn is_fastplus_mode(&self) -> bool {
        *self == SPEEDR::FASTPLUS_MODE
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED_MODE`"]
    #[inline]
    pub fn is_high_speed_mode(&self) -> bool {
        *self == SPEEDR::HIGH_SPEED_MODE
    }
}
#[doc = r" Value of the field"]
pub struct SCLSMR {
    bits: bool,
}
impl SCLSMR {
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
#[doc = "Possible values of the field `INACTOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INACTOUTR {
    #[doc = "Disabled"]
    DISABLE,
    #[doc = "5-6 SCL Time-Out(50-60us)"]
    _55US,
    #[doc = "10-11 SCL Time-Out(100-110us)"]
    _105US,
    #[doc = "20-21 SCL Time-Out(200-210us)"]
    _205US,
}
impl INACTOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INACTOUTR::DISABLE => 0,
            INACTOUTR::_55US => 1,
            INACTOUTR::_105US => 2,
            INACTOUTR::_205US => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INACTOUTR {
        match value {
            0 => INACTOUTR::DISABLE,
            1 => INACTOUTR::_55US,
            2 => INACTOUTR::_105US,
            3 => INACTOUTR::_205US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == INACTOUTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `_55US`"]
    #[inline]
    pub fn is_55us(&self) -> bool {
        *self == INACTOUTR::_55US
    }
    #[doc = "Checks if the value of the field is `_105US`"]
    #[inline]
    pub fn is_105us(&self) -> bool {
        *self == INACTOUTR::_105US
    }
    #[doc = "Checks if the value of the field is `_205US`"]
    #[inline]
    pub fn is_205us(&self) -> bool {
        *self == INACTOUTR::_205US
    }
}
#[doc = r" Value of the field"]
pub struct LOWTOUTENR {
    bits: bool,
}
impl LOWTOUTENR {
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
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEW {
    #[doc = "USART with external clock"]
    USART_EXT_CLK,
    #[doc = "USART with internal clock"]
    USART_INT_CLK,
    #[doc = "SPI in slave operation"]
    SPI_SLAVE,
    #[doc = "SPI in master operation"]
    SPI_MASTER,
    #[doc = "I2C slave operation"]
    I2C_SLAVE,
    #[doc = "I2C master operation"]
    I2C_MASTER,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::USART_EXT_CLK => 0,
            MODEW::USART_INT_CLK => 1,
            MODEW::SPI_SLAVE => 2,
            MODEW::SPI_MASTER => 3,
            MODEW::I2C_SLAVE => 4,
            MODEW::I2C_MASTER => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "USART with external clock"]
    #[inline]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODEW::USART_EXT_CLK)
    }
    #[doc = "USART with internal clock"]
    #[inline]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODEW::USART_INT_CLK)
    }
    #[doc = "SPI in slave operation"]
    #[inline]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODEW::SPI_SLAVE)
    }
    #[doc = "SPI in master operation"]
    #[inline]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODEW::SPI_MASTER)
    }
    #[doc = "I2C slave operation"]
    #[inline]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODEW::I2C_SLAVE)
    }
    #[doc = "I2C master operation"]
    #[inline]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODEW::I2C_MASTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _PINOUTW<'a> {
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
#[doc = "Values that can be written to the field `SDAHOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDAHOLDW {
    #[doc = "Disabled"]
    DISABLE,
    #[doc = "50-100ns hold time"]
    _75NS,
    #[doc = "300-600ns hold time"]
    _450NS,
    #[doc = "400-800ns hold time"]
    _600NS,
}
impl SDAHOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDAHOLDW::DISABLE => 0,
            SDAHOLDW::_75NS => 1,
            SDAHOLDW::_450NS => 2,
            SDAHOLDW::_600NS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDAHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _SDAHOLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDAHOLDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDAHOLDW::DISABLE)
    }
    #[doc = "50-100ns hold time"]
    #[inline]
    pub fn _75ns(self) -> &'a mut W {
        self.variant(SDAHOLDW::_75NS)
    }
    #[doc = "300-600ns hold time"]
    #[inline]
    pub fn _450ns(self) -> &'a mut W {
        self.variant(SDAHOLDW::_450NS)
    }
    #[doc = "400-800ns hold time"]
    #[inline]
    pub fn _600ns(self) -> &'a mut W {
        self.variant(SDAHOLDW::_600NS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEXTTOENW<'a> {
    w: &'a mut W,
}
impl<'a> _MEXTTOENW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEXTTOENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEXTTOENW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDW {
    #[doc = "Standard Mode(Sm) Upto 100kHz and Fast Mode(Fm) Upto 400kHz "]
    STANDARD_AND_FAST_MODE,
    #[doc = "Fast-mode Plus Upto 1MHz"]
    FASTPLUS_MODE,
    #[doc = "High-speed mode Upto 3.4MHz"]
    HIGH_SPEED_MODE,
}
impl SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPEEDW::STANDARD_AND_FAST_MODE => 0,
            SPEEDW::FASTPLUS_MODE => 1,
            SPEEDW::HIGH_SPEED_MODE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPEEDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard Mode(Sm) Upto 100kHz and Fast Mode(Fm) Upto 400kHz"]
    #[inline]
    pub fn standard_and_fast_mode(self) -> &'a mut W {
        self.variant(SPEEDW::STANDARD_AND_FAST_MODE)
    }
    #[doc = "Fast-mode Plus Upto 1MHz"]
    #[inline]
    pub fn fastplus_mode(self) -> &'a mut W {
        self.variant(SPEEDW::FASTPLUS_MODE)
    }
    #[doc = "High-speed mode Upto 3.4MHz"]
    #[inline]
    pub fn high_speed_mode(self) -> &'a mut W {
        self.variant(SPEEDW::HIGH_SPEED_MODE)
    }
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
#[doc = r" Proxy"]
pub struct _SCLSMW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLSMW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INACTOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INACTOUTW {
    #[doc = "Disabled"]
    DISABLE,
    #[doc = "5-6 SCL Time-Out(50-60us)"]
    _55US,
    #[doc = "10-11 SCL Time-Out(100-110us)"]
    _105US,
    #[doc = "20-21 SCL Time-Out(200-210us)"]
    _205US,
}
impl INACTOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INACTOUTW::DISABLE => 0,
            INACTOUTW::_55US => 1,
            INACTOUTW::_105US => 2,
            INACTOUTW::_205US => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INACTOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _INACTOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INACTOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(INACTOUTW::DISABLE)
    }
    #[doc = "5-6 SCL Time-Out(50-60us)"]
    #[inline]
    pub fn _55us(self) -> &'a mut W {
        self.variant(INACTOUTW::_55US)
    }
    #[doc = "10-11 SCL Time-Out(100-110us)"]
    #[inline]
    pub fn _105us(self) -> &'a mut W {
        self.variant(INACTOUTW::_105US)
    }
    #[doc = "20-21 SCL Time-Out(200-210us)"]
    #[inline]
    pub fn _205us(self) -> &'a mut W {
        self.variant(INACTOUTW::_205US)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOWTOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWTOUTENW<'a> {
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRSTR { bits }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline]
    pub fn pinout(&self) -> PINOUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINOUTR { bits }
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline]
    pub fn sdahold(&self) -> SDAHOLDR {
        SDAHOLDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Master SCL Low Extend Timeout"]
    #[inline]
    pub fn mexttoen(&self) -> MEXTTOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MEXTTOENR { bits }
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline]
    pub fn sexttoen(&self) -> SEXTTOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEXTTOENR { bits }
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline]
    pub fn sclsm(&self) -> SCLSMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLSMR { bits }
    }
    #[doc = "Bits 28:29 - Inactive Time-Out"]
    #[inline]
    pub fn inactout(&self) -> INACTOUTR {
        INACTOUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline]
    pub fn lowtouten(&self) -> LOWTOUTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOWTOUTENR { bits }
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline]
    pub fn pinout(&mut self) -> _PINOUTW {
        _PINOUTW { w: self }
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline]
    pub fn sdahold(&mut self) -> _SDAHOLDW {
        _SDAHOLDW { w: self }
    }
    #[doc = "Bit 22 - Master SCL Low Extend Timeout"]
    #[inline]
    pub fn mexttoen(&mut self) -> _MEXTTOENW {
        _MEXTTOENW { w: self }
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline]
    pub fn sexttoen(&mut self) -> _SEXTTOENW {
        _SEXTTOENW { w: self }
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline]
    pub fn speed(&mut self) -> _SPEEDW {
        _SPEEDW { w: self }
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline]
    pub fn sclsm(&mut self) -> _SCLSMW {
        _SCLSMW { w: self }
    }
    #[doc = "Bits 28:29 - Inactive Time-Out"]
    #[inline]
    pub fn inactout(&mut self) -> _INACTOUTW {
        _INACTOUTW { w: self }
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline]
    pub fn lowtouten(&mut self) -> _LOWTOUTENW {
        _LOWTOUTENW { w: self }
    }
}
