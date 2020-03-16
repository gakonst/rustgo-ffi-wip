#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct PublicKey PublicKey;

typedef struct {
  uint16_t index;
  uint32_t maximum_non_signers;
  const PublicKey *new_pubkeys;
  uintptr_t pubkeys_len;
} EpochBlockFFI;

bool verify(EpochBlockFFI epoch);