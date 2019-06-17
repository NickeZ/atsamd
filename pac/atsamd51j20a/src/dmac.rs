#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x02 - CRC Control"]
    pub crcctrl: CRCCTRL,
    #[doc = "0x04 - CRC Data Input"]
    pub crcdatain: CRCDATAIN,
    #[doc = "0x08 - CRC Checksum"]
    pub crcchksum: CRCCHKSUM,
    #[doc = "0x0c - CRC Status"]
    pub crcstatus: CRCSTATUS,
    #[doc = "0x0d - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved0: [u8; 2usize],
    #[doc = "0x10 - Software Trigger Control"]
    pub swtrigctrl: SWTRIGCTRL,
    #[doc = "0x14 - Priority Control 0"]
    pub prictrl0: PRICTRL0,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - Interrupt Pending"]
    pub intpend: INTPEND,
    _reserved2: [u8; 2usize],
    #[doc = "0x24 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x28 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x2c - Pending Channels"]
    pub pendch: PENDCH,
    #[doc = "0x30 - Active Channel and Levels"]
    pub active: ACTIVE,
    #[doc = "0x34 - Descriptor Memory Section Base Address"]
    pub baseaddr: BASEADDR,
    #[doc = "0x38 - Write-Back Memory Section Base Address"]
    pub wrbaddr: WRBADDR,
    _reserved3: [u8; 4usize],
    #[doc = "0x40 - test"]
    pub channel: [CHANNEL; 32],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control A"]
    pub chctrla: self::channel::CHCTRLA,
    #[doc = "0x04 - Channel n Control B"]
    pub chctrlb: self::channel::CHCTRLB,
    #[doc = "0x05 - Channel n Priority Level"]
    pub chprilvl: self::channel::CHPRILVL,
    #[doc = "0x06 - Channel n Event Control"]
    pub chevctrl: self::channel::CHEVCTRL,
    _reserved0: [u8; 5usize],
    #[doc = "0x0c - Channel n Interrupt Enable Clear"]
    pub chintenclr: self::channel::CHINTENCLR,
    #[doc = "0x0d - Channel n Interrupt Enable Set"]
    pub chintenset: self::channel::CHINTENSET,
    #[doc = "0x0e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: self::channel::CHINTFLAG,
    #[doc = "0x0f - Channel n Status"]
    pub chstatus: self::channel::CHSTATUS,
}
#[doc = r" Register block"]
#[doc = "test"]
pub mod channel;
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "CRC Control"]
pub struct CRCCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRC Data Input"]
pub struct CRCDATAIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Data Input"]
pub mod crcdatain;
#[doc = "CRC Checksum"]
pub struct CRCCHKSUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Checksum"]
pub mod crcchksum;
#[doc = "CRC Status"]
pub struct CRCSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC Status"]
pub mod crcstatus;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Software Trigger Control"]
pub struct SWTRIGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Trigger Control"]
pub mod swtrigctrl;
#[doc = "Priority Control 0"]
pub struct PRICTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Control 0"]
pub mod prictrl0;
#[doc = "Interrupt Pending"]
pub struct INTPEND {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Pending"]
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
#[doc = "Pending Channels"]
pub struct PENDCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending Channels"]
pub mod pendch;
#[doc = "Active Channel and Levels"]
pub struct ACTIVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Active Channel and Levels"]
pub mod active;
#[doc = "Descriptor Memory Section Base Address"]
pub struct BASEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Memory Section Base Address"]
pub mod baseaddr;
#[doc = "Write-Back Memory Section Base Address"]
pub struct WRBADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write-Back Memory Section Base Address"]
pub mod wrbaddr;
