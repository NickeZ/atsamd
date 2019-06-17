use usb::host::HOST_PIPE;
use usb::device::DEVICE_ENDPOINT;

#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB is Device"]
    pub device: DEVICE,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct DEVICE {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::device::CTRLA,
    _reserved0: [u8; 1usize],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: self::device::SYNCBUSY,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: self::device::QOSCTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - DEVICE Control B"]
    pub ctrlb: self::device::CTRLB,
    #[doc = "0x0a - DEVICE Device Address"]
    pub dadd: self::device::DADD,
    _reserved2: [u8; 1usize],
    #[doc = "0x0c - DEVICE Status"]
    pub status: self::device::STATUS,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: self::device::FSMSTATUS,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - DEVICE Device Frame Number"]
    pub fnum: self::device::FNUM,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - DEVICE Device Interrupt Enable Clear"]
    pub intenclr: self::device::INTENCLR,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - DEVICE Device Interrupt Enable Set"]
    pub intenset: self::device::INTENSET,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - DEVICE Device Interrupt Flag"]
    pub intflag: self::device::INTFLAG,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - DEVICE End Point Interrupt Summary"]
    pub epintsmry: self::device::EPINTSMRY,
    _reserved8: [u8; 2usize],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: self::device::DESCADD,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: self::device::PADCAL,
    _reserved9: [u8; 214usize],
    #[doc = "0x100 - test"]
    pub device_endpoint0: DEVICE_ENDPOINT,
    _reserved10: [u8; 22usize],
    #[doc = "0x120 - test"]
    pub device_endpoint1: DEVICE_ENDPOINT,
    _reserved11: [u8; 22usize],
    #[doc = "0x140 - test"]
    pub device_endpoint2: DEVICE_ENDPOINT,
    _reserved12: [u8; 22usize],
    #[doc = "0x160 - test"]
    pub device_endpoint3: DEVICE_ENDPOINT,
    _reserved13: [u8; 22usize],
    #[doc = "0x180 - test"]
    pub device_endpoint4: DEVICE_ENDPOINT,
    _reserved14: [u8; 22usize],
    #[doc = "0x1a0 - test"]
    pub device_endpoint5: DEVICE_ENDPOINT,
    _reserved15: [u8; 22usize],
    #[doc = "0x1c0 - test"]
    pub device_endpoint6: DEVICE_ENDPOINT,
    _reserved16: [u8; 22usize],
    #[doc = "0x1e0 - test"]
    pub device_endpoint7: DEVICE_ENDPOINT,
}
#[doc = r" Register block"]
#[doc = "USB is Device"]
pub mod device;
#[doc = r" Register block"]
#[repr(C)]
pub struct HOST {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::host::CTRLA,
    _reserved0: [u8; 1usize],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: self::host::SYNCBUSY,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: self::host::QOSCTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - HOST Control B"]
    pub ctrlb: self::host::CTRLB,
    #[doc = "0x0a - HOST Host Start Of Frame Control"]
    pub hsofc: self::host::HSOFC,
    _reserved2: [u8; 1usize],
    #[doc = "0x0c - HOST Status"]
    pub status: self::host::STATUS,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: self::host::FSMSTATUS,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - HOST Host Frame Number"]
    pub fnum: self::host::FNUM,
    #[doc = "0x12 - HOST Host Frame Length"]
    pub flenhigh: self::host::FLENHIGH,
    _reserved4: [u8; 1usize],
    #[doc = "0x14 - HOST Host Interrupt Enable Clear"]
    pub intenclr: self::host::INTENCLR,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - HOST Host Interrupt Enable Set"]
    pub intenset: self::host::INTENSET,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - HOST Host Interrupt Flag"]
    pub intflag: self::host::INTFLAG,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - HOST Pipe Interrupt Summary"]
    pub pintsmry: self::host::PINTSMRY,
    _reserved8: [u8; 2usize],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: self::host::DESCADD,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: self::host::PADCAL,
    _reserved9: [u8; 214usize],
    #[doc = "0x100 - test"]
    pub host_pipe0: HOST_PIPE,
    _reserved10: [u8; 22usize],
    #[doc = "0x120 - test"]
    pub host_pipe1: HOST_PIPE,
    _reserved11: [u8; 22usize],
    #[doc = "0x140 - test"]
    pub host_pipe2: HOST_PIPE,
    _reserved12: [u8; 22usize],
    #[doc = "0x160 - test"]
    pub host_pipe3: HOST_PIPE,
    _reserved13: [u8; 22usize],
    #[doc = "0x180 - test"]
    pub host_pipe4: HOST_PIPE,
    _reserved14: [u8; 22usize],
    #[doc = "0x1a0 - test"]
    pub host_pipe5: HOST_PIPE,
    _reserved15: [u8; 22usize],
    #[doc = "0x1c0 - test"]
    pub host_pipe6: HOST_PIPE,
    _reserved16: [u8; 22usize],
    #[doc = "0x1e0 - test"]
    pub host_pipe7: HOST_PIPE,
}
#[doc = r" Register block"]
#[doc = "USB is Host"]
pub mod host;
