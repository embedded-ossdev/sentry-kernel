// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-License-Identifier: Apache-2.0

use crate::systypes::shm::ShmInfo;
use crate::systypes::{ExchangeHeader, ShmHandle, Status};
use core::ptr::*;

const EXCHANGE_AREA_LEN: usize = 128; // TODO: replace by CONFIG-defined value

/// The effective kernelspace/userspace exchange zone, set in a dedicated section
///
#[unsafe(link_section = ".svcexchange")]
static mut EXCHANGE_AREA: [u8; EXCHANGE_AREA_LEN] = [0u8; EXCHANGE_AREA_LEN];

/// Trait of kernel-user exchangeable objects
///
/// This trait written in order to support the notion of "kernel-exchangeable"
/// type. Any type that do support this trait can be delivered to (and/or received
/// from) the kernel.
///
/// The effective implementation of this trait public functions are kept private,
/// and can't be implemented out of this very crate in order to ensure that only
/// this crate's declared types are exchangeable.
/// To ensure such a restriction, this trait is hosted by the current, crate-private,
/// module.
///
/// As a consequence, don't try to implement that trait in any of the upper layers.
///
pub trait SentryExchangeable {
    /// Copy the current object to the kernel exchagne zone
    fn to_kernel(&self) -> Result<Status, Status>;

    /// set the current object using the data delivered by the kernel in the exchange zone
    fn from_kernel(&mut self) -> Result<Status, Status>;
}

/// SentryExchangeable trait implementation for ShmInfo.
/// Shminfo is a typical structure which is returned by the kernel to the
/// userspace in order to delivers various SHM-related information to a given
/// task that is using the corresponding SHM.
///
/// In test mode only, this structure can be written back to the Exchange Area.
/// In production mode, the application can't write such a content to the exchange
/// as the kernel as strictly no use of it.
///
impl SentryExchangeable for crate::systypes::shm::ShmInfo {
    #[allow(static_mut_refs)]
    fn from_kernel(&mut self) -> Result<Status, Status> {
        unsafe {
            core::ptr::copy_nonoverlapping(
                EXCHANGE_AREA.as_ptr(),
                addr_of_mut!(*self) as *mut u8,
                core::mem::size_of::<ShmInfo>().min(EXCHANGE_AREA_LEN),
            );
        }
        Ok(Status::Ok)
    }

    #[cfg(test)]
    #[allow(static_mut_refs)]
    fn to_kernel(&self) -> Result<Status, Status> {
        unsafe {
            core::ptr::copy_nonoverlapping(
                addr_of!(*self) as *const u8,
                EXCHANGE_AREA.as_mut_ptr(),
                core::mem::size_of::<ShmInfo>().min(EXCHANGE_AREA_LEN),
            );
        }
        Ok(Status::Ok)
    }

    #[cfg(not(test))]
    #[allow(static_mut_refs)]
    fn to_kernel(&self) -> Result<Status, Status> {
        Err(Status::Invalid)
    }
}

/// SentryExchangeable trait implementation for ShmHandle.
/// ShmHandle is a another structure which is returned by the kernel to the
/// userspace in order to delivers SHM handle to a given
/// task.
///
/// In test mode only, this structure can be written back to the Exchange Area.
/// In production mode, the application can't write such a content to the exchange
/// as the kernel as strictly no use of it.
///
impl SentryExchangeable for crate::systypes::ShmHandle {
    #[allow(static_mut_refs)]
    fn from_kernel(&mut self) -> Result<Status, Status> {
        unsafe {
            core::ptr::copy_nonoverlapping(
                EXCHANGE_AREA.as_ptr() as *const u32,
                addr_of_mut!(*self) as *mut u32,
                core::mem::size_of::<ShmHandle>().min(EXCHANGE_AREA_LEN),
            );
        }
        Ok(Status::Ok)
    }

    #[cfg(test)]
    #[allow(static_mut_refs)]
    fn to_kernel(&self) -> Result<Status, Status> {
        unsafe {
            core::ptr::copy_nonoverlapping(
                addr_of!(*self) as *const u8,
                EXCHANGE_AREA.as_mut_ptr(),
                core::mem::size_of::<ShmHandle>().min(EXCHANGE_AREA_LEN),
            );
        }
        Ok(Status::Ok)
    }

    #[cfg(not(test))]
    #[allow(static_mut_refs)]
    fn to_kernel(&self) -> Result<Status, Status> {
        Err(Status::Invalid)
    }
}

// from-exchange related capacity to Exchang header
impl ExchangeHeader {
    unsafe fn from_addr(self, address: usize) -> &'static Self {
        &*(address as *const Self)
    }

    #[cfg(test)]
    unsafe fn from_addr_mut(self, address: usize) -> &'static mut Self {
        &mut *(address as *mut Self)
    }

    pub unsafe fn from_exchange(self) -> &'static Self {
        self.from_addr(EXCHANGE_AREA.as_ptr() as usize)
    }

    #[cfg(test)]
    pub unsafe fn from_exchange_mut(self) -> &'static mut Self {
        self.from_addr_mut(EXCHANGE_AREA.as_mut_ptr() as usize)
    }
}

/// Event SentryExchangeable trait implementation
///
/// Events are objects that are used to hold event ifnormation that need to be delivered or
/// received from the kernel.
///
/// Events are received from the kernel and hold a header and an associated data bloc.
/// The SentryExchangeable trait only support, in nominal mode, the from_kernel() usage for any
/// event, and to_kernel when emitting IPC
///
/// This trait allows to easily receive or deliver properly formatted events, including the
/// event header forged by the kernel and associated data.
///
/// # Example
///
/// ```ignore
/// let mut my_event = uapi::systypes::Event {
///     header: uapi::systypes::ExchangeHeader {
///         peer: 0,
///         event: uapi::systypes::EventType::None.into(),
///         length: 0,
///         magic: 0,
///     },
///     data: &mut[0; 12],
/// };
/// // wait for kernel events of type IRQ or IPC
/// let _ = uapi::syscall::wait_for_event(
///             uapi::systypes::EventType::IRQ.into() | uapi::systypes::EventType::Ipc.into(),
///             0;
///         );
/// // get back event data from kernel
/// let _ = my_event.from_kernel();
/// // handle event
/// if !my_event.header.is_valid() {
///     return Err(),
/// }
/// match my_event.header.event {
///     EventType::Irq => treat_irq(&my_event.data, my_event.length),
///     EventType::IPC => treat_ipc(&my_event.data,  my_event.length),
///     any_other      => Err(),
/// }
/// ```
///
impl SentryExchangeable for crate::systypes::Event<'_> {
    #[allow(static_mut_refs)]
    fn from_kernel(&mut self) -> Result<Status, Status> {
        // declare exchange as header first
        let k_header: &ExchangeHeader = unsafe { ExchangeHeader::from_exchange(self.header) };
        if !k_header.is_valid() {
            return Err(Status::Invalid);
        }
        self.header = *k_header;
        let header_len = core::mem::size_of::<ExchangeHeader>() as usize;
        // be sure we have enough size in exchange zone
        if header_len + usize::from(self.header.length) > EXCHANGE_AREA_LEN {
            return Err(Status::Invalid);
        }
        if usize::from(self.header.length) > EXCHANGE_AREA_LEN - header_len {
            // the length field is set by the kernel and thus, should not be invalid
            // yet we check that there is no overflow as we use an unsafe block to get
            // back from the exchange area
            return Err(Status::Invalid);
        }
        // copy the amount of data in data slice using the header length info.
        // Note: here we do not do any semantic related content check (i.e. data length or content
        // based on the exchange type) but we let the kernel ensuring the correlation instead.
        unsafe {
            let data_ptr: *const u8 = (EXCHANGE_AREA.as_ptr() as usize + header_len) as *const u8;
            let data_slice = core::slice::from_raw_parts(data_ptr, self.header.length.into());
            // the destination slice must have enough space to get back data from the exchange zone
            if data_slice.iter().count() > self.data.len() {
                return Err(Status::Invalid);
            }
            for (dst, src) in self.data.iter_mut().zip(data_slice.iter()) {
                *dst = *src
            }
        }
        Ok(Status::Ok)
    }

    /// Event can be used as source for to_kernel() when being an IPC
    ///
    /// Events not being and IPC do not need to emit any data in the
    /// kernel exchange zone, leading to Err(Status::Invalid) return code.
    ///
    #[cfg(not(test))]
    fn to_kernel(&self) -> Result<Status, Status> {
        use crate::systypes::EventType;

        if self.header.event == EventType::Ipc.into() {
            self.data.to_kernel()
        } else {
            Err(Status::Invalid)
        }
    }

    #[cfg(test)]
    #[allow(static_mut_refs)]
    fn to_kernel(&self) -> Result<Status, Status> {
        // copy exchange header to exhcange zone
        let k_header: &mut ExchangeHeader =
            unsafe { ExchangeHeader::from_exchange_mut(self.header) };
        let header_len = core::mem::size_of::<ExchangeHeader>() as usize;
        k_header.peer = self.header.peer;
        k_header.magic = self.header.magic;
        k_header.length = self.header.length;
        k_header.event = self.header.event;
        // now append data to header in exchange zone
        if usize::from(self.header.length) > self.data.len() {
            return Err(Status::Invalid);
        }
        unsafe {
            let data_addr = EXCHANGE_AREA.as_ptr() as usize + header_len;
            let data_ptr =
                data_addr as *mut [u8; EXCHANGE_AREA_LEN - core::mem::size_of::<ExchangeHeader>()];
            core::ptr::copy_nonoverlapping(
                self.data.as_ptr(),
                data_ptr as *mut u8,
                self.header.length.into(),
            );
        }
        Ok(Status::Ok)
    }
}

impl SentryExchangeable for &mut [u8] {
    #[allow(static_mut_refs)]
    fn from_kernel(&mut self) -> Result<Status, Status> {
        unsafe {
            core::ptr::copy_nonoverlapping(
                EXCHANGE_AREA.as_ptr(),
                self.as_mut_ptr(),
                self.len().min(EXCHANGE_AREA_LEN),
            );
        }
        Ok(Status::Ok)
    }

    #[allow(static_mut_refs)]
    fn to_kernel(&self) -> Result<Status, Status> {
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.as_ptr(),
                EXCHANGE_AREA.as_mut_ptr(),
                self.len().min(EXCHANGE_AREA_LEN),
            );
        }
        Ok(Status::Ok)
    }
}

impl SentryExchangeable for &[u8] {
    #[allow(static_mut_refs)]
    fn from_kernel(&mut self) -> Result<Status, Status> {
        Err(Status::Invalid)
    }

    #[allow(static_mut_refs)]
    fn to_kernel(&self) -> Result<Status, Status> {
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.as_ptr(),
                EXCHANGE_AREA.as_mut_ptr(),
                self.len().min(EXCHANGE_AREA_LEN),
            );
        }
        Ok(Status::Ok)
    }
}

pub const fn length() -> usize {
    EXCHANGE_AREA_LEN
}

/// copy to kernel generic implementation
///
/// This API is a generic implementation in order to allow userspace to kernelspace
/// exchange for any type that do implement the SentryExchangeable trait.
///
/// # Example
///
/// A basic usage would look like the following:
///
/// ```ignore
/// let my_info: &[u8] = &[1,2,3,4];
/// copy_to_kernel(my_info);
/// ```
///
pub fn copy_to_kernel<T>(from: &T) -> Result<Status, Status>
where
    T: SentryExchangeable + ?Sized,
{
    from.to_kernel()
}

pub fn copy_from_kernel<T>(to: &mut T) -> Result<Status, Status>
where
    T: SentryExchangeable + ?Sized,
{
    to.from_kernel()
}

#[cfg(test)]
mod tests {
    use crate::systypes::EventType;

    use super::*;

    #[test]
    fn back_to_back_shminfo() {
        let src = ShmInfo {
            handle: 2,
            label: 42,
            base: 0x123456,
            len: 64,
            perms: 0x1,
        };
        let mut dst = ShmInfo {
            handle: 0,
            label: 0,
            base: 0,
            len: 0,
            perms: 0,
        };
        let _ = src.to_kernel();
        let _ = dst.from_kernel();
        assert_eq!(src, dst);
    }

    #[test]
    fn back_to_back_shmhandle() {
        let src = 2;
        let mut dst = 2;
        let _ = src.to_kernel();
        let _ = dst.from_kernel();
        assert_eq!(src, dst);
    }

    #[test]
    fn back_to_back_event() {
        let src = crate::systypes::Event {
            header: ExchangeHeader {
                peer: 0x42,
                event: EventType::Irq.into(),
                length: 12,
                magic: 0x4242,
            },
            data: &mut [0x42; 12],
        };
        let mut dst = crate::systypes::Event {
            header: ExchangeHeader {
                peer: 0,
                event: EventType::None.into(),
                length: 0,
                magic: 0,
            },
            data: &mut [0; 12],
        };
        assert_eq!(src.to_kernel(), Ok(Status::Ok));
        assert_eq!(dst.from_kernel(), Ok(Status::Ok));
        assert_eq!(src.header, dst.header);
        assert_eq!(
            src.data[..src.header.length.into()],
            dst.data[..src.header.length.into()]
        );
        let mut shorter_dst = crate::systypes::Event {
            header: ExchangeHeader {
                peer: 0,
                event: EventType::None.into(),
                length: 0,
                magic: 0,
            },
            data: &mut [0; 8],
        };
        // dest that are not able to hold overall data must not generate panic
        assert_eq!(shorter_dst.from_kernel(), Err(Status::Invalid));
    }

    #[test]
    fn back_to_back_c_string() {
        let src: &[u8] = &[42, 1, 3, 5, 12];
        let mut dst: &mut [u8] = &mut [0, 0, 0, 0, 0];
        assert_eq!(src.to_kernel(), Ok(Status::Ok));
        assert_eq!(dst.from_kernel(), Ok(Status::Ok));
        assert_eq!(src, dst);
    }
}
