package game

import (
	"checkers/logic/core"
	"checkers/logic/saveLoad"
	"checkers/server/pkg/defines"
)

func NewContainer(
	settings defines.Settings, password string,
) Container {
	return NewContainerClass(settings, password)
}

type Container interface {
	CheckAccess(userID int) bool
	AddUser(userID int, password string) bool
	Move(userID int, from core.Coordinate, way []core.Coordinate) bool
	Game
}

func NewContainerClass(
	settings defines.Settings, password string,
) *ContainerClass {
	return &ContainerClass{
		password: password, accessList: make(map[int]bool),
		Game:   NewGame(settings),
		userID: [2]int{-1, -1},
	}
}

type ContainerClass struct {
	password   string
	accessList map[int]bool
	userID     [2]int
	Game
}

func (c *ContainerClass) CheckAccess(userID int) bool {
	return c.accessList[userID]
}

func (c *ContainerClass) AddUser(userID int, password string) bool {
	if c.password != password {
		return false
	}
	c.accessList[userID] = true
	c.addGamer(userID)
	return true
}

func (c *ContainerClass) Move(
	userID int, from core.Coordinate, way []core.Coordinate,
) bool {
	isMoved := false
	for i := 0; i < 2; i++ {
		if c.userID[i] == userID {
			isMoved = isMoved || c.Game.Move(i, from, way)
		}
	}
	return isMoved
}

func (c *ContainerClass) addGamer(userID int) {
	for i := 0; i < 2; i++ {
		if c.userID[i] == -1 && c.Game.getSettingsGamer(i) == saveLoad.Man {
			c.userID[i] = userID
			return
		}
	}
}
