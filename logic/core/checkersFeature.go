package core

type checkersFeature struct {
	desk *Field
}

func (c checkersFeature) CheckMove(
	from, to Coordinate, gamerID int,
) bool {
	if c.isMoveToEat(from, to) {
		return true
	}
	if c.isGamerHasEater(gamerID) {
		return false
	}
	return true
}

func (c checkersFeature) isGamerHasEater(gamerID int) bool {
	coordinates, figures := c.desk.GetFigures(gamerID)
	for i, figure := range figures {
		from := coordinates[i]
		if figure.GetOwnerID() == gamerID {
			if figure.GetAvailableMovesToEat(c.desk, from) != nil {
				return true
			}
		}
	}
	return false
}

func (c checkersFeature) isMoveToEat(from, to Coordinate) bool {
	_, foodPosition := c.desk.At(from).IsMoveOne(c.desk, from, to)
	if foodPosition == c.desk.BordersLeft {
		return false
	}
	return true
}
