package domain

import (
	"fmt"
)

type Material struct {
	no int
}

func NewMaterial(no int) Material {
	return Material{no}
}

func (u Material) Id() string {
	return fmt.Sprintf("MAT-%05d", u.no)
}

func (u Material) Name() string {
	return fmt.Sprintf("Material %05d", u.no)
}

func (u Material) No() int {
	return u.no
}
