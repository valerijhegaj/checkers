package ai

import (
	"checkers/core"
	"checkers/gamer"
)

func NewBot(level int) Ai {
	var ai Ai
	if level == 0 {
		ai.Mind = NewRandomMoves()
	} else if level <= 4 {
		ai.Mind = NewMinMaxMind(level)
	} else {
		ai.Mind = NewMinMaxMindV2(level-1, 3, 1)
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
