#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "USB Quality Of Service"]
pub struct QOSCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Quality Of Service"]
pub mod qosctrl;
#[doc = "DEVICE Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Control B"]
pub mod ctrlb;
#[doc = "DEVICE Device Address"]
pub struct DADD {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE Device Address"]
pub mod dadd;
#[doc = "DEVICE Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DEVICE Status"]
pub mod status;
#[doc = "Finite State Machine Status"]
pub struct FSMSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "DEVICE Device Frame Number"]
pub struct FNUM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Device Frame Number"]
pub mod fnum;
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "DEVICE Device Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Device Interrupt Enable Set"]
pub mod intenset;
#[doc = "DEVICE Device Interrupt Flag"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE Device Interrupt Flag"]
pub mod intflag;
#[doc = "DEVICE End Point Interrupt Summary"]
pub struct EPINTSMRY {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DEVICE End Point Interrupt Summary"]
pub mod epintsmry;
#[doc = "Descriptor Address"]
pub struct DESCADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Address"]
pub mod descadd;
#[doc = "USB PAD Calibration"]
pub struct PADCAL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB PAD Calibration"]
pub mod padcal;
#[doc = r" Register block"]
#[repr(C)]
pub struct DEVICE_ENDPOINT {
    #[doc = "0x00 - DEVICE_ENDPOINT End Point Configuration"]
    pub epcfg: self::device_endpoint::EPCFG,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - DEVICE_ENDPOINT End Point Pipe Status Clear"]
    pub epstatusclr: self::device_endpoint::EPSTATUSCLR,
    #[doc = "0x05 - DEVICE_ENDPOINT End Point Pipe Status Set"]
    pub epstatusset: self::device_endpoint::EPSTATUSSET,
    #[doc = "0x06 - DEVICE_ENDPOINT End Point Pipe Status"]
    pub epstatus: self::device_endpoint::EPSTATUS,
    #[doc = "0x07 - DEVICE_ENDPOINT End Point Interrupt Flag"]
    pub epintflag: self::device_endpoint::EPINTFLAG,
    #[doc = "0x08 - DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
    pub epintenclr: self::device_endpoint::EPINTENCLR,
    #[doc = "0x09 - DEVICE_ENDPOINT End Point Interrupt Set Flag"]
    pub epintenset: self::device_endpoint::EPINTENSET,
}
#[doc = r" Register block"]
#[doc = "test"]
pub mod device_endpoint;
