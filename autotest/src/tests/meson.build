# SPDX-FileCopyrightText: 2023 - 2025 Ledger SAS
# SPDX-FileCopyrightText: 2025 H2Lab OSS Team
# SPDX-License-Identifier: Apache-2.0

# test library

# the test library required device config when tested
devices_template_h = files(['devices-dt.h.in'])
devices_dtsgen_h = dtsgen.process(devices_template_h)

shm_template_h = files(['shms-dt.h.in'])
shm_dtsgen_h = dtsgen.process(shm_template_h)

testlib_sourceset = ssmod.source_set()

testlib_sourceset.add(when: 'CONFIG_TEST_DEVICES', if_true: devices_dtsgen_h)
testlib_sourceset.add(when: 'CONFIG_TEST_SHM', if_true: shm_dtsgen_h)

testlib_sourceset.add(when: 'CONFIG_TEST_CYCLES', if_true: files('test_cycles.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_DMA', if_true: files('test_dma.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_GPIO', if_true: files('test_gpio.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_HANDLES', if_true: files('test_handle.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_IPC', if_true: files('test_ipc.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_IRQ', if_true: files('test_irq.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_DEVICES', if_true: files('test_map.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_RANDOM', if_true: files('test_random.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_SHM', if_true: files('test_shm.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_SIGNALS', if_true: files('test_signal.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_SLEEP', if_true: files('test_sleep.c'))
testlib_sourceset.add(when: 'CONFIG_TEST_YIELD', if_true: files('test_yield.c'))

testlib_sourceset_config = testlib_sourceset.apply(kconfig_data, strict: false)

# here we hold both source and headers files in the same dir
testlib_inc = include_directories('.')

# test library, to be used as C implementation of the test suite interface
# depends on dts-based devices and shm listing
autotest_test_lib = static_library('testlib',
    sources: [
        testlib_sourceset_config.sources(),
    ],
    include_directories: [ testlib_inc, autotest_inc, uapi_inc ],
    c_args: [ target_arch_args, global_build_args, activated_lib_hardening_cflags ],
    install: false,
    dependencies: [testlib_sourceset_config.dependencies(), external_deps ],
)

autotest_test_dep = declare_dependency(
    link_with: autotest_test_lib,
    include_directories: testlib_inc,
)
