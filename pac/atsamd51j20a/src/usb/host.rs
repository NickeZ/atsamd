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
#[doc = "HOST Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Control B"]
pub mod ctrlb;
#[doc = "HOST Host Start Of Frame Control"]
pub struct HSOFC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Host Start Of Frame Control"]
pub mod hsofc;
#[doc = "HOST Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Status"]
pub mod status;
#[doc = "Finite State Machine Status"]
pub struct FSMSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "HOST Host Frame Number"]
pub struct FNUM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Host Frame Number"]
pub mod fnum;
#[doc = "HOST Host Frame Length"]
pub struct FLENHIGH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HOST Host Frame Length"]
pub mod flenhigh;
#[doc = "HOST Host Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Host Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "HOST Host Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Host Interrupt Enable Set"]
pub mod intenset;
#[doc = "HOST Host Interrupt Flag"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Host Interrupt Flag"]
pub mod intflag;
#[doc = "HOST Pipe Interrupt Summary"]
pub struct PINTSMRY {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "HOST Pipe Interrupt Summary"]
pub mod pintsmry;
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
pub struct HOST_PIPE {
    #[doc = "0x00 - HOST_PIPE End Point Configuration"]
    pub pcfg: self::host_pipe::PCFG,
    _reserved0: [u8; 2usize],
    #[doc = "0x03 - HOST_PIPE Bus Access Period of Pipe"]
    pub binterval: self::host_pipe::BINTERVAL,
    #[doc = "0x04 - HOST_PIPE End Point Pipe Status Clear"]
    pub pstatusclr: self::host_pipe::PSTATUSCLR,
    #[doc = "0x05 - HOST_PIPE End Point Pipe Status Set"]
    pub pstatusset: self::host_pipe::PSTATUSSET,
    #[doc = "0x06 - HOST_PIPE End Point Pipe Status"]
    pub pstatus: self::host_pipe::PSTATUS,
    #[doc = "0x07 - HOST_PIPE Pipe Interrupt Flag"]
    pub pintflag: self::host_pipe::PINTFLAG,
    #[doc = "0x08 - HOST_PIPE Pipe Interrupt Flag Clear"]
    pub pintenclr: self::host_pipe::PINTENCLR,
    #[doc = "0x09 - HOST_PIPE Pipe Interrupt Flag Set"]
    pub pintenset: self::host_pipe::PINTENSET,
}
#[doc = r" Register block"]
#[doc = "test"]
pub mod host_pipe;
