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
pub struct IBONR {
    bits: bool,
}
impl IBONR {
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
#[doc = "Possible values of the field `DOPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOPOR {
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\] and SS on PAD\\[2\\]"]
    PAD0,
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\] and SS on PAD\\[2\\]"]
    PAD2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DOPOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DOPOR::PAD0 => 0,
            DOPOR::PAD2 => 2,
            DOPOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DOPOR {
        match value {
            0 => DOPOR::PAD0,
            2 => DOPOR::PAD2,
            i => DOPOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline]
    pub fn is_pad0(&self) -> bool {
        *self == DOPOR::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline]
    pub fn is_pad2(&self) -> bool {
        *self == DOPOR::PAD2
    }
}
#[doc = "Possible values of the field `DIPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIPOR {
    #[doc = "SERCOM PAD\\[0\\] is used as data input"]
    PAD0,
    #[doc = "SERCOM PAD\\[1\\] is used as data input"]
    PAD1,
    #[doc = "SERCOM PAD\\[2\\] is used as data input"]
    PAD2,
    #[doc = "SERCOM PAD\\[3\\] is used as data input"]
    PAD3,
}
impl DIPOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIPOR::PAD0 => 0,
            DIPOR::PAD1 => 1,
            DIPOR::PAD2 => 2,
            DIPOR::PAD3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIPOR {
        match value {
            0 => DIPOR::PAD0,
            1 => DIPOR::PAD1,
            2 => DIPOR::PAD2,
            3 => DIPOR::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline]
    pub fn is_pad0(&self) -> bool {
        *self == DIPOR::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline]
    pub fn is_pad1(&self) -> bool {
        *self == DIPOR::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline]
    pub fn is_pad2(&self) -> bool {
        *self == DIPOR::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline]
    pub fn is_pad3(&self) -> bool {
        *self == DIPOR::PAD3
    }
}
#[doc = "Possible values of the field `FORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMR {
    #[doc = "SPI Frame"]
    SPI_FRAME,
    #[doc = "SPI Frame with Addr"]
    SPI_FRAME_WITH_ADDR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FORMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORMR::SPI_FRAME => 0,
            FORMR::SPI_FRAME_WITH_ADDR => 2,
            FORMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORMR {
        match value {
            0 => FORMR::SPI_FRAME,
            2 => FORMR::SPI_FRAME_WITH_ADDR,
            i => FORMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI_FRAME`"]
    #[inline]
    pub fn is_spi_frame(&self) -> bool {
        *self == FORMR::SPI_FRAME
    }
    #[doc = "Checks if the value of the field is `SPI_FRAME_WITH_ADDR`"]
    #[inline]
    pub fn is_spi_frame_with_addr(&self) -> bool {
        *self == FORMR::SPI_FRAME_WITH_ADDR
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    LEADING_EDGE,
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    TRAILING_EDGE,
}
impl CPHAR {
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
            CPHAR::LEADING_EDGE => false,
            CPHAR::TRAILING_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::LEADING_EDGE,
            true => CPHAR::TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEADING_EDGE`"]
    #[inline]
    pub fn is_leading_edge(&self) -> bool {
        *self == CPHAR::LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `TRAILING_EDGE`"]
    #[inline]
    pub fn is_trailing_edge(&self) -> bool {
        *self == CPHAR::TRAILING_EDGE
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "SCK is low when idle"]
    IDLE_LOW,
    #[doc = "SCK is high when idle"]
    IDLE_HIGH,
}
impl CPOLR {
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
            CPOLR::IDLE_LOW => false,
            CPOLR::IDLE_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::IDLE_LOW,
            true => CPOLR::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOLR::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOLR::IDLE_HIGH
    }
}
#[doc = "Possible values of the field `DORD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DORDR {
    #[doc = "MSB is transferred first"]
    MSB,
    #[doc = "LSB is transferred first"]
    LSB,
}
impl DORDR {
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
            DORDR::MSB => false,
            DORDR::LSB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DORDR {
        match value {
            false => DORDR::MSB,
            true => DORDR::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline]
    pub fn is_msb(&self) -> bool {
        *self == DORDR::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline]
    pub fn is_lsb(&self) -> bool {
        *self == DORDR::LSB
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
pub struct _IBONW<'a> {
    w: &'a mut W,
}
impl<'a> _IBONW<'a> {
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
#[doc = "Values that can be written to the field `DOPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOPOW {
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\] and SS on PAD\\[2\\]"]
    PAD0,
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\] and SS on PAD\\[2\\]"]
    PAD2,
}
impl DOPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DOPOW::PAD0 => 0,
            DOPOW::PAD2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOPOW<'a> {
    w: &'a mut W,
}
impl<'a> _DOPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOPOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\] and SS on PAD\\[2\\]"]
    #[inline]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DOPOW::PAD0)
    }
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\] and SS on PAD\\[2\\]"]
    #[inline]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DOPOW::PAD2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIPOW {
    #[doc = "SERCOM PAD\\[0\\] is used as data input"]
    PAD0,
    #[doc = "SERCOM PAD\\[1\\] is used as data input"]
    PAD1,
    #[doc = "SERCOM PAD\\[2\\] is used as data input"]
    PAD2,
    #[doc = "SERCOM PAD\\[3\\] is used as data input"]
    PAD3,
}
impl DIPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIPOW::PAD0 => 0,
            DIPOW::PAD1 => 1,
            DIPOW::PAD2 => 2,
            DIPOW::PAD3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIPOW<'a> {
    w: &'a mut W,
}
impl<'a> _DIPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIPOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SERCOM PAD\\[0\\] is used as data input"]
    #[inline]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DIPOW::PAD0)
    }
    #[doc = "SERCOM PAD\\[1\\] is used as data input"]
    #[inline]
    pub fn pad1(self) -> &'a mut W {
        self.variant(DIPOW::PAD1)
    }
    #[doc = "SERCOM PAD\\[2\\] is used as data input"]
    #[inline]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DIPOW::PAD2)
    }
    #[doc = "SERCOM PAD\\[3\\] is used as data input"]
    #[inline]
    pub fn pad3(self) -> &'a mut W {
        self.variant(DIPOW::PAD3)
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
#[doc = "Values that can be written to the field `FORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMW {
    #[doc = "SPI Frame"]
    SPI_FRAME,
    #[doc = "SPI Frame with Addr"]
    SPI_FRAME_WITH_ADDR,
}
impl FORMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORMW::SPI_FRAME => 0,
            FORMW::SPI_FRAME_WITH_ADDR => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORMW<'a> {
    w: &'a mut W,
}
impl<'a> _FORMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SPI Frame"]
    #[inline]
    pub fn spi_frame(self) -> &'a mut W {
        self.variant(FORMW::SPI_FRAME)
    }
    #[doc = "SPI Frame with Addr"]
    #[inline]
    pub fn spi_frame_with_addr(self) -> &'a mut W {
        self.variant(FORMW::SPI_FRAME_WITH_ADDR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAW {
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    LEADING_EDGE,
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    TRAILING_EDGE,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::LEADING_EDGE => false,
            CPHAW::TRAILING_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline]
    pub fn leading_edge(self) -> &'a mut W {
        self.variant(CPHAW::LEADING_EDGE)
    }
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline]
    pub fn trailing_edge(self) -> &'a mut W {
        self.variant(CPHAW::TRAILING_EDGE)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLW {
    #[doc = "SCK is low when idle"]
    IDLE_LOW,
    #[doc = "SCK is high when idle"]
    IDLE_HIGH,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::IDLE_LOW => false,
            CPOLW::IDLE_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCK is low when idle"]
    #[inline]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOLW::IDLE_LOW)
    }
    #[doc = "SCK is high when idle"]
    #[inline]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOLW::IDLE_HIGH)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DORD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DORDW {
    #[doc = "MSB is transferred first"]
    MSB,
    #[doc = "LSB is transferred first"]
    LSB,
}
impl DORDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DORDW::MSB => false,
            DORDW::LSB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DORDW<'a> {
    w: &'a mut W,
}
impl<'a> _DORDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DORDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MSB is transferred first"]
    #[inline]
    pub fn msb(self) -> &'a mut W {
        self.variant(DORDW::MSB)
    }
    #[doc = "LSB is transferred first"]
    #[inline]
    pub fn lsb(self) -> &'a mut W {
        self.variant(DORDW::LSB)
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
    #[doc = "Bit 7 - Run during Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline]
    pub fn ibon(&self) -> IBONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IBONR { bits }
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline]
    pub fn dopo(&self) -> DOPOR {
        DOPOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline]
    pub fn dipo(&self) -> DIPOR {
        DIPOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline]
    pub fn form(&self) -> FORMR {
        FORMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline]
    pub fn dord(&self) -> DORDR {
        DORDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 7 - Run during Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline]
    pub fn ibon(&mut self) -> _IBONW {
        _IBONW { w: self }
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline]
    pub fn dopo(&mut self) -> _DOPOW {
        _DOPOW { w: self }
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline]
    pub fn dipo(&mut self) -> _DIPOW {
        _DIPOW { w: self }
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline]
    pub fn form(&mut self) -> _FORMW {
        _FORMW { w: self }
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline]
    pub fn dord(&mut self) -> _DORDW {
        _DORDW { w: self }
    }
}
