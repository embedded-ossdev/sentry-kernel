# SPDX-FileCopyrightText: 2023 Ledger SAS
# SPDX-License-Identifier: Apache-2.0

managers_source_set = ssmod.source_set()
managers_private_header_set = ssmod.source_set()
managers_private_gen_source_set = ssmod.source_set()
managers_private_gen_header_set = ssmod.source_set()

# Sentry managers list
subdir('security')
subdir('task')
subdir('time')
subdir('io')
if kconfig_data.get('CONFIG_BUILD_TARGET_RELEASE', 0) == 0
subdir('debug')
endif
subdir('clock')
subdir('interrupt')
subdir('memory')
subdir('device')
if kconfig_data.get('CONFIG_HAS_GPDMA', 0) == 1
subdir('dma')
endif

managers_source_set_config = managers_source_set.apply(kconfig_data, strict: false)
managers_private_header_set_config = managers_private_header_set.apply(kconfig_data, strict: false)
managers_private_gen_source_set_config = managers_private_gen_source_set.apply(kconfig_data, strict: false)
managers_private_gen_header_set_config = managers_private_gen_header_set.apply(kconfig_data, strict: false)

sentry_managers_lib = static_library('sentry_managers',
    sources: [
        managers_source_set_config.sources(),
        managers_private_header_set_config.sources(),
        managers_private_gen_source_set_config.sources(),
        managers_private_gen_header_set_config.sources(),
        managers_header_set_config.sources(),
    ],
    include_directories: kernel_inc,
    c_args: [ target_arch_args, global_build_args, activated_lib_hardening_cflags ],
    install: false,
    dependencies: [
        sentry_arch_dep,
        sentry_zlib_dep,
        managers_source_set_config.dependencies(),
        external_deps],
)

sentry_managers_dep = declare_dependency(
    link_with: sentry_managers_lib,
    sources: managers_header_set_config.sources(),
    include_directories: kernel_inc,
)
