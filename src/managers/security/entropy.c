// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-License-Identifier: Apache-2.0

#include <stddef.h>
#if defined(CONFIG_HAS_RNG)
#include <bsp/drivers/rng/rng.h>
#endif
#include <sentry/crypto/pgc32.h>
#include <sentry/ktypes.h>
#include <sentry/managers/debug.h>

/* can be initialized once only */
static uint32_t seed;
/**
 * @file Entropy source management
 */

 /**
  * @brief initialize Sentry entropy source
  *
  * The entropy source may vary based on the underlaying platform
  * This function is a security-manager private function called by mgr_security_init()
  *
  */
kstatus_t mgr_security_entropy_init(void)
{
    kstatus_t status;
#if defined(CONFIG_HAS_RNG)
    printk("HW RNG supported, initializing HW entropy backend... ");
    status = rng_probe();
    if (unlikely(status != K_STATUS_OKAY)) {
        goto end;
    }
    status = rng_get(&seed);
    if (unlikely(status != K_STATUS_OKAY)) {
        printk("failed!\n")
    } else {
        printk("done.\n")
    }
#else
    printk("HW RNG not supported, initializing SW entropy backend.\n");
    /* Here we use PGC32 has this is the lonely function we have to generate random
     sequence in SW mode. To be replaced by another pseudo-random (or higher security
     level) random source */
    /**
     * XXX: define the way to reseed the pgc32 at boot time when no HW-backed RNG
     * source exist. By now using default hardocded state/sequence
     * pgc32_seed(state, sequence);
     */
    seed = pcg32();
    status = K_STATUS_OKAY;
#endif
end:
    return status;
}

/**
  * @brief generate a new random value from the initialized entropy source
  *
  */
kstatus_t mgr_security_entropy_generate(uint32_t *seed)
{
    kstatus_t status = K_ERROR_INVPARAM;
    if (unlikely(seed == NULL)) {
        goto end;
    }
#if CONFIG_HAS_RNG
    status = rng_get(seed);
#else
    *seed = pcg32();
    status = K_STATUS_OKAY;
#endif
end:
    return status;
}
