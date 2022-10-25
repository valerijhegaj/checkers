package core

type TestFigure struct {
	OwnerId int
}

func (c TestFigure) ToString() string {
	return "TestFigure"
}

func (c TestFigure) GetOwnerID() int {
	return c.OwnerId
}

func (c TestFigure) Move(
	desk *Field, from Coordinate, way []Coordinate,
) (bool, Coordinate) {
	return false, from
}

func (c TestFigure) IsMoveOne(desk *Field, from, to Coordinate) (
	bool, Coordinate,
) {
	return false, desk.BordersLeft
}

func (c TestFigure) GetAvailableMoves(
	desk *Field, from Coordinate,
) []Coordinate {
	return nil
}

func (c TestFigure) GetAvailableMovesToEat(
	desk *Field, from Coordinate,
) []Coordinate {
	return nil
}