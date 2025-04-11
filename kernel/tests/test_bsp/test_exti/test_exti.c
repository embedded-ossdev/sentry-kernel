// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-License-Identifier: Apache-2.0

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
 
static void test_probe(void **state) {
    assert_int_equal(exti_probe(), K_STATUS_OKAY);
}
 

int main(void) {
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_probe),
    };
 
    return cmocka_run_group_tests(tests, NULL, NULL);
}
