// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-License-Identifier: Apache-2.0

#include <sys/mman.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include <stdio.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <setjmp.h>
#include <cmocka.h>

#include <sentry/arch/asm-cortex-m/layout.h>
#include <bsp/drivers/exti/exti.h>
#include <sentry/ktypes.h>
#include "stm32-exti-dt.h"

/* mocked external APIs (out of EXTI module)*/
kstatus_t mgr_mm_map_kdev(
        uint32_t addr __attribute__((unused)),
        size_t size __attribute__((unused))
    )
{
    return K_STATUS_OKAY;
}

kstatus_t mgr_mm_unmap_kdev(void) {
    return K_STATUS_OKAY;
}

/* Setup and teardown */
typedef struct {
    void *map;
} teststate_t;

/* For EXTI we need to allocate an emulated device in a memory area that match the
 * defined EXTI_BASE_ADDR. This memory area is also set with reset-time values (0x0)
 * TODO: such a setup should be unified for all bsp test suite, in a local library
 * to avoid duplication.
 */
static int group_setup (void** state) {
    teststate_t *exti_test = malloc(sizeof(teststate_t));
    assert(exti_test != NULL);

    exti_test->map = mmap((void*)EXTI_BASE_ADDR, 4096UL, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    if (exti_test->map != (void*)EXTI_BASE_ADDR) {
        if ((size_t)exti_test->map > EXTI_BASE_ADDR) {
            printf("Unable to map device to correct address 0x%h\n", EXTI_BASE_ADDR);
            goto err;
        }
        /* the kernel may have align the mapping on 4k size */
        if ((size_t)exti_test->map + 4096 < (EXTI_BASE_ADDR + 0x400)) {
            printf("Unable to map device to correct address 0x%h\n", EXTI_BASE_ADDR);
            printf("Page alignment problem not resolvable\n");
            printf("address is 0x%h\n\n", exti_test->map);
            printf(strerror(errno));
            goto err;
        }
        /* page-alignment do include the device*/
    }
    if (exti_test->map == (void*)-1) {
        printf("mmap has failed ! %s\n", strerror(errno));
        goto err;
    }
    /*
     * push reset values (0x0 for EXTI= in the device. Using standard memory mapping size of device,
     * portable to any STM32 for offset
     */
    for (uint8_t *addr = (uint8_t*)EXTI_BASE_ADDR; addr < (uint8_t*)exti_test->map+0x400; ++addr) {
        *addr = 0x0;
    }
    *state = exti_test;
    return 0;
err:
    return 1;
}

static int group_teardown(void** state) {
    teststate_t* exti_test = *state;
    munmap(exti_test->map, 4096UL);
    free(exti_test);
    return 0;
}

/* A test case that does nothing and succeeds. */
static void null_test_success(void **state) {
    (void) state; /* unused */
}
 
static void test_probe(void **state) {
    assert_int_equal(exti_probe(), K_STATUS_OKAY);
}
 

int main(void) {
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_probe),
    };
 
    return cmocka_run_group_tests(tests, group_setup, group_teardown);
}