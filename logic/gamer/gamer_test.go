package gamer

import (
	"testing"

	core2 "checkers/logic/core"
)

func TestGamer_GetField(t *testing.T) {
	var c core2.GameCore
	field := core2.NewField()
	field.BordersRight = core2.Coordinate{-1, -1}
	field.BordersLeft = core2.Coordinate{-2, -2}
	ptr := core2.Coordinate{-1, -1}
	field.Put(ptr, core2.TestFigure{0})
	c.InitField(field)

	gamer := PrimitiveGamer{0, &c}
	fieldInCore := gamer.GetField()
	if field.BordersRight != fieldInCore.BordersRight ||
		len(field.Figures) != 1 ||
		field.At(ptr) != fieldInCore.At(ptr) {
		t.Error("Not that field in core")
	}
}

func TestGamer_IsTurn(t *testing.T) {
	var c core2.GameCore
	c.InitTurnGamerID(0)
	if !c.IsTurn(0) {
		t.Error("((")
	}
}

func TestGamer_Move(t *testing.T) {
	var c core2.GameCore
	c.InitField(core2.NewStandard8x8Field())
	var gamer [2]PrimitiveGamer
	gamer[0] = PrimitiveGamer{0, &c}
	gamer[1] = PrimitiveGamer{1, &c}
	if gamer[0].Move(
		core2.Coordinate{2, 1}, []core2.Coordinate{{3, 0}},
	) {
		t.Error()
	}
	if gamer[1].Move(
		core2.Coordinate{2, 0}, []core2.Coordinate{{3, 1}},
	) {
		t.Error(c)
	}
	if gamer[1].Move(
		core2.Coordinate{2, 1}, []core2.Coordinate{{3, 0}},
	) {
		t.Error()
	}
	if gamer[1].Move(
		core2.Coordinate{2, 0}, []core2.Coordinate{{3, 1}},
	) {
		t.Error()
	}
	if !gamer[0].Move(
		core2.Coordinate{2, 0}, []core2.Coordinate{{3, 1}},
	) {
		t.Error()
	}
	if c.Move(core2.Coordinate{2, 2}, []core2.Coordinate{{3, 3}}, 0) {
		t.Error()
	}
	if c.Move(core2.Coordinate{2, 2}, []core2.Coordinate{{3, 3}}, 1) {
		t.Error()
	}
	if !c.Move(core2.Coordinate{5, 1}, []core2.Coordinate{{4, 2}}, 1) {
		t.Error()
	}
}

func TestGamer_GetWinner(t *testing.T) {

}

func TestGamer_InitSave(t *testing.T) {

}
