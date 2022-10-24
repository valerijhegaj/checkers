package ai

import (
	"math"

	"checkers/core"
)

func NewMinMaxMind(level int) Mind {
	return &MinMaxTree{level: level, Heuristics: NewSimpleAmount()}
}

func NewMinMaxMindV2(level int, kingCost, checkerCost float64) Mind {
	return &MinMaxTree{
		level:      level,
		Heuristics: NewAmountWithCosts(kingCost, checkerCost),
	}
}

type MinMaxTree struct {
	level int
	root  *node
	Heuristics
}

func (c *MinMaxTree) GetMove(
	field *core.Field, gamerID int,
) (core.Coordinate, []core.Coordinate) {
	c.root = &node{field: *field, gamerID: gamerID}
	c.root.createChildren(c.level*2, c.root.gamerID, c.Heuristics)
	defer func() { c.root = nil }()

	for _, move := range c.root.moves {
		if move.score == c.root.score {
			return move.from, move.way
		}
	}
	return core.Coordinate{}, nil
}

func (c *MinMaxTree) GetRandomMove(
	field *core.Field, gamerID int, random Random,
) (core.Coordinate, []core.Coordinate) {
	c.root = &node{field: *field, gamerID: gamerID}
	c.root.createChildren(2, c.root.gamerID, c.Heuristics)
	defer func() { c.root = nil }()

	if len(c.root.moves) != 0 {
		choice := random.randn(len(c.root.moves))
		return c.root.moves[choice].from, c.root.moves[choice].way
	}
	return core.Coordinate{}, nil
}

type node struct {
	moves   []*node
	field   core.Field
	from    core.Coordinate
	way     []core.Coordinate
	gamerID int
	score   float64
}

func (c *node) createChildren(
	depth int, gamerID int, heuristics Heuristics,
) float64 {
	if depth == 1 {
		c.score = heuristics.CalculateScore(gamerID, &c.field)
		return c.score
	}

	isExistMovesToEat := c.createMovesToEat()
	if isExistMovesToEat {
		c.createSuperMovesToEat()
	} else {
		c.createMovesWithoutEat()
	}

	if c.moves == nil {
		c.score = heuristics.CalculateScore(gamerID, &c.field)
		return c.score
	}

	return c.finalizeScore(depth, gamerID, heuristics)
}

func (c *node) finalizeScore(
	depth int, gamerID int, heuristics Heuristics,
) float64 {
	compare := math.Max
	c.score = -math.MaxFloat64
	if c.gamerID != gamerID {
		compare = math.Min
		c.score = math.MaxFloat64
	}

	for _, child := range c.moves {
		c.score = compare(
			child.createChildren(depth-1, gamerID, heuristics), c.score,
		)
		child.moves = nil
	}
	return c.score
}

func (c *node) createMovesWithoutEat() {
	coordinates, figures := c.field.GetFigures(c.gamerID)

	for i, figure := range figures {
		from := coordinates[i]
		moves := figure.GetAvailableMoves(&c.field, from)
		for _, to := range moves {
			childField := c.field.GetCopy()
			childField.Move(from, to)
			c.moves = append(
				c.moves, &node{
					field: childField, from: from,
					way: []core.Coordinate{to}, gamerID: c.gamerID ^ 1,
				},
			)
		}
	}
}

func (c *node) createSuperMovesToEat() {
	for i := 0; i < len(c.moves); i++ {
		move := c.moves[i]
		from := move.way[len(move.way)-1]
		figure := move.field.At(from)
		moves := figure.GetAvailableMovesToEat(&move.field, from)

		for _, to := range moves {
			copyField := move.field.GetCopy()

			figure := copyField.At(from)
			figure.Move(&copyField, from, []core.Coordinate{to})

			way := make([]core.Coordinate, len(move.way)+1)
			copy(way, move.way)
			way[len(move.way)] = to

			c.moves = append(
				c.moves, &node{
					field: copyField, from: move.from, way: way,
					gamerID: c.gamerID ^ 1,
				},
			)
		}
	}
}

func (c *node) createMovesToEat() bool {
	isWasMovesToEat := false
	coordinates, figures := c.field.GetFigures(c.gamerID)

	for i, figure := range figures {
		from := coordinates[i]
		moves := figure.GetAvailableMovesToEat(&c.field, from)
		if len(moves) != 0 {
			isWasMovesToEat = true
		}

		for _, to := range moves {
			copyField := c.field.GetCopy()
			figure := copyField.At(from)
			figure.Move(&copyField, from, []core.Coordinate{to})
			c.moves = append(
				c.moves, &node{
					field: copyField, from: from, way: []core.Coordinate{to},
					gamerID: c.gamerID ^ 1,
				},
			)
		}
	}

	return isWasMovesToEat
}
