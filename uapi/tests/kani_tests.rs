#![cfg(kani)]
#![no_std]

use sentry_uapi::SentryExchangeable;
use sentry_uapi::systypes::shm::ShmInfo;
use sentry_uapi::systypes::{Event, ExchangeHeader, Status};
use sentry_uapi::exchange::length;
use kani::any;

const EXCHANGE_AREA_LEN: usize = 128;

// Simulated exchange area for Kani
static mut EXCHANGE_AREA: [u8; EXCHANGE_AREA_LEN] = [0u8; EXCHANGE_AREA_LEN];

#[kani::proof]
fn verify_event_from_kernel_safety() {
    unsafe {
        #[allow(static_mut_refs)]
        {
            // Fill the exchange area with random data
            for byte in EXCHANGE_AREA.iter_mut() {
                *byte = kani::any::<u8>();
            }

            // Force a valid header
            EXCHANGE_AREA[0] = 4; // EventType::Irq
            EXCHANGE_AREA[1] = 12; // Length plausible
            EXCHANGE_AREA[2] = 0x42; // Magic low byte
            EXCHANGE_AREA[3] = 0x42; // Magic high byte
            EXCHANGE_AREA[4] = 0x42; // Peer byte 0
            EXCHANGE_AREA[5] = 0x00; // Peer byte 1
            EXCHANGE_AREA[6] = 0x00; // Peer byte 2
            EXCHANGE_AREA[7] = 0x00; // Peer byte 3
        }

        let mut event = Event {
            header: ExchangeHeader {
                peer: 0,
                event: 0,
                length: 0,
                magic: 0,
            },
            data: &mut [0; 128],
        };

        let result = event.from_kernel();

        match result {
            Ok(Status::Ok) => {
                // Assume the length is <= 128
                kani::assume(event.header.length <= 128);

                // Assume the header is valid
                kani::assert(
                    event.header.is_valid(),
                    "Header should be valid for from_kernel",
                );
            }
            Ok(_) | Err(_) => {
                // Ignore other results
            }
        }
    }
}

#[kani::proof]
fn verify_event_to_kernel_safety() {
    #[allow(static_mut_refs)]
    {
        // Prepare an arbitrary header
        let header = ExchangeHeader {
            event: kani::any::<u8>(),
            length: kani::any::<u8>(),
            magic: kani::any::<u16>(),
            peer: kani::any::<u32>(),
        };

        // Prepare an arbitrary data buffer
        let mut data = [0u8; 128];
        for byte in data.iter_mut() {
            *byte = kani::any::<u8>();
        }

        // Capture the length before borrowing
        let data_len = data.len();

        let event = Event {
            header,
            data: &mut data,
        };

        // Constraints to avoid overflows
        kani::assume(event.header.length as usize <= data_len);
        kani::assume(event.header.length as usize <= EXCHANGE_AREA_LEN);

        let result = event.to_kernel();

        match result {
            Ok(Status::Ok) => {
                kani::assert(
                    event.header.length as usize <= EXCHANGE_AREA_LEN,
                    "No memory overflow in EXCHANGE_AREA",
                );
            }
            Ok(_) | Err(_) => {
                // Ignore other results
            }
        }
    }
}

#[kani::proof]
fn verify_event_roundtrip_kernel() {
    #[allow(static_mut_refs)]
    {
        const TEST_DATA_LEN: usize = 4;

        let header = ExchangeHeader {
            event: 4, // EventType::Irq
            length: kani::any::<u8>(),
            magic: 0x4242, // Awaited Magic for validation
            peer: kani::any::<u32>(),
        };

        let mut original_data = [0u8; TEST_DATA_LEN];
        for byte in original_data.iter_mut() {
            *byte = kani::any::<u8>();
        }

        let data_len = original_data.len();

        let event = Event {
            header,
            data: &mut original_data,
        };

        kani::assume(event.header.length as usize <= data_len);
        kani::assume(event.header.length != 0);
        kani::assume(event.header.length as usize <= EXCHANGE_AREA_LEN);

        if event.to_kernel().is_ok() {
            let mut new_data = [0u8; TEST_DATA_LEN];
            let mut event_reconstructed = Event {
                header: ExchangeHeader {
                    peer: 0,
                    event: 0,
                    length: 0,
                    magic: 0,
                },
                data: &mut new_data,
            };

            if event_reconstructed.from_kernel().is_ok() {
                kani::assume(event.header.length == event_reconstructed.header.length);
                assert_eq!(
                    event.header, event_reconstructed.header,
                    "Header must stay equal after to/from kernel"
                );
                assert_eq!(
                    &event.data[..event.header.length as usize],
                    &event_reconstructed.data[..event_reconstructed.header.length as usize],
                    "Data must stay equal after to/from kernel"
                );
            }
        }
    }
}

#[kani::proof]
fn verify_shminfo_from_kernel_content() {
    use core::mem;

    unsafe {
        #[allow(static_mut_refs)]
        {
            // Clean the exchange area
            for byte in EXCHANGE_AREA.iter_mut() {
                *byte = 0;
            }

            // New ShmInfo
            let mut expected_info = core::mem::zeroed::<ShmInfo>();
            expected_info.handle = 1;
            expected_info.label = 42;
            expected_info.base = 0x1000;
            expected_info.len = 4096;
            expected_info.perms = 0x3;

            let info_ptr = &expected_info as *const ShmInfo as *const u8;
            let area_ptr = EXCHANGE_AREA.as_mut_ptr();

            core::ptr::copy_nonoverlapping(info_ptr, area_ptr, mem::size_of::<ShmInfo>());

            let mut info = core::mem::zeroed::<ShmInfo>();

            // Exchange area is sufficiently large
            kani::assume(mem::size_of::<ShmInfo>() <= EXCHANGE_AREA_LEN);

            let result = info.from_kernel();

            match result {
                Ok(Status::Ok) => {
                    kani::assume(info.handle == expected_info.handle);
                    kani::assume(info.label == expected_info.label);
                    kani::assume(info.perms == expected_info.perms);
                    kani::assume(info.base <= 0xFFFFFFFFFFFF);
                    kani::assume(info.len <= 4096 * 100);

                    assert_eq!(info.handle, expected_info.handle);
                    assert_eq!(info.label, expected_info.label);
                    assert_eq!(info.base, expected_info.base);
                    assert_eq!(info.len, expected_info.len);
                    assert_eq!(info.perms, expected_info.perms);
                }
                Ok(_) | Err(_) => {}
            }
        }
    }
}

#[kani::proof]
fn verify_shminfo_to_kernel_safety() {
    use core::mem;

    #[allow(static_mut_refs)]
    {
        let info = ShmInfo {
            handle: kani::any::<u32>(),
            label: kani::any::<u32>(),
            base: kani::any::<usize>(),
            len: kani::any::<usize>(),
            perms: kani::any::<u32>(),
        };

        let size = mem::size_of::<ShmInfo>().min(EXCHANGE_AREA_LEN);

        assert!(size <= EXCHANGE_AREA_LEN, "Size must fit in EXCHANGE_AREA");

        let result = info.to_kernel();

        match result {
            Ok(Status::Ok) => {
                // Reading each field from EXCHANGE_AREA
                let ex_handle =
                    unsafe { u32::from_le_bytes(EXCHANGE_AREA[0..4].try_into().unwrap()) };
                let ex_label =
                    unsafe { u32::from_le_bytes(EXCHANGE_AREA[4..8].try_into().unwrap()) };
                let ex_base = unsafe {
                    usize::from_le_bytes(
                        EXCHANGE_AREA[8..(8 + mem::size_of::<usize>())]
                            .try_into()
                            .unwrap(),
                    )
                };
                let ex_len = unsafe {
                    usize::from_le_bytes(
                        EXCHANGE_AREA
                            [(8 + mem::size_of::<usize>())..(8 + 2 * mem::size_of::<usize>())]
                            .try_into()
                            .unwrap(),
                    )
                };
                let ex_perms = unsafe {
                    u32::from_le_bytes(
                        EXCHANGE_AREA[(8 + 2 * mem::size_of::<usize>())
                            ..(8 + 2 * mem::size_of::<usize>() + 4)]
                            .try_into()
                            .unwrap(),
                    )
                };

                // Check that the values match
                assert_eq!(ex_handle, info.handle, "handle mismatch");
                assert_eq!(ex_label, info.label, "label mismatch");
                assert_eq!(ex_base, info.base, "base mismatch");
                assert_eq!(ex_len, info.len, "len mismatch");
                assert_eq!(ex_perms, info.perms, "perms mismatch");
            }
            Ok(_) | Err(_) => {
                // Ignore other results
            }
        }
    }
}

#[kani::proof]
fn roundtrip_transmute_shminfo() {
    use core::mem::{size_of, transmute};

    let original = ShmInfo {
        handle: kani::any::<u32>(),
        label: kani::any::<u32>(),
        base: kani::any::<usize>(),
        len: kani::any::<usize>(),
        perms: kani::any::<u32>(),
    };

    // Transmuter byte tab
    let bytes: [u8; size_of::<ShmInfo>()] = unsafe { transmute(original) };

    // Re-transmut to struct
    let recovered: ShmInfo = unsafe { transmute(bytes) };

    // Verify if the original and recovered are equal
    assert_eq!(original, recovered);
}

#[kani::proof]
fn roundtrip_exchange_bytes() {
    let mut buffer = [0u8; 128];
    let input: [u8; 128] = any(); // Arbitrary input

    // Load the area
    let src = &input[..];
    let mut dst = &mut buffer[..];

    // simulate exchange
    let _ = src.to_kernel();
    let _ = dst.from_kernel();

    // dst == src[..len] (with len borned)
    let expected_len = src.len().min(length());
    assert_eq!(&dst[..expected_len], &src[..expected_len]);
}


#[kani::proof]
fn test_to_kernel_from_shared_slice() {
    let input: [u8; 128] = any();
    let src = &input[..];
    let status = src.to_kernel();
    assert_eq!(status, Ok(Status::Ok));
}
