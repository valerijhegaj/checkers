package ai

import (
	"math/rand"

	core2 "checkers/logic/core"
)

func NewRandomMoves() RandomMoves {
	return RandomMoves{
		psevdoRandom{}, MinMaxTree{2, nil, NewSimpleAmount()},
	}
}

type RandomMoves struct {
	Random
	body MinMaxTree
}

func (c RandomMoves) GetMove(
	field *core2.Field, gamerID int,
) (core2.Coordinate, []core2.Coordinate) {
	return c.body.GetRandomMove(field, gamerID, c.Random)
}

type Random interface {
	randlr(l, t int) int
	randn(n int) int
}

type psevdoRandom struct{}

func (c psevdoRandom) randlr(l, r int) int {
	return rand.Intn(r-l) + l
}

func (c psevdoRandom) randn(n int) int {
	return rand.Intn(n)
}
