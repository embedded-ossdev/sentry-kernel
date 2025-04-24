// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-FileCopyrightText: 2025 H2Lab
// SPDX-License-Identifier: Apache-2.0

#include <inttypes.h>
#include <sentry/ktypes.h>
#include <sentry/arch/asm-cortex-m/layout.h>
#include <bsp/drivers/exti/exti.h>
#include <framac_entropy.h>


int main(void)
{
    uint8_t it_or_ev = Frama_C_interval_u8(0, 42);

    /* Initialize device with randomness (over-approximation of
       all content possibilities, avoid first device access ioread32()
       uninitialized-read red alarms.
    */
    memset((void*)EXTI_BASE_ADDR, Frama_C_entropy_source_u32, 0x20);

    exti_probe();
    /*
     * read registers are volative values. FramaC consider that their value
     * change each time they are read. As function may read more than one
     * register to define their behavior, the full path coverage based on the
     * full register values possibilities is the combination of successive
     * randomly generated values of the register's fields content. This
     * requires multiple pass to reach the full coverage
     */
    for (uint8_t i = 0; i < 4; ++i) {
        exti_mask_interrupt(it_or_ev);
        exti_unmask_interrupt(it_or_ev);

        exti_mask_event(it_or_ev);
        exti_unmask_event(it_or_ev);

        exti_generate_swinterrupt(it_or_ev);
        /* bit should be already set here */
        exti_generate_swinterrupt(it_or_ev);
        exti_clear_pending(it_or_ev);
    }
    return 0;
}
