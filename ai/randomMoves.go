package ai

import (
	"math/rand"

	"checkers/core"
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
	field *core.Field, gamerID int,
) (core.Coordinate, []core.Coordinate) {
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
