pub use libc::{
    can_frame, canfd_frame, canxl_frame, can_filter, sockaddr_can, canid_t, can_err_mask_t,
    CAN_EFF_FLAG, CAN_RTR_FLAG, CAN_ERR_FLAG, CAN_SFF_MASK, CAN_EFF_MASK, CAN_ERR_MASK,
    CAN_MAX_DLC, CAN_MAX_DLEN, CANFD_MAX_DLC, CANFD_MAX_DLEN, CANFD_BRS, CANFD_ESI, CANFD_FDF,
    CAN_MTU, CANFD_MTU, CAN_RAW, CAN_BCM, CAN_TP16, CAN_TP20, CAN_MCNET, CAN_ISOTP, CAN_J1939,
    CAN_NPROTO, AF_CAN, PF_CAN, SOL_CAN_BASE, SOL_CAN_RAW, CAN_RAW_FILTER, CAN_RAW_ERR_FILTER,
    CAN_RAW_LOOPBACK, CAN_RAW_RECV_OWN_MSGS, CAN_RAW_FD_FRAMES, CAN_RAW_JOIN_FILTERS,
    CAN_RAW_FILTER_MAX, CAN_INV_FILTER, c_int, c_void, socklen_t
};

use crate::CanAddr;
use crate::CanFrame;
use crate::CanSocket;
use crate::IoResult;
use crate::Socket;
use crate::as_bytes_mut;

use socket2::SockAddr;

use crate::frame::can_frame_default;
use crate::frame::AsPtr;

use std::io::{Read, Write};

/// Tries to open the CAN socket by the interface number.
pub(crate) fn raw_open_socket(addr: &CanAddr) -> IoResult<socket2::Socket> {
    let af_can = socket2::Domain::from(AF_CAN);
    let can_raw = socket2::Protocol::from(CAN_RAW);

    let sock = socket2::Socket::new_raw(af_can, socket2::Type::RAW, Some(can_raw))?;
    sock.bind(&SockAddr::from(*addr))?;
    Ok(sock)
}

// Wrapper over setsockopt, which we define in our compatibility layers to avoid invalid system
// calls under an incompatible operating system, such as SocketCAN calls on OSX
pub(crate) unsafe fn setsockopt_wrapper(socket :c_int, level :c_int, name :c_int, value :*const c_void, option_len :socklen_t) -> c_int {
    unsafe {
        libc::setsockopt(
            socket,
            level,
            name,
            value as *const _ as *const c_void,
            option_len
        )
    }
}

impl CanSocket {
    /// Reads a low-level libc `can_frame` from the socket.
    pub fn read_raw_frame(&self) -> IoResult<can_frame> {
        let mut frame = can_frame_default();
        self.as_raw_socket().read_exact(as_bytes_mut(&mut frame))?;
        Ok(frame)
    }

    pub fn write_raw_frame<F>(&self, frame :&F) -> IoResult<()>
      where F :Into<CanFrame> + AsPtr {
        self.as_raw_socket().write_all(frame.as_bytes())
    }
}
