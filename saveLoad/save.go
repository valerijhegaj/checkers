package saveLoad

import (
	"encoding/json"
	"io/ioutil"
	"reflect"

	"checkers/core"
)

const (
	Man = iota
	Bot
)

func NewSaveFromRawSave(rawSave []byte) *Save {
	var save Save
	save.InitFromRawSave(rawSave)
	return &save
}

type Participants struct {
	Gamer0 int `json:"gamer0"`
	Level0 int `json:"level0"`
	Gamer1 int `json:"gamer1"`
	Level1 int `json:"level1"`
}

func GetSaveList(path string) ([]string, error) {
	var saveList []string
	files, err := ioutil.ReadDir(path)
	if err != nil {
		return saveList, err
	}
	for _, file := range files {
		if !file.IsDir() && len(file.Name()) > 5 &&
			file.Name()[len(file.Name())-5:len(file.Name())] == ".json" {
			saveList = append(saveList, file.Name())
		}
	}

	return saveList, err
}

type Save struct {
	Field       core.Field
	Master      Participants
	TurnGamerId int
	Winner      int
}

func (c *Save) putFiguresOnField(figures []figureInfo) {
	for _, i := range figures {
		if i.Figure == "Checker" {
			c.Field.Figures[core.Coordinate{
				i.X, i.Y,
			}] = core.Checker{OwnerId: i.GamerId}
		} else if i.Figure == "King" {
			c.Field.Figures[core.Coordinate{
				i.X, i.Y,
			}] = core.King{OwnerId: i.GamerId}
		}
	}
}

func (c *Save) initFromJsonSave(jsonSave *jsonSave) {
	c.Field = core.NewField()
	c.putFiguresOnField(jsonSave.Figures)
	c.Field.BordersRight = jsonSave.BordersRight
	c.Field.BordersLeft = jsonSave.BordersLeft
	c.Master = jsonSave.Position
	c.TurnGamerId = jsonSave.TurnGamerId
	c.Winner = jsonSave.Winner
}

func (c *Save) GetRawSave() (
	[]byte,
	error,
) {
	var helper jsonSave
	helper.initFromSave(c)
	return helper.getRawSave()
}

func (c *Save) InitFromRawSave(rawSave []byte) error {
	var helper jsonSave
	err := helper.initFromRawSave(rawSave)
	if err != nil {
		return err
	}
	c.initFromJsonSave(&helper)
	return nil
}

func (c *Save) Read(path string) error {
	var helper jsonSave
	err := helper.read(path)
	c.initFromJsonSave(&helper)
	return err
}

func (c *Save) Write(path string) error {
	var helper jsonSave
	helper.initFromSave(c)
	return helper.write(path)
}

type figureInfo struct {
	X       int    `json:"x"`
	Y       int    `json:"y"`
	Figure  string `json:"figure"`
	GamerId int    `json:"gamerId"`
}

type jsonSave struct {
	Figures      []figureInfo    `json:"figures"`
	BordersRight core.Coordinate `json:"bordersRight"`
	BordersLeft  core.Coordinate `json:"bordersLeft"`
	Position     Participants    `json:"position"`
	TurnGamerId  int             `json:"turnGamerId"`
	Winner       int             `json:"winner"`
}

// warning reflect.TypeOf(figure).String()[5:]
// can don't work with other names and struct of project
func (c *jsonSave) takeFiguresFromField(field core.Field) {
	c.Figures = make([]figureInfo, len(field.Figures))
	i := 0
	for coordinate, figure := range field.Figures {
		c.Figures[i].X = coordinate.X
		c.Figures[i].Y = coordinate.Y
		c.Figures[i].Figure = reflect.TypeOf(figure).String()[5:]
		c.Figures[i].GamerId = figure.GetOwnerId()
		i++
	}
}

func (c *jsonSave) initFromSave(save *Save) {
	c.Position = save.Master
	c.TurnGamerId = save.TurnGamerId
	c.BordersRight = save.Field.BordersRight
	c.BordersLeft = save.Field.BordersLeft
	c.Winner = save.Winner
	c.takeFiguresFromField(save.Field)
}

func (c *jsonSave) getRawSave() (
	[]byte,
	error,
) {
	return json.Marshal(c)
}

func (c *jsonSave) initFromRawSave(rawSave []byte) error {
	return json.Unmarshal(rawSave, c)
}

func (c *jsonSave) write(path string) error {
	rawSave, err := c.getRawSave()
	if err != nil {
		return err
	}
	err = ioutil.WriteFile(path, rawSave, 0644)
	return err
}

func (c *jsonSave) read(path string) error {
	rawSave, err := ioutil.ReadFile(path)
	if err != nil {
		return err
	}
	err = json.Unmarshal(rawSave, c)
	return err
}
