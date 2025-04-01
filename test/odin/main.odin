package test

import a "base:intrinsics"
import "core:fmt"

CONST_VAL :: 42

My_Int :: int

My_Int_Distinct :: distinct int

main :: proc() -> int {

	fmt.printf("hello, world")

	entity := Entity {
		field = 12,
	}

	return 1
}

test_method :: proc(entity: Entity) -> u16 {
	return entity.field
}

Entity :: struct {
	field: u16,
}
