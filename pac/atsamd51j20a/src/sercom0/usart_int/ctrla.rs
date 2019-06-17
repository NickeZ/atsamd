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
#[doc = r" Value of the field"]
pub struct TXINVR {
    bits: bool,
}
impl TXINVR {
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
pub struct RXINVR {
    bits: bool,
}
impl RXINVR {
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
#[doc = "Possible values of the field `SAMPR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPRR {
    #[doc = "16x over-sampling using arithmetic baudrate generation"]
    _16X_ARITHMETIC,
    #[doc = "16x over-sampling using fractional baudrate generation"]
    _16X_FRACTIONAL,
    #[doc = "8x over-sampling using arithmetic baudrate generation"]
    _8X_ARITHMETIC,
    #[doc = "8x over-sampling using fractional baudrate generation"]
    _8X_FRACTIONAL,
    #[doc = "3x over-sampling using arithmetic baudrate generation"]
    _3X_ARITHMETIC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAMPRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAMPRR::_16X_ARITHMETIC => 0,
            SAMPRR::_16X_FRACTIONAL => 1,
            SAMPRR::_8X_ARITHMETIC => 2,
            SAMPRR::_8X_FRACTIONAL => 3,
            SAMPRR::_3X_ARITHMETIC => 4,
            SAMPRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAMPRR {
        match value {
            0 => SAMPRR::_16X_ARITHMETIC,
            1 => SAMPRR::_16X_FRACTIONAL,
            2 => SAMPRR::_8X_ARITHMETIC,
            3 => SAMPRR::_8X_FRACTIONAL,
            4 => SAMPRR::_3X_ARITHMETIC,
            i => SAMPRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16X_ARITHMETIC`"]
    #[inline]
    pub fn is_16x_arithmetic(&self) -> bool {
        *self == SAMPRR::_16X_ARITHMETIC
    }
    #[doc = "Checks if the value of the field is `_16X_FRACTIONAL`"]
    #[inline]
    pub fn is_16x_fractional(&self) -> bool {
        *self == SAMPRR::_16X_FRACTIONAL
    }
    #[doc = "Checks if the value of the field is `_8X_ARITHMETIC`"]
    #[inline]
    pub fn is_8x_arithmetic(&self) -> bool {
        *self == SAMPRR::_8X_ARITHMETIC
    }
    #[doc = "Checks if the value of the field is `_8X_FRACTIONAL`"]
    #[inline]
    pub fn is_8x_fractional(&self) -> bool {
        *self == SAMPRR::_8X_FRACTIONAL
    }
    #[doc = "Checks if the value of the field is `_3X_ARITHMETIC`"]
    #[inline]
    pub fn is_3x_arithmetic(&self) -> bool {
        *self == SAMPRR::_3X_ARITHMETIC
    }
}
#[doc = "Possible values of the field `TXPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPOR {
    #[doc = "SERCOM PAD\\[0\\] is used for data transmission"]
    PAD0,
    #[doc = "SERCOM_PAD\\[0\\] is used for data transmission"]
    PAD3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXPOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXPOR::PAD0 => 0,
            TXPOR::PAD3 => 3,
            TXPOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXPOR {
        match value {
            0 => TXPOR::PAD0,
            3 => TXPOR::PAD3,
            i => TXPOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline]
    pub fn is_pad0(&self) -> bool {
        *self == TXPOR::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline]
    pub fn is_pad3(&self) -> bool {
        *self == TXPOR::PAD3
    }
}
#[doc = "Possible values of the field `RXPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPOR {
    #[doc = "SERCOM PAD\\[0\\] is used for data reception"]
    PAD0,
    #[doc = "SERCOM PAD\\[1\\] is used for data reception"]
    PAD1,
    #[doc = "SERCOM PAD\\[2\\] is used for data reception"]
    PAD2,
    #[doc = "SERCOM PAD\\[3\\] is used for data reception"]
    PAD3,
}
impl RXPOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXPOR::PAD0 => 0,
            RXPOR::PAD1 => 1,
            RXPOR::PAD2 => 2,
            RXPOR::PAD3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXPOR {
        match value {
            0 => RXPOR::PAD0,
            1 => RXPOR::PAD1,
            2 => RXPOR::PAD2,
            3 => RXPOR::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline]
    pub fn is_pad0(&self) -> bool {
        *self == RXPOR::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline]
    pub fn is_pad1(&self) -> bool {
        *self == RXPOR::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline]
    pub fn is_pad2(&self) -> bool {
        *self == RXPOR::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline]
    pub fn is_pad3(&self) -> bool {
        *self == RXPOR::PAD3
    }
}
#[doc = r" Value of the field"]
pub struct SAMPAR {
    bits: u8,
}
impl SAMPAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMR {
    #[doc = "USART frame"]
    USART_FRAME_NO_PARITY,
    #[doc = "USART frame with parity"]
    USART_FRAME_WITH_PARITY,
    #[doc = "LIN Master - Break and sync generation"]
    USART_FRAME_LIN_MASTER_MODE,
    #[doc = "Auto-baud - break detection and auto-baud"]
    USART_FRAME_AUTO_BAUD_NO_PARITY,
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    USART_FRAME_AUTO_BAUD_WITH_PARITY,
    #[doc = "ISO 7816"]
    USART_FRAME_ISO_7816,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FORMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORMR::USART_FRAME_NO_PARITY => 0,
            FORMR::USART_FRAME_WITH_PARITY => 1,
            FORMR::USART_FRAME_LIN_MASTER_MODE => 2,
            FORMR::USART_FRAME_AUTO_BAUD_NO_PARITY => 4,
            FORMR::USART_FRAME_AUTO_BAUD_WITH_PARITY => 5,
            FORMR::USART_FRAME_ISO_7816 => 7,
            FORMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORMR {
        match value {
            0 => FORMR::USART_FRAME_NO_PARITY,
            1 => FORMR::USART_FRAME_WITH_PARITY,
            2 => FORMR::USART_FRAME_LIN_MASTER_MODE,
            4 => FORMR::USART_FRAME_AUTO_BAUD_NO_PARITY,
            5 => FORMR::USART_FRAME_AUTO_BAUD_WITH_PARITY,
            7 => FORMR::USART_FRAME_ISO_7816,
            i => FORMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_NO_PARITY`"]
    #[inline]
    pub fn is_usart_frame_no_parity(&self) -> bool {
        *self == FORMR::USART_FRAME_NO_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_WITH_PARITY`"]
    #[inline]
    pub fn is_usart_frame_with_parity(&self) -> bool {
        *self == FORMR::USART_FRAME_WITH_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_LIN_MASTER_MODE`"]
    #[inline]
    pub fn is_usart_frame_lin_master_mode(&self) -> bool {
        *self == FORMR::USART_FRAME_LIN_MASTER_MODE
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_AUTO_BAUD_NO_PARITY`"]
    #[inline]
    pub fn is_usart_frame_auto_baud_no_parity(&self) -> bool {
        *self == FORMR::USART_FRAME_AUTO_BAUD_NO_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_AUTO_BAUD_WITH_PARITY`"]
    #[inline]
    pub fn is_usart_frame_auto_baud_with_parity(&self) -> bool {
        *self == FORMR::USART_FRAME_AUTO_BAUD_WITH_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_ISO_7816`"]
    #[inline]
    pub fn is_usart_frame_iso_7816(&self) -> bool {
        *self == FORMR::USART_FRAME_ISO_7816
    }
}
#[doc = "Possible values of the field `CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMODER {
    #[doc = "Asynchronous Communication"]
    ASYNC,
    #[doc = "Synchronous Communication"]
    SYNC,
}
impl CMODER {
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
            CMODER::ASYNC => false,
            CMODER::SYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMODER {
        match value {
            false => CMODER::ASYNC,
            true => CMODER::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline]
    pub fn is_async(&self) -> bool {
        *self == CMODER::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline]
    pub fn is_sync(&self) -> bool {
        *self == CMODER::SYNC
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    IDLE_LOW,
    #[doc = "TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
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
    #[doc = "MSB is transmitted first"]
    MSB,
    #[doc = "LSB is transmitted first"]
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
#[doc = r" Proxy"]
pub struct _TXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINVW<'a> {
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
pub struct _RXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXINVW<'a> {
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
#[doc = "Values that can be written to the field `SAMPR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPRW {
    #[doc = "16x over-sampling using arithmetic baudrate generation"]
    _16X_ARITHMETIC,
    #[doc = "16x over-sampling using fractional baudrate generation"]
    _16X_FRACTIONAL,
    #[doc = "8x over-sampling using arithmetic baudrate generation"]
    _8X_ARITHMETIC,
    #[doc = "8x over-sampling using fractional baudrate generation"]
    _8X_FRACTIONAL,
    #[doc = "3x over-sampling using arithmetic baudrate generation"]
    _3X_ARITHMETIC,
}
impl SAMPRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMPRW::_16X_ARITHMETIC => 0,
            SAMPRW::_16X_FRACTIONAL => 1,
            SAMPRW::_8X_ARITHMETIC => 2,
            SAMPRW::_8X_FRACTIONAL => 3,
            SAMPRW::_3X_ARITHMETIC => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPRW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16x over-sampling using arithmetic baudrate generation"]
    #[inline]
    pub fn _16x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPRW::_16X_ARITHMETIC)
    }
    #[doc = "16x over-sampling using fractional baudrate generation"]
    #[inline]
    pub fn _16x_fractional(self) -> &'a mut W {
        self.variant(SAMPRW::_16X_FRACTIONAL)
    }
    #[doc = "8x over-sampling using arithmetic baudrate generation"]
    #[inline]
    pub fn _8x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPRW::_8X_ARITHMETIC)
    }
    #[doc = "8x over-sampling using fractional baudrate generation"]
    #[inline]
    pub fn _8x_fractional(self) -> &'a mut W {
        self.variant(SAMPRW::_8X_FRACTIONAL)
    }
    #[doc = "3x over-sampling using arithmetic baudrate generation"]
    #[inline]
    pub fn _3x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPRW::_3X_ARITHMETIC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPOW {
    #[doc = "SERCOM PAD\\[0\\] is used for data transmission"]
    PAD0,
    #[doc = "SERCOM_PAD\\[0\\] is used for data transmission"]
    PAD3,
}
impl TXPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXPOW::PAD0 => 0,
            TXPOW::PAD3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPOW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SERCOM PAD\\[0\\] is used for data transmission"]
    #[inline]
    pub fn pad0(self) -> &'a mut W {
        self.variant(TXPOW::PAD0)
    }
    #[doc = "SERCOM_PAD\\[0\\] is used for data transmission"]
    #[inline]
    pub fn pad3(self) -> &'a mut W {
        self.variant(TXPOW::PAD3)
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
#[doc = "Values that can be written to the field `RXPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPOW {
    #[doc = "SERCOM PAD\\[0\\] is used for data reception"]
    PAD0,
    #[doc = "SERCOM PAD\\[1\\] is used for data reception"]
    PAD1,
    #[doc = "SERCOM PAD\\[2\\] is used for data reception"]
    PAD2,
    #[doc = "SERCOM PAD\\[3\\] is used for data reception"]
    PAD3,
}
impl RXPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXPOW::PAD0 => 0,
            RXPOW::PAD1 => 1,
            RXPOW::PAD2 => 2,
            RXPOW::PAD3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPOW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SERCOM PAD\\[0\\] is used for data reception"]
    #[inline]
    pub fn pad0(self) -> &'a mut W {
        self.variant(RXPOW::PAD0)
    }
    #[doc = "SERCOM PAD\\[1\\] is used for data reception"]
    #[inline]
    pub fn pad1(self) -> &'a mut W {
        self.variant(RXPOW::PAD1)
    }
    #[doc = "SERCOM PAD\\[2\\] is used for data reception"]
    #[inline]
    pub fn pad2(self) -> &'a mut W {
        self.variant(RXPOW::PAD2)
    }
    #[doc = "SERCOM PAD\\[3\\] is used for data reception"]
    #[inline]
    pub fn pad3(self) -> &'a mut W {
        self.variant(RXPOW::PAD3)
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
pub struct _SAMPAW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPAW<'a> {
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
#[doc = "Values that can be written to the field `FORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMW {
    #[doc = "USART frame"]
    USART_FRAME_NO_PARITY,
    #[doc = "USART frame with parity"]
    USART_FRAME_WITH_PARITY,
    #[doc = "LIN Master - Break and sync generation"]
    USART_FRAME_LIN_MASTER_MODE,
    #[doc = "Auto-baud - break detection and auto-baud"]
    USART_FRAME_AUTO_BAUD_NO_PARITY,
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    USART_FRAME_AUTO_BAUD_WITH_PARITY,
    #[doc = "ISO 7816"]
    USART_FRAME_ISO_7816,
}
impl FORMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORMW::USART_FRAME_NO_PARITY => 0,
            FORMW::USART_FRAME_WITH_PARITY => 1,
            FORMW::USART_FRAME_LIN_MASTER_MODE => 2,
            FORMW::USART_FRAME_AUTO_BAUD_NO_PARITY => 4,
            FORMW::USART_FRAME_AUTO_BAUD_WITH_PARITY => 5,
            FORMW::USART_FRAME_ISO_7816 => 7,
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
    #[doc = "USART frame"]
    #[inline]
    pub fn usart_frame_no_parity(self) -> &'a mut W {
        self.variant(FORMW::USART_FRAME_NO_PARITY)
    }
    #[doc = "USART frame with parity"]
    #[inline]
    pub fn usart_frame_with_parity(self) -> &'a mut W {
        self.variant(FORMW::USART_FRAME_WITH_PARITY)
    }
    #[doc = "LIN Master - Break and sync generation"]
    #[inline]
    pub fn usart_frame_lin_master_mode(self) -> &'a mut W {
        self.variant(FORMW::USART_FRAME_LIN_MASTER_MODE)
    }
    #[doc = "Auto-baud - break detection and auto-baud"]
    #[inline]
    pub fn usart_frame_auto_baud_no_parity(self) -> &'a mut W {
        self.variant(FORMW::USART_FRAME_AUTO_BAUD_NO_PARITY)
    }
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    #[inline]
    pub fn usart_frame_auto_baud_with_parity(self) -> &'a mut W {
        self.variant(FORMW::USART_FRAME_AUTO_BAUD_WITH_PARITY)
    }
    #[doc = "ISO 7816"]
    #[inline]
    pub fn usart_frame_iso_7816(self) -> &'a mut W {
        self.variant(FORMW::USART_FRAME_ISO_7816)
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
#[doc = "Values that can be written to the field `CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMODEW {
    #[doc = "Asynchronous Communication"]
    ASYNC,
    #[doc = "Synchronous Communication"]
    SYNC,
}
impl CMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMODEW::ASYNC => false,
            CMODEW::SYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asynchronous Communication"]
    #[inline]
    pub fn async(self) -> &'a mut W {
        self.variant(CMODEW::ASYNC)
    }
    #[doc = "Synchronous Communication"]
    #[inline]
    pub fn sync(self) -> &'a mut W {
        self.variant(CMODEW::SYNC)
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
    #[doc = "TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    IDLE_LOW,
    #[doc = "TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
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
    #[doc = "TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    #[inline]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOLW::IDLE_LOW)
    }
    #[doc = "TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
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
    #[doc = "MSB is transmitted first"]
    MSB,
    #[doc = "LSB is transmitted first"]
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
    #[doc = "MSB is transmitted first"]
    #[inline]
    pub fn msb(self) -> &'a mut W {
        self.variant(DORDW::MSB)
    }
    #[doc = "LSB is transmitted first"]
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
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline]
    pub fn txinv(&self) -> TXINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXINVR { bits }
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline]
    pub fn rxinv(&self) -> RXINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXINVR { bits }
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline]
    pub fn sampr(&self) -> SAMPRR {
        SAMPRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline]
    pub fn txpo(&self) -> TXPOR {
        TXPOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline]
    pub fn rxpo(&self) -> RXPOR {
        RXPOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline]
    pub fn sampa(&self) -> SAMPAR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAMPAR { bits }
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
    #[doc = "Bit 28 - Communication Mode"]
    #[inline]
    pub fn cmode(&self) -> CMODER {
        CMODER::_from({
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
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline]
    pub fn txinv(&mut self) -> _TXINVW {
        _TXINVW { w: self }
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline]
    pub fn rxinv(&mut self) -> _RXINVW {
        _RXINVW { w: self }
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline]
    pub fn sampr(&mut self) -> _SAMPRW {
        _SAMPRW { w: self }
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline]
    pub fn txpo(&mut self) -> _TXPOW {
        _TXPOW { w: self }
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline]
    pub fn rxpo(&mut self) -> _RXPOW {
        _RXPOW { w: self }
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline]
    pub fn sampa(&mut self) -> _SAMPAW {
        _SAMPAW { w: self }
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline]
    pub fn form(&mut self) -> _FORMW {
        _FORMW { w: self }
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline]
    pub fn cmode(&mut self) -> _CMODEW {
        _CMODEW { w: self }
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
