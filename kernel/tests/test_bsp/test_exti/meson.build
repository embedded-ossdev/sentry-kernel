# SPDX-FileCopyrightText: 2023 Ledger SAS
# SPDX-FileCopyrightText: 2025 Outpost OSS Team
# SPDX-License-Identifier: Apache-2.0


test_bsp_exti_c = executable(
    'test_bsp_exti_c',
    sources: [
        files(
            'test_exti.c',
            join_paths(meson.project_source_root(), 'kernel/src/drivers/exti/stm32-exti.c'),
        ),
        exti_h, stm32_exti_dtsgen_h, stm32_exti_dtsgen_c,
        sentry_header_set_config.sources(),
    ],
    include_directories: kernel_inc,
    c_args: [
        '-I'+join_paths(meson.project_build_root(), 'kernel/src/drivers/exti'),
        global_build_args,
        '-DTEST_MODE=1',
    ],
    dependencies: [ cmocka_dep, cmsis_dep, mock_lib_dep ],
    native: true,
)

test('exti-c',
     test_bsp_exti_c,
     env: nomalloc,
     suite: 'ut-bsp')

test_bsp_exti = executable(
    'test_bsp_exti',
    sources: [
        files(
            'test_exti.cpp',
            join_paths(meson.project_source_root(), 'kernel/src/drivers/exti/stm32-exti.c'),
        ),
        exti_h, stm32_exti_dtsgen_h, stm32_exti_dtsgen_c,
        sentry_header_set_config.sources(),
    ],
    include_directories: kernel_inc,
    override_options: ['cpp_std=gnu++20'],
    cpp_args: [
        '-I'+join_paths(meson.project_build_root(), 'kernel/src/drivers/exti'),
        '-include', join_paths(fs.parent(kconfig_h),fs.name(kconfig_h)),
        '-DTEST_MODE=1',
    ],
    c_args: [
        '-I'+join_paths(meson.project_build_root(), 'kernel/src/drivers/exti'),
        global_build_args,
        '-DTEST_MODE=1',
    ],
    dependencies: [ gtest_main, cmsis_dep ],
    link_language: 'cpp',
    native: true,
)

test('exti',
     test_bsp_exti,
     env: nomalloc,
     suite: 'ut-bsp')
