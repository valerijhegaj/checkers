package gamer

import (
	"sync"

	"checkers/logic/core"
	"checkers/logic/saveLoad"
)

func NewGamers() [2]Gamer {
	var gamers [2]Gamer
	gamerPrimitives := NewGuardedGamers()
	gamers[0] = &gamerPrimitives[0]
	gamers[1] = &gamerPrimitives[1]
	return gamers
}

type Gamer interface {
	GetGamerID() int
	GetField() core.Field
	IsTurn() bool
	Move(from core.Coordinate, way []core.Coordinate) bool
	InitSave(save saveLoad.Save)
	GetWinner() (bool, PrimitiveGamer)
}

//--------------------------------------------------------------------

func NewPrimitiveGamers() [2]PrimitiveGamer {
	var c core.GameCore
	gamers := [2]PrimitiveGamer{{0, &c}, {1, &c}}
	return gamers
}

type PrimitiveGamer struct {
	GamerID int
	Core    *core.GameCore
}

func (c *PrimitiveGamer) GetGamerID() int {
	return c.GamerID
}

func (c *PrimitiveGamer) GetField() core.Field {
	return c.Core.GetField()
}

func (c *PrimitiveGamer) IsTurn() bool {
	return c.Core.IsTurn(c.GamerID)
}

func (c *PrimitiveGamer) Move(
	from core.Coordinate, way []core.Coordinate,
) bool {
	return c.Core.Move(from, way, c.GamerID)
}

func (c *PrimitiveGamer) InitSave(save saveLoad.Save) {
	c.Core.InitField(save.Field)
	c.Core.InitTurnGamerID(save.TurnGamerID)
}

// GetWinner if your turn and you can't move: you lose
func (c *PrimitiveGamer) GetWinner() (bool, PrimitiveGamer) {
	field := c.GetField()
	var isCanMakeTurn [2]bool
	var numberFigures [2]int
	for i := 0; i < 2; i++ {
		coordinates, figures := field.GetFigures(i)
		for j, figure := range figures {
			numberFigures[i]++
			moves := figure.GetAvailableMoves(&field, coordinates[j])
			if moves != nil {
				isCanMakeTurn[i] = true
			}
		}
	}

	if numberFigures[0] == 0 || (!isCanMakeTurn[0] && c.Core.IsTurn(0)) {
		return true, PrimitiveGamer{1, c.Core}
	}
	if numberFigures[1] == 0 || (!isCanMakeTurn[1] && c.Core.IsTurn(1)) {
		return true, PrimitiveGamer{0, c.Core}
	}
	return false, PrimitiveGamer{0, nil}
}

//--------------------------------------------------------------------

func NewGuardedGamers() [2]GuardedGamer {
	var mu sync.RWMutex
	var gamers [2]GuardedGamer
	primitiveGamers := NewPrimitiveGamers()

	gamers[0].mu = &mu
	gamers[0].PrimitiveGamer = primitiveGamers[0]
	gamers[1].mu = &mu
	gamers[1].PrimitiveGamer = primitiveGamers[1]

	return gamers
}

type GuardedGamer struct {
	mu *sync.RWMutex
	PrimitiveGamer
}

func (c *GuardedGamer) GetGamerID() int {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return c.PrimitiveGamer.GetGamerID()
}

func (c *GuardedGamer) GetField() core.Field {
	c.mu.RLock()
	defer c.mu.RUnlock()
	field := c.PrimitiveGamer.GetField()
	return field.GetCopy()
}

func (c *GuardedGamer) IsTurn() bool {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return c.PrimitiveGamer.IsTurn()
}

func (c *GuardedGamer) Move(
	from core.Coordinate, way []core.Coordinate,
) bool {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.PrimitiveGamer.Move(from, way)
}

func (c *GuardedGamer) InitSave(save saveLoad.Save) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.PrimitiveGamer.InitSave(save)
}

// GetWinner if your turn and you can't move: you lose
func (c *GuardedGamer) GetWinner() (bool, PrimitiveGamer) {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return c.PrimitiveGamer.GetWinner()
}
