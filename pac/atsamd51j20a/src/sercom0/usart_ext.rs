#[doc = "USART_EXT Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_EXT Control A"]
pub mod ctrla;
#[doc = "USART_EXT Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_EXT Control B"]
pub mod ctrlb;
#[doc = "USART_EXT Control C"]
pub struct CTRLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_EXT Control C"]
pub mod ctrlc;
#[doc = "USART_EXT Baud Rate"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART_EXT Baud Rate"]
pub mod baud;
#[doc = "USART_EXT Baud Rate"]
pub struct BAUD_FRAC_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART_EXT Baud Rate"]
pub mod baud_frac_mode;
#[doc = "USART_EXT Baud Rate"]
pub struct BAUD_FRACFP_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART_EXT Baud Rate"]
pub mod baud_fracfp_mode;
#[doc = "USART_EXT Baud Rate"]
pub struct BAUD_USARTFP_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART_EXT Baud Rate"]
pub mod baud_usartfp_mode;
#[doc = "USART_EXT Receive Pulse Length"]
pub struct RXPL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART_EXT Receive Pulse Length"]
pub mod rxpl;
#[doc = "USART_EXT Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART_EXT Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "USART_EXT Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART_EXT Interrupt Enable Set"]
pub mod intenset;
#[doc = "USART_EXT Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART_EXT Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "USART_EXT Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART_EXT Status"]
pub mod status;
#[doc = "USART_EXT Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_EXT Synchronization Busy"]
pub mod syncbusy;
#[doc = "USART_EXT Receive Error Count"]
pub struct RXERRCNT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART_EXT Receive Error Count"]
pub mod rxerrcnt;
#[doc = "USART_EXT Length"]
pub struct LENGTH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USART_EXT Length"]
pub mod length;
#[doc = "USART_EXT Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_EXT Data"]
pub mod data;
#[doc = "USART_EXT Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USART_EXT Debug Control"]
pub mod dbgctrl;
