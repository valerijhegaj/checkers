package saveLoad

import (
	"checkers/logic/core"
)

const (
	Man = iota
	Bot
)

func NewSaveFromRawSave(rawSave []byte) (*Save, error) {
	var save Save
	err := save.InitFromRawSave(rawSave)
	return &save, err
}

type Participants struct {
	Gamer [2]int `json:"gamer"`
	Level [2]int `json:"level"`
}

type Save struct {
	Field       core.Field
	Master      Participants
	TurnGamerID int
	Winner      int
}

func (c *Save) putFiguresOnField(figures []figureInfo) {
	for _, i := range figures {
		place := core.Coordinate{X: i.X, Y: i.Y}
		if i.Figure == "Checker" {
			c.Field.Put(place, core.Checker{OwnerID: i.GamerID})
		} else if i.Figure == "King" {
			c.Field.Put(place, core.King{OwnerID: i.GamerID})
		}
	}
}

func (c *Save) initFromJsonSave(jsonSave *jsonSave) {
	c.Field = core.NewField()
	c.putFiguresOnField(jsonSave.Figures)
	c.Field.BordersRight = jsonSave.BordersRight
	c.Field.BordersLeft = jsonSave.BordersLeft

	c.Master = jsonSave.Position
	c.TurnGamerID = jsonSave.TurnGamerId
	c.Winner = jsonSave.Winner
}

func (c *Save) GetRawSave() ([]byte, error) {
	helper := newJsonSaveFromSave(c)
	return helper.getRawSave()
}

func (c *Save) InitFromRawSave(rawSave []byte) error {
	helper, err := newJsonSaveFromRawSave(rawSave)
	if err != nil {
		return err
	}
	c.initFromJsonSave(&helper)
	return nil
}
