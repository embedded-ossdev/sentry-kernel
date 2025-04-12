// SPDX-FileCopyrightText: 2025 Outpost OSS team
// SPDX-License-Identifier: Apache-2.0

#include <stdlib.h>
#include <time.h>
#include <stdbool.h>
#include <stdint.h>
#include <assert.h>
#include <sentry/ktypes.h>

static bool seeded = false;

void get_garbage_u32(uint32_t *rng)
{
    if (unlikely(seeded == false)) {
        srand(time(NULL));
    }
    assert(rng != NULL);
    *rng = (uint32_t)(rand()%__UINT32_MAX__);
}

void get_garbage_u16(uint16_t *rng)
{
    if (unlikely(seeded == false)) {
        srand(time(NULL));
    }
    assert(rng != NULL);
    *rng = (uint16_t)(rand()%__UINT16_MAX__);
}

void get_garbage_u8(uint8_t *rng)
{
    if (unlikely(seeded == false)) {
        srand(time(NULL));
    }
    assert(rng != NULL);
    *rng = (uint8_t)(rand()%__UINT8_MAX__);
}
