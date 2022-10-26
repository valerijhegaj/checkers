package ai

import (
	core2 "checkers/logic/core"
	"checkers/logic/gamer"
)

func NewBot(level int) Ai {
	var ai Ai
	if level == 0 {
		ai.Mind = NewRandomMoves()
	} else {
		ai.Mind = NewMinMaxMindV2(level, 3, 1)
	}
	return ai
}

type Ai struct {
	Mind
}

func (c *Ai) Move(gamer gamer.Gamer) (
	core2.Coordinate, []core2.Coordinate,
) {
	field := gamer.GetField()
	from, way := c.GetMove(&field, gamer.GetGamerID())

	gamer.Move(from, way)
	return from, way
}

type Mind interface {
	GetMove(field *core2.Field, gamerID int) (
		core2.Coordinate, []core2.Coordinate,
	)
}
