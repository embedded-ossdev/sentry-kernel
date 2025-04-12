# SPDX-FileCopyrightText: 2023 Ledger SAS
# SPDX-License-Identifier: Apache-2.0

test_bits = executable(
    'test_bits',
    sources: [ files('test_bits.c'), sentry_header_set_config.sources() ],
    include_directories: kernel_inc,
    c_args: [ '-DTEST_MODE=1', '-Wno-pedantic' ],
    dependencies: [cmocka_dep, mock_lib_dep ],
    native: true,
)

test('bits',
     test_bits,
     env: nomalloc,
     suite: 'ut-utils')
