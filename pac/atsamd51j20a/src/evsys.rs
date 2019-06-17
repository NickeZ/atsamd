#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Software Event"]
    pub swevt: SWEVT,
    #[doc = "0x08 - Priority Control"]
    pub prictrl: PRICTRL,
    _reserved1: [u8; 7usize],
    #[doc = "0x10 - Channel Pending Interrupt"]
    pub intpend: INTPEND,
    _reserved2: [u8; 2usize],
    #[doc = "0x14 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x18 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x1c - Ready Users"]
    pub readyusr: READYUSR,
    #[doc = "0x20 - test"]
    pub channel: [CHANNEL; 32],
    #[doc = "0x120 - User Multiplexer n"]
    pub user: [USER; 67],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control"]
    pub channel: self::channel::CHANNEL,
    #[doc = "0x04 - Channel n Interrupt Enable Clear"]
    pub chintenclr: self::channel::CHINTENCLR,
    #[doc = "0x05 - Channel n Interrupt Enable Set"]
    pub chintenset: self::channel::CHINTENSET,
    #[doc = "0x06 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: self::channel::CHINTFLAG,
    #[doc = "0x07 - Channel n Status"]
    pub chstatus: self::channel::CHSTATUS,
}
#[doc = r" Register block"]
#[doc = "test"]
pub mod channel;
#[doc = "Control"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrla;
#[doc = "Software Event"]
pub struct SWEVT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Event"]
pub mod swevt;
#[doc = "Priority Control"]
pub struct PRICTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "Channel Pending Interrupt"]
pub struct INTPEND {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "Interrupt Status"]
pub struct INTSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "Busy Channels"]
pub struct BUSYCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "Ready Users"]
pub struct READYUSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "User Multiplexer n"]
pub struct USER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Multiplexer n"]
pub mod user;
