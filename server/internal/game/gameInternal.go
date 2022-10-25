package game

import "checkers/logic/core"

type Game interface {
	Move(gamerID int, from core.Coordinate, way []core.Coordinate) bool
	GetGame()
	OnChangeGame(func([]byte))
}
