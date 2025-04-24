// SPDX-FileCopyrightText: 2023 Ledger SAS
// SPDX-FileCopyrightText: 2025 H2Lab
// SPDX-License-Identifier: Apache-2.0

#include <stdbool.h>
#include <stdint.h>
#include <framac_entropy.h>

#include <sentry/managers/task.h>

extern task_meta_t __task_meta_table[CONFIG_MAX_TASKS];

static void Frama_C_task_init_meta(void)
{
    uint32_t *target = (uint32_t*)&__task_meta_table[0];
    for (uint32_t offset = 0; offset < (sizeof(task_meta_t)*CONFIG_MAX_TASKS) / sizeof(uint32_t); ++offset)
    {
        target[offset] = ghost_Frama_C_get_random_u32();
    }
}

void Reset_Handler(void);

void kernel_entrypoint(void)
{
    /** INFO: inject garbage in metadata. This structure is build system forged.
     * This allows to:
     * 1. avoid uninitialized error from frama-C
     * 2. generate potential invalid inputs values from corrupted build system
     */
    Frama_C_task_init_meta();
    /* calling kernel entrypoint */
    Reset_Handler();
    //_entrypoint();
}
