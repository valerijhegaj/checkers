package game

import "checkers/logic/core"

//func NewGame(settings defines.Settings, password string) *Game {
//	var c core.GameCore
//	game := Game{
//		gamer: [2]gamer.Gamer{{0, &c}, {1, &c}},
//		bot: [2]ai.Ai{
//			ai.NewBot(settings.Level[0]), ai.NewBot(settings.Level[1]),
//		},
//
//		userID: [2]int{-1, -1},
//		Participants: Participants{
//			[2]int{settings.Gamer[0], settings.Gamer[1]},
//			[2]int{settings.Level[0], settings.Level[1]},
//		},
//
//		password:   password,
//		accessList: make(map[int]bool),
//
//		winner: -1,
//	}
//	save := saveLoad.Save{
//		Field:       core.NewStandard8x8Field(),
//		TurnGamerID: 0,
//	}
//	game.gamer[0].InitSave(save)
//	return &game
//}
//
//type Participants struct {
//	gamer [2]int
//	level [2]int
//}
//
//type Game struct {
//	gamer [2]gamer.Gamer
//	bot   [2]ai.Ai
//
//	userID [2]int
//	Participants
//
//	password   string
//	accessList map[int]bool
//
//	winner int
//}
//
//func (c *Game) Move(
//	userID int, from core.Coordinate, path []core.Coordinate,
//) error {
//	var i int
//	switch userID {
//	case c.userID[0]:
//		i = 0
//	case c.userID[1]:
//		i = 1
//	default:
//		return errors.New(errorsStrings.PermissionDenied)
//	}
//	if !c.gamer[i].Move(from, path) {
//		return errors.New(errorsStrings.IncorrectMove)
//	}
//
//	{
//		isEnd, winner := c.gamer[0].GetWinner()
//		if isEnd {
//			c.winner = winner.GamerID
//			return nil
//		}
//	}
//
//	i = i ^ 1
//	if c.Participants.gamer[i] == saveLoad.Bot {
//		go func() {
//			c.bot[i].Move(c.gamer[i])
//			isEnd, winner := c.gamer[0].GetWinner()
//			if isEnd {
//				c.winner = winner.GamerID
//			}
//		}()
//	}
//	return nil
//}
//
//func (c *Game) GetGame(userID int) ([]byte, error) {
//	var save saveLoad.Save
//	if !c.accessList[userID] {
//		return nil, errors.New(errorsStrings.PermissionDenied)
//	}
//	save.Field = c.gamer[0].GetField()
//	if c.gamer[0].IsTurn() {
//		save.TurnGamerID = 0
//	} else {
//		save.TurnGamerID = 1
//	}
//
//	for gamerID := 0; gamerID < 2; gamerID++ {
//		save.Master.Gamer[gamerID] = c.Participants.gamer[gamerID]
//		save.Master.Level[gamerID] = c.Participants.level[gamerID]
//	}
//
//	save.Winner = c.winner
//	return save.GetRawSave()
//}
//
//func (c *Game) AddUser(userID int, password string) error {
//	if c.password != password {
//		return errors.New(errorsStrings.PermissionDenied)
//	}
//	defer func() { c.accessList[userID] = true }()
//	if c.Participants.gamer[0] == saveLoad.Bot {
//		if c.userID[1] == -1 {
//			go c.bot[0].Move(c.gamer[0])
//			c.userID[1] = userID
//		}
//		return nil
//	}
//	if c.userID[0] == -1 {
//		c.userID[0] = userID
//		return nil
//	}
//	if c.Participants.gamer[1] == saveLoad.Man && c.userID[1] == -1 {
//		c.userID[1] = userID
//		return nil
//	}
//	return nil
//}

type Container interface {
	CheckAccess(userID int) bool
	AddUser(userID int)
	Move(userID int, from core.Coordinate, way []core.Coordinate) bool
	Game
}
