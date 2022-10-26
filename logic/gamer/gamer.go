package gamer

import (
	"checkers/logic/core"
	"checkers/logic/saveLoad"
)

type Gamer struct {
	GamerID int
	Core    *core.GameCore
}

func (c Gamer) GetGamerID() int {
	return c.GamerID
}

func (c Gamer) GetField() core.Field {
	return c.Core.GetField()
}

func (c Gamer) IsTurn() bool {
	return c.Core.IsTurn(c.GamerID)
}

func (c Gamer) Move(
	from core.Coordinate, way []core.Coordinate,
) bool {
	return c.Core.Move(from, way, c.GamerID)
}

func (c Gamer) InitSave(save saveLoad.Save) {
	c.Core.InitField(save.Field)
	c.Core.InitTurnGamerID(save.TurnGamerID)
}

// GetWinner if your turn and you can't move: you lose
func (c Gamer) GetWinner() (bool, Gamer) {
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
		return true, Gamer{1, c.Core}
	}
	if numberFigures[1] == 0 || (!isCanMakeTurn[1] && c.Core.IsTurn(1)) {
		return true, Gamer{0, c.Core}
	}
	return false, Gamer{0, nil}
}
