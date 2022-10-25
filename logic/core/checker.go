package core

type Checker struct {
	OwnerID int
}

func (c Checker) ToString() string {
	return "Checker"
}

func (c Checker) GetOwnerID() int {
	return c.OwnerID
}

func (c Checker) toTransKing(desk *Field, position Coordinate) King {
	desk.RemoveWithOutBin(position)
	king := King{c.OwnerID}
	desk.Put(position, king)
	return king
}

func (c Checker) getFinishLine(desk *Field) int {
	vertical := desk.BordersRight.X
	if c.OwnerID == 1 {
		vertical = desk.BordersLeft.X
	}
	return vertical
}

func (c Checker) Move(
	desk *Field, from Coordinate, way []Coordinate,
) (bool, Coordinate) {
	vertical := c.getFinishLine(desk)

	isMoveWithoutEat, _ := c.isMoveWithoutEat(desk, from, way[0])
	if isMoveWithoutEat {
		desk.Move(from, way[0])

		if way[0].X == vertical {
			c.toTransKing(desk, way[0])
		}
		return true, way[0]
	}

	isCanBeMoved, foodPosition := c.isMoveToEat(desk, from, way[0])
	if !isCanBeMoved {
		return false, from
	}
	desk.Remove(foodPosition)
	desk.Move(from, way[0])

	if way[0].X == vertical {
		king := c.toTransKing(desk, way[0])
		return king.moveToEat(desk, way[0], way[1:])
	}
	return c.moveToEat(desk, way[0], way[1:])
}

func (c Checker) moveToEat(
	desk *Field, from Coordinate, way []Coordinate,
) (bool, Coordinate) {
	vertical := c.getFinishLine(desk)
	for i, to := range way {
		isCanBeMoved, foodPosition := c.isMoveToEat(desk, from, to)

		if !isCanBeMoved {
			return true, from
		}

		desk.Remove(foodPosition)
		desk.Move(from, to)

		from = to
		if to.X == vertical {
			king := c.toTransKing(desk, from)
			return king.moveToEat(desk, from, way[i+1:])
		}
	}
	return true, from
}

func (c Checker) IsMoveOne(desk *Field, from, to Coordinate) (
	bool, Coordinate,
) {
	isMoveWithFood, foodPosition := c.isMoveToEat(desk, from, to)
	if isMoveWithFood {
		return isMoveWithFood, foodPosition
	}
	isMoveWithOutFood, _ := c.isMoveWithoutEat(desk, from, to)
	return isMoveWithOutFood, foodPosition
}

func (c Checker) isMoveWithoutEat(
	desk *Field, from, to Coordinate,
) (bool, Coordinate) {
	verticalDirection := 1
	if c.GetOwnerID() == 1 {
		verticalDirection = -1
	}
	if to.X-from.X == verticalDirection &&
		(to.Y-from.Y == 1 || to.Y-from.Y == -1) {
		if desk.IsAvailable(to) {
			return true, desk.BordersLeft
		}
	}
	return false, desk.BordersLeft
}

func (c Checker) isMoveToEat(desk *Field, from, to Coordinate) (
	bool, Coordinate,
) {
	foodPosition := Coordinate{(to.X + from.X) / 2, (to.Y + from.Y) / 2}

	if (to.X-from.X == 2 || to.X-from.X == -2) &&
		(to.Y-from.Y == 2 || to.Y-from.Y == -2) {
		if desk.IsAvailable(to) && !desk.IsAvailable(foodPosition) {
			food := desk.At(foodPosition)
			if food.GetOwnerID() != c.GetOwnerID() {
				return true, foodPosition
			}
		}
	}

	return false, desk.BordersLeft
}

func (c Checker) addMove(
	moves []Coordinate, desk *Field, d, from Coordinate,
	IsCanMove func(field *Field, from, to Coordinate) (
		bool, Coordinate,
	),
) []Coordinate {
	move := Coordinate{from.X + d.X, from.Y + d.Y}
	isMove, _ := IsCanMove(desk, from, move)
	if isMove {
		return append(moves, move)
	}
	return moves
}

func (c Checker) GetAvailableMoves(
	desk *Field, from Coordinate,
) []Coordinate {
	var moves []Coordinate
	verticalDirection := 1
	if c.GetOwnerID() == 1 {
		verticalDirection = -1
	}
	moves = c.addMove(
		moves, desk, Coordinate{verticalDirection, 1}, from,
		c.isMoveWithoutEat,
	)
	moves = c.addMove(
		moves, desk, Coordinate{verticalDirection, -1}, from,
		c.isMoveWithoutEat,
	)
	moves = c.addMove(
		moves, desk, Coordinate{2, 2}, from,
		c.isMoveToEat,
	)
	moves = c.addMove(
		moves, desk, Coordinate{2, -2}, from,
		c.isMoveToEat,
	)
	moves = c.addMove(
		moves, desk, Coordinate{-2, 2}, from,
		c.isMoveToEat,
	)
	moves = c.addMove(
		moves, desk, Coordinate{-2, -2}, from,
		c.isMoveToEat,
	)
	return moves
}

func (c Checker) GetAvailableMovesToEat(
	desk *Field,
	from Coordinate,
) []Coordinate {
	var moves []Coordinate
	moves = c.addMove(
		moves, desk, Coordinate{2, 2}, from, c.isMoveToEat,
	)
	moves = c.addMove(
		moves, desk, Coordinate{-2, 2}, from, c.isMoveToEat,
	)
	moves = c.addMove(
		moves, desk, Coordinate{2, -2}, from, c.isMoveToEat,
	)
	moves = c.addMove(
		moves, desk, Coordinate{-2, -2}, from, c.isMoveToEat,
	)
	return moves
}
