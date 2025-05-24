package domain

import (
	"fmt"
)

type User struct {
	no int
}

func NewUser(no int) User {
	return User{no}
}
func (u User) Id() string {
	return fmt.Sprintf("USER-%05d", u.no)
}

func (u User) No() int {
	return u.no
}
