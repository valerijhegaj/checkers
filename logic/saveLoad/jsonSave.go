package saveLoad

import (
	"encoding/json"

	"checkers/logic/core"
)

func newJsonSaveFromRawSave(rawSave []byte) (jsonSave, error) {
	var obj jsonSave
	err := obj.initFromRawSave(rawSave)
	return obj, err
}

func newJsonSaveFromSave(save *Save) jsonSave {
	var obj jsonSave
	obj.initFromSave(save)
	return obj
}

type figureInfo struct {
	X       int    `json:"x"`
	Y       int    `json:"y"`
	Figure  string `json:"figure"`
	GamerID int    `json:"gamerId"`
}

type jsonSave struct {
	Figures      []figureInfo    `json:"figures"`
	BordersRight core.Coordinate `json:"bordersRight"`
	BordersLeft  core.Coordinate `json:"bordersLeft"`
	Position     Participants    `json:"position"`
	TurnGamerId  int             `json:"turnGamerId"`
	Winner       int             `json:"winner"`
}

func (c *jsonSave) takeFiguresFromField(field core.Field) {
	for gamerID := 0; gamerID < 2; gamerID++ {
		coordinates, figures := field.GetFigures(gamerID)
		for i, figure := range figures {
			elem := figureInfo{
				X: coordinates[i].X, Y: coordinates[i].Y,
				Figure:  figure.ToString(),
				GamerID: gamerID,
			}
			c.Figures = append(c.Figures, elem)
		}
	}
}

func (c *jsonSave) initFromSave(save *Save) {
	c.Position = save.Master
	c.TurnGamerId = save.TurnGamerID
	c.BordersRight = save.Field.BordersRight
	c.BordersLeft = save.Field.BordersLeft
	c.Winner = save.Winner
	c.takeFiguresFromField(save.Field)
}

func (c *jsonSave) getRawSave() ([]byte, error) {
	return json.Marshal(c)
}

func (c *jsonSave) initFromRawSave(rawSave []byte) error {
	return json.Unmarshal(rawSave, c)
}
