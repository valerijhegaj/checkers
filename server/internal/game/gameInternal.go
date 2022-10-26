package game

import (
	"checkers/logic/ai"
	"checkers/logic/core"
	"checkers/logic/gamer"
	"checkers/logic/saveLoad"
	"checkers/server/pkg/defines"
)

func NewGame(settings defines.Settings) Game {
	return newGameClass(settings)
}

type Game interface {
	Move(gamerID int, from core.Coordinate, way []core.Coordinate) bool
	GetGame() ([]byte, error)
	OnChangeGame(handler func([]byte))
	getSettingsGamer(i int) int
	SubscriberSystem
}

func newGameClass(settings defines.Settings) *GameClass {
	var c core.GameCore
	game := GameClass{
		gamer: [2]gamer.Gamer{{0, &c}, {1, &c}},
		bot: [2]ai.Ai{
			ai.NewBot(settings.Level[0]), ai.NewBot(settings.Level[1]),
		},

		Settings: settings,

		winner: -1,

		SubscriberSystem: NewSubscribeSystem(),
	}
	save := saveLoad.Save{
		Field:       core.NewStandard8x8Field(),
		TurnGamerID: 0,
	}
	game.gamer[0].InitSave(save)

	game.subscribeBot()

	return &game
}

type GameClass struct {
	gamer [2]gamer.Gamer
	bot   [2]ai.Ai

	defines.Settings

	winner int
	SubscriberSystem
}

func (c *GameClass) Move(
	gamerID int, from core.Coordinate, way []core.Coordinate,
) bool {
	if !c.gamer[gamerID].IsTurn() {
		return false
	}
	isMoved := c.gamer[gamerID].Move(from, way)
	if isMoved {
		c.gamer[gamerID].GetWinner()
		isEnd, winner := c.gamer[0].GetWinner()
		if isEnd {
			c.winner = winner.GamerID
		}
		c.NotifyAll()
	}
	return isMoved
}

func (c *GameClass) GetGame() ([]byte, error) {
	var save saveLoad.Save
	save.Field = c.gamer[0].GetField()

	save.TurnGamerID = 1
	if c.gamer[0].IsTurn() {
		save.TurnGamerID = 0
	}

	for gamerID := 0; gamerID < 2; gamerID++ {
		save.Master.Gamer[gamerID] = c.Settings.Gamer[gamerID]
		save.Master.Level[gamerID] = c.Settings.Level[gamerID]
	}

	save.Winner = c.winner
	return save.GetRawSave()
}

func (c *GameClass) OnChangeGame(handler func([]byte)) {
	unsubscribe := func() {}
	observer := func() {
		data, err := c.GetGame()
		if err != nil {
			handler(nil)
		}
		handler(data)
		unsubscribe()
	}
	unsubscribe = c.Subscribe(observer)
}

func (c *GameClass) subscribeBot() {
	for i := 0; i < 2; i++ {
		if c.Settings.Gamer[i] == saveLoad.Bot {
			ptr := i
			observer := func() {
				field := c.gamer[ptr].GetField()
				from, way := c.bot[ptr].GetMove(&field, ptr)
				c.gamer[ptr].Move(from, way)
			}
			c.Subscribe(observer)
		}
	}
}

func (c *GameClass) getSettingsGamer(i int) int {
	return c.Settings.Gamer[i]
}
