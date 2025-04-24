// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-FileCopyrightText: 2025 H2Lab
// SPDX-License-Identifier: Apache-2.0

#include <stdbool.h>
#include <stdint.h>
#include <framac_entropy.h>
#include <sentry/zlib/crypto.h>
#include <sentry/zlib/math.h>
#include <sentry/zlib/sort.h>
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

void kernel_zlib(void)
{
    uint32_t res;
    char buf[128];
    /* calling kernel zlib */
    uint32_t init = Frama_C_entropy_source_u32;
    res = crc32(NULL, Frama_C_entropy_source_u32, init);
    /*@ assert (res == init); */
    memset(buf, Frama_C_entropy_source_u8, 128);
    crc32(buf, Frama_C_interval_u32(0,128), Frama_C_entropy_source_u32);
}
