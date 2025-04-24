// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-FileCopyrightText: 2025 H2Lab
// SPDX-License-Identifier: Apache-2.0

#include <stdbool.h>
#include <stdint.h>
#include <framac_entropy.h>
#include <sentry/arch/asm-generic/interrupt.h>

/** TODO: expose sentry_xxx of string.h instead of using externs here */

/**
 * NOTE: in non-proof mode, these symbols are aliased to corresponding compiler
 * builtins, and as such resolvable by the compiler.
 * Nonetheless, we want here to check their implementation, and thus be able
 * to explicitly call them.
 * These symbols are a part of the Sentry zlib
 */
void   *sentry_memcpy(void * restrict dest, const void* restrict src, size_t n);
void   *sentry_memset(void *s, int c, unsigned int n);
size_t sentry_strnlen(const char *s, size_t maxlen);

/**
 * Here we call all the arch-relative API, in the way initial early init do.
 * We cover overall API (arch/ headers) so that EVA is able to cover all libarch.
 * Then, we use WP to demonstrate that all subprogram contracts are true, and to
 * analyse and validate all border effects.
 */
void kernel_arch(void)
{
    uint32_t prio;

    system_reset();
    interrupt_init();
    prio = nvig_get_prioritygrouping();
    nvic_set_prioritygrouping(prio);

    uint16_t irq = Frama_C_interval_u16(0, NUM_IRQS-1);
    nvic_enableirq(irq);
    nvic_disableirq(irq);
    nvic_get_pendingirq(irq);
    nvic_set_pendingirq(irq);
    nvic_clear_pendingirq(irq);
    wait_for_interrupt();
    wait_for_event();
    notify_event();
    interrupt_disable();
    interrupt_enable();
}
