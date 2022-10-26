package data

import (
	"errors"

	"checkers/logic/core"
	"checkers/logic/saveLoad"
	"checkers/server/internal/errorsStrings"
	"checkers/server/internal/game"
	"checkers/server/pkg/defines"
)

func NewCuratorRAMG() GameCurator {
	return &CuratorRAMG{
		game:      make(map[int]*game.Game),
		gameID:    make(map[string]int),
		maxGameID: 1,
	}
}

type CuratorRAMG struct {
	game   map[int]*game.Game
	gameID map[string]int

	maxGameID int
}

func (c *CuratorRAMG) NewGame(
	gameName, password string, settings defines.Settings,
) error {
	if settings.Gamer[0] == settings.Gamer[1] &&
		settings.Gamer[0] == saveLoad.Bot {
		return errors.New(errorsStrings.PermissionDenied)
	}
	_, ok := c.gameID[gameName]
	if ok {
		return errors.New(errorsStrings.GameAlreadyExist)
	}

	c.game[c.maxGameID] = game.NewGame(settings, password)
	c.gameID[gameName] = c.maxGameID

	c.maxGameID++
	return nil
}

func (c *CuratorRAMG) OnChangeGame(
	token string, gameName string, handler func([]byte),
) error {
	//not implemented
	return nil
}

func (c *CuratorRAMG) GetGame(
	token string, gameName string,
) ([]byte, error) {
	userID, err := GetGlobalStorage().GetUserID(token)
	if err != nil {
		return nil, errors.New(errorsStrings.NotAuthorized)
	}
	gameID, ok := c.gameID[gameName]
	if !ok {
		return nil, errors.New(errorsStrings.NotFound)
	}
	Game := c.game[gameID]
	return Game.GetGame(userID)
}

func (c *CuratorRAMG) LoginGame(
	token, gameName, password string,
) error {
	userID, err := GetGlobalStorage().GetUserID(token)
	if err != nil {
		return errors.New(errorsStrings.NotAuthorized)
	}
	gameID, ok := c.gameID[gameName]
	if !ok {
		return errors.New(errorsStrings.NotFound)
	}
	Game := c.game[gameID]
	return Game.AddUser(userID, password)
}
func (c *CuratorRAMG) ChangeGame(
	token, gameName string, settings defines.Settings,
) error {
	return nil
}
func (c *CuratorRAMG) DeleteGame(
	token string, gameName string,
) error {
	return nil
}
func (c *CuratorRAMG) MakeMove(
	token, gameName string, from core.Coordinate,
	path []core.Coordinate,
) error {
	userID, err := GetGlobalStorage().GetUserID(token)
	if err != nil {
		return errors.New(errorsStrings.NotAuthorized)
	}
	gameID, ok := c.gameID[gameName]
	if !ok {
		return errors.New(errorsStrings.NotFound)
	}
	Game := c.game[gameID]
	return Game.Move(userID, from, path)
}
