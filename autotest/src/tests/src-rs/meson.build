# SPDX-FileCopyrightText: 2025 H2Lab OSS Team
# SPDX-License-Identifier: Apache-2.0


# Rust implementation of the autotest test suite.
# See ../include header files for FFI-C conformance
# Note: ideally, write a full rust implementation, with
# a FFI-C modules to handle the lonely unsafe parts for C
# exports
testlib_rust_libfile = files('lib.rs')

testlib_rust_sources = files([
  # adding testsuites Rust implementations
])

# autotest test lib, in Rust
autotest_test_lib = static_library('testlib',
    sources: [
        testlib_rust_sources,
    ],
    rust_abi: 'c',
    rust_args: global_rust_build_args,
    override_options: [uapi_rust_std],
    extra_files: uapi_modules,
    install: false
)
