package core

type GameCore struct {
	field       Field
	turnGamerID int
	checkersFeature
}

func (c GameCore) GetField() Field {
	return c.field
}

func (c GameCore) IsTurn(gamerID int) bool {
	return gamerID == c.turnGamerID
}

func (c *GameCore) Move(
	from Coordinate, way []Coordinate, gamerID int,
) bool {
	if gamerID != c.turnGamerID {
		return false
	}
	figure := c.field.At(from)
	if figure == nil {
		return false
	}
	if figure.GetOwnerID() != c.turnGamerID {
		return false
	}
	if !c.checkersFeature.CheckMove(from, way[0], gamerID) {
		return false
	}

	success, _ := figure.Move(&c.field, from, way)
	if success {
		c.turnGamerID ^= 1
	}
	return success
}

func (c *GameCore) InitField(field Field) {
	c.field = field
	c.checkersFeature.desk = &field
}

func (c *GameCore) InitTurnGamerID(turnGamerID int) {
	c.turnGamerID = turnGamerID
}
