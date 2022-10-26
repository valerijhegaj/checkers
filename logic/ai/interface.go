package ai

import (
	"checkers/logic/core"
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
	core.Coordinate, []core.Coordinate,
) {
	field := gamer.GetField()
	from, way := c.GetMove(&field, gamer.GetGamerID())

	gamer.Move(from, way)
	return from, way
}

type Mind interface {
	GetMove(field *core.Field, gamerID int) (
		core.Coordinate, []core.Coordinate,
	)
}
