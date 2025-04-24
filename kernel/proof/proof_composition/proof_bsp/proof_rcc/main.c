// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-FileCopyrightText: 2025 H2Lab
// SPDX-License-Identifier: Apache-2.0

#include <inttypes.h>
#include <sentry/ktypes.h>
#include <bsp/drivers/clk/rcc.h>
#include <sentry/arch/asm-cortex-m/layout.h>
#include <framac_entropy.h>

volatile uint32_t regvalue;

int main(void)
{
    uint8_t busid = Frama_C_interval_8(0, 0xff);
    uint32_t clkid = Frama_C_interval_32(0, 0x0a0b0c0dUL);
    uint8_t flag = Frama_C_interval_8(0, 42);
    volatile uint8_t val = 0;
    volatile uint8_t clkmsk = 0;
    uint32_t clk;
    uint32_t busclk;

    /* Initialize device with randomness (over-approximation of
       all content possibilities, avoid first device access ioread32()
       uninitialized-read red alarms.
    */
    memset((void*)RCC_BASE_ADDR, Frama_C_entropy_source_u32, 0x40);

    /* as registers are volatile values, their content varies from one test to
     * another.... At a time, they will pass the framaC test
     */
    rcc_probe();

    rcc_get_core_frequency();

    rcc_enable(busid, clkid, flag);
    rcc_disable(busid, clkid, flag);
    rcc_get_bus_clock(busid, &busclk);
    rcc_get_bus_clock(busid, NULL);
    rcc_mux_select_clock_source(clkid, clkmsk, val);

    return 0;
}
