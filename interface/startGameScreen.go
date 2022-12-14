package _interface

import (
	"fmt"

	"checkers/core"
	"checkers/saveLoad"
)

type startGameScreen struct {
	interactor *Interface
}

func (c startGameScreen) Display() {
	fmt.Println("write 3 commands sequentially:")
	fmt.Println("start")
	fmt.Println("gamer or bot00 (01, 08 level) (who will be for white)")
	fmt.Println("gamer or bot00 (who will be for red)")
	go c.Resume()
}

func (c startGameScreen) DisplayHelp() {
	displayHelpBasic()
	go c.Display()
}

func (c startGameScreen) parse(command string) int {
	if command == "start" || command == "Start" {
		var save saveLoad.Save
		save.Field = core.NewStandard8x8Field()

		save.Master = c.getMaster()
		save.TurnGamerId = 0
		c.interactor.initSave(save)

		c.interactor.status = menu

		return game
	}
	return parseBasic(command)
}

func (c startGameScreen) getMaster() saveLoad.Participants {
	var master saveLoad.Participants
	var name string
	c.interactor.mutex.Lock()
	fmt.Scan(&name)
	c.interactor.mutex.Unlock()
	c.parseMasterOne(&master.Gamer0, &master.Level0, name)
	c.interactor.mutex.Lock()
	fmt.Scan(&name)
	c.interactor.mutex.Unlock()
	c.parseMasterOne(&master.Gamer1, &master.Level1, name)
	return master
}

func (c startGameScreen) parseMasterOne(
	gamer, level *int,
	name string,
) {
	if len(name) != 5 {
		*gamer = saveLoad.Man
		return
	}
	if name[:3] == "bot" || name[:3] == "Bot" {
		*gamer = saveLoad.Bot
		*level = int(name[4]-'0') + int(name[3]-'0')*10
	} else {
		*gamer = saveLoad.Man
	}
}

func (c startGameScreen) Resume() {
	command := c.interactor.GetCommand(c.parse)
	c.interactor.switchCommander(command, c)
}
