# SPDX-FileCopyrightText: 2025 H2Lab
# SPDX-License-Identifier: Apache-2.0

libproof_src = files(
    'src/entropy.c',
)

libproof_hdr = files(
    'include/framac_entropy.h',
)

libproof_inc = include_directories('include')

# XXX: int-to-pointer is the consequence of keeping u32 as addr source,
# while x86_64 is 64 bits arch. Although, we need to map in a u32 memory
# domain to emulate the MCU device here
proof_lib = static_library(
    'proof',
    sources: [
        libproof_src,
        framac_dep,
    ],
    include_directories: libproof_inc,
    install: false,
    c_args: [ global_build_args, target_arch_args ],
)

# Note: there is no 'include dir' is as this is only a mocking lib
# for kernel exported API
proof_lib_dep = declare_dependency(
    link_with: proof_lib,
    include_directories: libproof_inc,
)
