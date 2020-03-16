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

type Header struct {
	index uint16
	non_signers uint32
	pubkeys *Pubkey
	pubkeys_len int
}

func main() {
	pubkeys := []Pubkey{
		Pubkey { X: 1 },
		Pubkey { X: 2 },
		Pubkey { X: 3 },
	}
	header := Header {
		index: 3,
		non_signers: 5,
		pubkeys: &pubkeys[0],
		pubkeys_len: len(pubkeys),
	}
	h1 := (*C.EpochBlockFFI)(unsafe.Pointer(&header))
	C.verify(h1)
}