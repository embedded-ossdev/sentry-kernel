// SPDX-FileCopyrightText: 2025 Outpost OSS Team
// SPDX-License-Identifier: Apache-2.0

#ifndef GARBAGE_H
#define GARBAGE_H

#include <stdbool.h>
#include <stdint.h>

void get_garbage_u32(uint32_t *rng);
void get_garbage_u16(uint16_t *rng);
void get_garbage_u8(uint8_t *rng);

/** @brief Generic input scalar garbage generator */
#define get_garbage(T) _Generic((T),   \
              uint32_t*: get_garbage_u32,      \
              uint16_t*: get_garbage_u16,      \
              uint8_t*:  get_garbage_u8        \
        ) (T)


#endif/*!GARBAGE_H */
