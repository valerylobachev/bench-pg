package domain

import (
	"fmt"
)

type Customer struct {
	no int
}

func NewCustomer(no int) Customer {
	return Customer{no}
}

func (u Customer) Id() string {
	return fmt.Sprintf("CUST-%05d", u.no)
}

func (u Customer) Name() string {
	return fmt.Sprintf("Customer %05d", u.no)
}

func (u Customer) No() int {
	return u.no
}
