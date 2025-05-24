package domain

import (
	"fmt"
)

type Vendor struct {
	no int
}

func NewVendor(no int) Vendor {
	return Vendor{no}
}

func (u Vendor) Id() string {
	return fmt.Sprintf("VEND-%05d", u.no)
}

func (u Vendor) Name() string {
	return fmt.Sprintf("Vendor %05d", u.no)
}

func (u Vendor) No() int {
	return u.no
}
