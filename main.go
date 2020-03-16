package main

/* 
#cgo LDFLAGS: -L./target/release/ -lffi_example
#include "verify.h"
*/
import "C"

import (
	"unsafe"
)

type Pubkey struct {
	X int
}

func main() {
	pubkeys := []Pubkey{
		Pubkey { X: 99 },
		Pubkey { X: 128 },
	}
	header := C.EpochBlockFFI {
		index: 3,
		maximum_non_signers: 1,
		new_pubkeys: (*C.PublicKey)(unsafe.Pointer(&pubkeys[0])),
		pubkeys_len: C.ulong(len(pubkeys)),
	}
	C.verify(header)
}