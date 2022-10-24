package gamer

import (
	core2 "checkers/logic/core"
	"checkers/logic/saveLoad"
)

type Gamer struct {
	GamerID int
	Core    *core2.GameCore
}

func (c Gamer) GetGamerID() int {
	return c.GamerID
}

func (c Gamer) GetField() core2.Field {
	return c.Core.GetField()
}

func (c Gamer) IsTurn() bool {
	return c.Core.IsTurn(c.GamerID)
}

func (c Gamer) Move(
	from core2.Coordinate, way []core2.Coordinate,
) bool {
	return c.Core.Move(from, way, c.GamerID)
}

func (c Gamer) InitSave(save saveLoad.Save) {
	c.Core.InitField(save.Field)
	c.Core.InitTurnGamerID(save.TurnGamerId)
}

// GetWinner if your turn and you can't move: you lose
func (c Gamer) GetWinner() (bool, Gamer) {
	field := c.GetField()
	var isCanMakeTurn [2]bool
	var numberFigures [2]int
	for from, figure := range field.Figures {
		numberFigures[figure.GetOwnerID()]++
		moves := figure.GetAvailableMoves(&field, from)
		if moves != nil {
			isCanMakeTurn[figure.GetOwnerID()] = true
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
