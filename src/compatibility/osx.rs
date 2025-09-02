use std::fmt::Debug;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::raw::c_void;

use libc::c_int;
use libc::sa_family_t;
use libc::socklen_t;

use crate::frame::AsPtr;
use crate::CanAddr;
use crate::CanFrame;
use crate::CanSocket;
use crate::IoResult;
use crate::Socket;

/// Extended frame format flag (29-bit ID)
pub const CAN_EFF_FLAG: canid_t = 0x80000000;
/// Remote transmission request flag
pub const CAN_RTR_FLAG: canid_t = 0x40000000;
/// Error frame flag
pub const CAN_ERR_FLAG: canid_t = 0x20000000;

/// Standard frame format ID mask (11-bit)
pub const CAN_SFF_MASK: canid_t = 0x000007FF;
/// Extended frame format ID mask (29-bit)
pub const CAN_EFF_MASK: canid_t = 0x1FFFFFFF;
/// Error frame mask
pub const CAN_ERR_MASK: canid_t = 0x1FFFFFFF;
/// CAN XL priority mask
pub const CANXL_PRIO_MASK: canid_t = CAN_SFF_MASK;

/// Number of bits in standard frame format ID
pub const CAN_SFF_ID_BITS: c_int = 11;
/// Number of bits in extended frame format ID
pub const CAN_EFF_ID_BITS: c_int = 29;
/// Number of bits in CAN XL priority field
pub const CANXL_PRIO_BITS: c_int = CAN_SFF_ID_BITS;

/// CAN error mask type
#[allow(non_camel_case_types)]
pub type can_err_mask_t = u32;

/// Maximum data length code for classic CAN
pub const CAN_MAX_DLC: c_int = 8;
/// Maximum data length for classic CAN frames
pub const CAN_MAX_DLEN: usize = 8;

/// Maximum data length code for CAN FD
pub const CANFD_MAX_DLC: c_int = 15;
/// Maximum data length for CAN FD frames
pub const CANFD_MAX_DLEN: usize = 64;

/// Minimum data length code for CAN XL
pub const CANXL_MIN_DLC: c_int = 0;
/// Maximum data length code for CAN XL
pub const CANXL_MAX_DLC: c_int = 2047;
/// Mask for CAN XL data length code
pub const CANXL_MAX_DLC_MASK: c_int = 0x07FF;
/// Minimum data length for CAN XL frames
pub const CANXL_MIN_DLEN: usize = 1;
/// Maximum data length for CAN XL frames
pub const CANXL_MAX_DLEN: usize = 2048;

/// Inverted CAN filter flag
pub const CAN_INV_FILTER: canid_t = 0x20000000;

/// Classic CAN 2.0 frame structure.
///
/// This structure is compatible with the Linux `can_frame` from libc.
/// Contains a CAN ID, data length code, and up to 8 bytes of data.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct can_frame {
    /// CAN identifier with flags
    pub can_id: canid_t,
    /// Data length code (renamed to `len` in Linux 5.11)
    // FIXME(1.0): this field was renamed to `len` in Linux 5.11
    pub can_dlc: u8,
    /// Padding byte
    __pad: u8,
    /// Reserved field
    __res0: u8,
    /// Data length code for 8-byte alignment
    pub len8_dlc: u8,
    /// Frame payload data
    pub data: [u8; CAN_MAX_DLEN],
}

/// Bit rate switch flag for CAN FD
pub const CANFD_BRS: c_int = 0x01;
/// Error state indicator for CAN FD
pub const CANFD_ESI: c_int = 0x02;
/// CAN FD format flag
pub const CANFD_FDF: c_int = 0x04;

/// CAN FD (Flexible Data Rate) frame structure.
///
/// This structure is compatible with the Linux `canfd_frame` from libc.
/// Supports up to 64 bytes of data payload.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct canfd_frame {
    /// CAN identifier with flags
    pub can_id: canid_t,
    /// Data length
    pub len: u8,
    /// CAN FD flags (BRS, ESI, FDF)
    pub flags: u8,
    /// Reserved field 0
    __res0: u8,
    /// Reserved field 1
    __res1: u8,
    /// Frame payload data
    pub data: [u8; CANFD_MAX_DLEN],
}

/// CAN XL format flag
pub const CANXL_XLF: c_int = 0x80;
/// CAN XL simple extended content flag
pub const CANXL_SEC: c_int = 0x01;

/// CAN XL (eXtended Length) frame structure.
///
/// This structure is compatible with the Linux `canxl_frame` from libc.
/// Supports up to 2048 bytes of data payload.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct canxl_frame {
    /// Priority field
    pub prio: canid_t,
    /// CAN XL flags
    pub flags: u8,
    /// Service data type
    pub sdt: u8,
    /// Data length
    pub len: u16,
    /// Acceptance field
    pub af: u32,
    /// Frame payload data
    pub data: [u8; CANXL_MAX_DLEN],
}

/// Maximum transmission unit for classic CAN frames
pub const CAN_MTU: usize = size_of::<can_frame>();
/// Maximum transmission unit for CAN FD frames
pub const CANFD_MTU: usize = size_of::<canfd_frame>();
/// Maximum transmission unit for CAN XL frames
pub const CANXL_MTU: usize = size_of::<canxl_frame>();
/// Size of CAN XL header (without data payload)
// FIXME(offset_of): use `core::mem::offset_of!` once that is available
// https://github.com/rust-lang/rfcs/pull/3308
// pub const CANXL_HDR_SIZE: usize = core::mem::offset_of!(canxl_frame, data);
pub const CANXL_HDR_SIZE: usize = 12;
/// Minimum MTU for CAN XL frames
pub const CANXL_MIN_MTU: usize = CANXL_HDR_SIZE + 64;
/// Maximum MTU for CAN XL frames
pub const CANXL_MAX_MTU: usize = CANXL_MTU;

/// Raw CAN protocol
pub const CAN_RAW: c_int = 1;
/// Broadcast manager protocol
pub const CAN_BCM: c_int = 2;
/// VAG transport protocol v1.6
pub const CAN_TP16: c_int = 3;
/// VAG transport protocol v2.0
pub const CAN_TP20: c_int = 4;
/// Mercedes MCNET protocol
pub const CAN_MCNET: c_int = 5;
/// ISO-TP transport protocol
pub const CAN_ISOTP: c_int = 6;
/// SAE J1939 protocol
pub const CAN_J1939: c_int = 7;
/// Number of CAN protocols
pub const CAN_NPROTO: c_int = 8;

/// Socket option level base for CAN
/// An invalid number to trigger a runtime error,
/// as SocketCAN is not supported on OSX.
pub const SOL_CAN_BASE: c_int = 0xFFFFF;
/// CAN address family
/// An invalid number to trigger a runtime error,
/// as SocketCAN is not supported on OSX.
pub const AF_CAN :c_int = 0xFFFFF;
/// CAN protocol family
pub const PF_CAN: c_int = AF_CAN;

/// Socket option level for raw CAN
pub const SOL_CAN_RAW: c_int = SOL_CAN_BASE + CAN_RAW;
/// Maximum number of CAN filters per raw socket
pub const CAN_RAW_FILTER_MAX: c_int = 512;

/// Socket option: set CAN filters
// FIXME(cleanup): use `c_enum!`, which needs to be adapted to allow omitting a type.
pub const CAN_RAW_FILTER: c_int = 1;
/// Socket option: set/get error filter
pub const CAN_RAW_ERR_FILTER: c_int = 2;
/// Socket option: enable/disable loopback
pub const CAN_RAW_LOOPBACK: c_int = 3;
/// Socket option: receive own messages
pub const CAN_RAW_RECV_OWN_MSGS: c_int = 4;
/// Socket option: enable CAN FD frames
pub const CAN_RAW_FD_FRAMES: c_int = 5;
/// Socket option: join filters
pub const CAN_RAW_JOIN_FILTERS: c_int = 6;
/// Socket option: enable CAN XL frames
pub const CAN_RAW_XL_FRAMES: c_int = 7;

/// CAN identifier type
#[allow(non_camel_case_types)]
pub type canid_t = u32;

/// SocketCAN address structure.
///
/// This structure is compatible with the Linux `sockaddr_can` from libc.
/// Used for binding CAN sockets to specific interfaces.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct sockaddr_can {
    /// Address family (AF_CAN)
    pub can_family: sa_family_t,
    /// CAN interface index
    pub can_ifindex: c_int,
    /// Protocol-specific address information
    pub can_addr: __c_anonymous_sockaddr_can_can_addr,
}

/// Anonymous union for protocol-specific CAN address data.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub union __c_anonymous_sockaddr_can_can_addr {
    /// Transport protocol address
    pub tp: __c_anonymous_sockaddr_can_tp,
    /// J1939 protocol address
    pub j1939: __c_anonymous_sockaddr_can_j1939,
}

impl Debug for __c_anonymous_sockaddr_can_can_addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("__c_anonymous_sockaddr_can_can_addr OSX compatible")
    }
}

/// Transport protocol address structure.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct __c_anonymous_sockaddr_can_tp {
    /// Receive CAN ID
    pub rx_id: canid_t,
    /// Transmit CAN ID
    pub tx_id: canid_t,
}

/// J1939 protocol address structure.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct __c_anonymous_sockaddr_can_j1939 {
    /// J1939 name field
    pub name: u64,
    /// Parameter group number
    pub pgn: u32,
    /// J1939 address
    pub addr: u8,
}

/// CAN filter structure.
///
/// This structure is compatible with the Linux `can_filter` from libc.
/// Used to define acceptance filters for CAN frames.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct can_filter {
    /// CAN ID to match
    pub can_id: canid_t,
    /// CAN ID mask for filtering
    pub can_mask: canid_t,
}

pub(crate) fn raw_open_socket(_addr: &CanAddr) -> IoResult<socket2::Socket> {
    panic!("Not supported outside of Linux")
}

impl CanSocket {
    /// Reads a low-level libc `can_frame` from the socket.
    pub fn read_raw_frame(&self) -> IoResult<can_frame> {
        panic!("Not supported outside of Linux")
    }

    /// Writes a CanFrame to the socket
    pub fn write_raw_frame<F>(&self, _frame :&F) -> IoResult<()>
      where F :Into<CanFrame> + AsPtr {
        panic!("Not supported outside of Linux")
    }
}

// Wrapper over setsockopt, which we define in our compatibility layers to avoid invalid system
// calls under an incompatible operating system, such as SocketCAN calls on OSX
pub(crate) unsafe fn setsockopt_wrapper(socket :c_int, level :c_int, name :c_int, value :*const c_void, option_len :socklen_t) -> c_int {
    unsafe {
        // Raw setsockopt values are only supported on Linux
        // We use dummy values anyways, but this panic is important to have a controlled
        // runtime exception.

        if SOL_CAN_RAW == level {
            panic!("Not supported outside of Linux")
        } else {
            libc::setsockopt(
                socket,
                level,
                name,
                value as *const _ as *const c_void,
                option_len
            )
        }
    }
}
