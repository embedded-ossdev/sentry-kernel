# SPDX-FileCopyrightText: 2023 Ledger SAS
# SPDX-License-Identifier: Apache-2.0

test_printk = executable(
    'test_printk',
    sources: [
        files('test_printk.c'),
        sentry_header_set_config.sources(),
        join_paths(meson.project_source_root(), 'kernel/src/managers/debug/log.c'),
        join_paths(meson.project_source_root(), 'kernel/src/managers/debug/log_lexer.c'),
    ],
    include_directories: kernel_inc,
    c_args: [
        '-DTEST_MODE=1',
        '-Wno-pedantic',
        '-DCONFIG_BUILD_TARGET_DEBUG=1',
        '-Wno-format-truncation', # required as %s, NULL is tested
        '-include', join_paths(fs.parent(kconfig_h),fs.name(kconfig_h)),
    ],
    dependencies: [cmocka_dep, mock_lib_dep ],
    native: true,
)

test('printk',
     test_printk,
     env: nomalloc,
     suite: 'ut-managers')
