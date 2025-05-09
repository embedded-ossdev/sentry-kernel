// SPDX-FileCopyrightText: 2025 ANSSI
// SPDX-License-Identifier: Apache-2.0


#![cfg_attr(not(feature = "std"), no_std)]
// here is the libtest Rust implementation, to be added
// The FFI-C interface to export only hold the following sybols:
//
//    test_yield()
//    test_handle()
//    test_signal()
//    test_ipc()
//    test_random()
//    test_cycles()
//    test_sleep()
//    test_gpio()
//    test_map()
//    test_shm()
//    test_dma()
//    test_irq()
//
// their implementation should be fully Rustic, using the libUAPI
// Rustic interface only
