// SPDX-FileCopyrightText: 2025 H2Lab
// SPDX-License-Identifier: Apache-2.0

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

/*
 * Frama-C entropy sources. This variables have their value changed each time
 * they are read
 */
extern volatile int Frama_C_entropy_source_int __attribute__((unused));
extern volatile uint8_t Frama_C_entropy_source_u8 __attribute__((unused));
extern volatile uint16_t Frama_C_entropy_source_u16 __attribute__((unused));
extern volatile uint16_t Frama_C_entropy_source_u32 __attribute__((unused));
extern volatile uint8_t Frama_C_entropy_source_bool __attribute__((unused));

/*@
  @ assigns Frama_C_entropy_source_int \from Frama_C_entropy_source_int;
  */
 int Frama_C_interval_int(int min, int max);

/*@
  @ assigns Frama_C_entropy_source_u8 \from Frama_C_entropy_source_u8;
  */
 uint8_t Frama_C_interval_u8(uint8_t min, uint8_t max);

 /*@
  @ assigns Frama_C_entropy_source_u16 \from Frama_C_entropy_source_u16;
  */
uint16_t Frama_C_interval_u16(uint16_t min, uint16_t max);

/*@
  @ assigns Frama_C_entropy_source_u16 \from Frama_C_entropy_source_u16;
  */
 uint16_t Frama_C_interval_u16(uint16_t min, uint16_t max);

 /*@
  @ assigns Frama_C_entropy_source_u32 \from Frama_C_entropy_source_u32;
  */
uint32_t Frama_C_interval_u32(uint32_t min, uint32_t max);

/*@
  @ assigns Frama_C_entropy_source_bool \from Frama_C_entropy_source_bool;
  */
bool Frama_C_interval_bool(void);

/*@
  @ requires \valid(data+(0..data_len-1));
  @ assigns data[0..data_len-1];
  */
static inline void Frama_C_set_random(uint8_t *data, size_t data_len)
{
  /*@
    @ loop invariant 0 <= len <= data_len;
    @ loop assigns data[len];
    @ loop variant data_len - len;
    @*/
  for (size_t len = 0; len < data_len; len++) {
    data[len] = Frama_C_interval_u8(0, 0xff);
  }
}
