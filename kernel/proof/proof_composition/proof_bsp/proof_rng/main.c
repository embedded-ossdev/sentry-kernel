// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-FileCopyrightText: 2025 H2Lab
// SPDX-License-Identifier: Apache-2.0

#include <inttypes.h>
#include <sentry/ktypes.h>
#include <bsp/drivers/rng/rng.h>
#include <sentry/arch/asm-cortex-m/layout.h>
#include <framac_entropy.h>

volatile uint32_t regvalue;

int main(void)
{
    uint32_t rng;

    /* Initialize device with randomness (over-approximation of
       all content possibilities, avoid first device access ioread32()
       uninitialized-read red alarms.
    */
    memset((void*)RNG_BASE_ADDR, Frama_C_entropy_source_u32, 0x20);

    /* as registers are volatile values, their content varies from one test to
     * another.... At a time, they will pass the framaC test
     */
    rng_probe();

    rng_get(&rng);
    rng_get(&rng);
    rng_get(&rng);
    rng_get(NULL);
    rng_get(&rng);

    return 0;
}
