package main

import (
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"testing"

	"checkers/logic/core"
	"checkers/logic/saveLoad"
	"checkers/server/pkg/defines"
	"checkers/server/test/api"
	"checkers/server/test/format"
)

func Test_server(t *testing.T) {
	os.Chdir("..")
	defer os.Chdir("cmd")
	log.SetOutput(ioutil.Discard)
	go main()

	valerijhegaj := &apiParser.User{
		Username: "valerijhegaj", Password: "123", PORT: 4444,
	}

	//----------------------test1---------------------------------------
	// create user, log in
	{
		code, err := valerijhegaj.Register()
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, code))
		}

		code, err = valerijhegaj.LogIn(60)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, code))
		}

		if valerijhegaj.IsEmptyCookies() {
			t.Error(format.ErrorString("cookies", "no cookies"))
		}
	}

	//----------------------test2---------------------------------------
	// try to create user with same nick
	{
		hacker := &apiParser.User{
			Username: valerijhegaj.Username, Password: "wrong", PORT: 4444,
		}
		code, err := hacker.Register()
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusForbidden {
			t.Error(format.ErrorInt(http.StatusForbidden, code))
		}

		code, err = hacker.LogIn(60)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusForbidden {
			t.Error(format.ErrorInt(http.StatusForbidden, code))
		}
	}

	gameName1, password1 := "fitstField", "1"
	firstField := core.NewStandard8x8Field()

	//----------------------test3---------------------------------------
	// create game, log in, get, moveFirstField and get
	{
		code, err := valerijhegaj.CreateGame(
			gameName1, password1, defines.Settings{},
		)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, code))
		}

		code, err = valerijhegaj.LogInGame(
			gameName1, password1,
		)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, code))
		}

		code, rawSave, err := valerijhegaj.GetGame(gameName1)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusOK {
			t.Error(format.ErrorInt(http.StatusOK, code))
		}

		save, err := saveLoad.NewSaveFromRawSave(rawSave)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}

		if !core.IsEqual(&save.Field, &firstField) {
			t.Error(format.ErrorField(&firstField, &save.Field))
		}
		if save.TurnGamerID != 0 {
			t.Error(format.ErrorInt(0, save.TurnGamerID))
		}
		if save.Winner != -1 {
			t.Error(format.ErrorInt(-1, save.Winner))
		}

		from := core.Coordinate{2, 0}
		to := []core.Coordinate{{3, 1}}

		code, err = valerijhegaj.Move(gameName1, from, to)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, code))
		}

		code, rawSave, err = valerijhegaj.GetGame(gameName1)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusOK {
			t.Error(format.ErrorInt(http.StatusOK, code))
		}

		save, err = saveLoad.NewSaveFromRawSave(rawSave)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}

		figure := firstField.At(from)
		figure.Move(&firstField, from, to)

		if !core.IsEqual(&save.Field, &firstField) {
			t.Error(format.ErrorField(&firstField, &save.Field))
		}
		if save.TurnGamerID != 1 {
			t.Error(format.ErrorInt(0, save.TurnGamerID))
		}
		if save.Winner != -1 {
			t.Error(format.ErrorInt(-1, save.Winner))
		}
	}

	aboba := &apiParser.User{
		Username: "aboba", Password: "abob", PORT: 4444,
	}

	{
		code, err := aboba.Register()
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, code))
		}

		code, err = aboba.LogIn(60)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, code))
		}

		if aboba.IsEmptyCookies() {
			t.Error(format.ErrorString("cookies", "no cookies"))
		}
	}

	//----------------------test4---------------------------------------
	// try to log in game with wrong password
	// try without log in to get field, moveFirstField
	// try to create game with such name
	{
		code, err := aboba.LogInGame(gameName1, password1+"evil")
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusForbidden {
			t.Error(format.ErrorInt(http.StatusForbidden, code))
		}

		code, err = aboba.LogInGame(gameName1+"evil", password1)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusNotFound {
			t.Error(format.ErrorInt(http.StatusNotFound, code))
		}

		code, _, err = aboba.GetGame(gameName1)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusForbidden {
			t.Error(format.ErrorInt(http.StatusForbidden, code))
		}

		code, err = aboba.Move(
			gameName1, core.Coordinate{5, 1}, []core.Coordinate{{4, 0}},
		)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusForbidden {
			t.Error(format.ErrorInt(http.StatusForbidden, code))
		}

		code, err = aboba.CreateGame(
			gameName1, password1, defines.Settings{},
		)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusForbidden {
			t.Error(format.ErrorInt(http.StatusForbidden, code))
		}

	}

	//----------------------test5---------------------------------------
	// log in game, get, moveFirstField and get
	generateFromTo := func(data []int) (
		core.Coordinate, []core.Coordinate,
	) {
		from := core.Coordinate{data[0], data[1]}
		var to []core.Coordinate
		for i := 2; i < len(data); i += 2 {
			to = append(to, core.Coordinate{data[i], data[i+1]})
		}
		return from, to
	}
	moveCreator := func(field *core.Field, gameName string) func(
		isCorrect bool, from core.Coordinate, to []core.Coordinate,
		user *apiParser.User,
	) {
		return func(
			isCorrect bool, from core.Coordinate, to []core.Coordinate,
			user *apiParser.User,
		) {
			code, err := user.Move(gameName, from, to)
			if err != nil {
				t.Error(format.ErrorString("without errors", err.Error()))
			}
			if isCorrect {
				if code != http.StatusCreated {
					t.Error(format.ErrorInt(http.StatusCreated, code))
				}
				figure := field.At(from)
				figure.Move(field, from, to)
			} else {
				if code != http.StatusMethodNotAllowed {
					t.Error(format.ErrorInt(http.StatusMethodNotAllowed, code))
				}
			}

			code, rawSave, err := user.GetGame(gameName)
			if err != nil {
				t.Error(format.ErrorString("without errors", err.Error()))
			}
			if code != http.StatusOK {
				t.Error(format.ErrorInt(http.StatusOK, code))
			}

			save, err := saveLoad.NewSaveFromRawSave(rawSave)
			if err != nil {
				t.Error(format.ErrorString("without errors", err.Error()))
			}
			if !core.IsEqual(field, &save.Field) {
				t.Error(format.ErrorField(field, &save.Field))
			}
		}
	}
	moveFirstField := moveCreator(&firstField, gameName1)

	{
		code, err := aboba.LogInGame(gameName1, password1)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, code))
		}

		code, rawSave, err := aboba.GetGame(gameName1)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if code != http.StatusOK {
			t.Error(format.ErrorInt(http.StatusOK, code))
		}

		save, err := saveLoad.NewSaveFromRawSave(rawSave)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if !core.IsEqual(&firstField, &save.Field) {
			t.Error(format.ErrorField(&firstField, &save.Field))
		}

		from, to := generateFromTo([]int{5, 1, 4, 0})
		moveFirstField(true, from, to, aboba)
	}

	//----------------------test6---------------------------------------
	// 0 try to make wrong moves
	// 0 moveFirstField to make 1 moveFirstField to eat
	// 1 try moveFirstField wrong
	// 1 eat two
	// 0 moveFirstField to make 1 moveFirstField to eat
	// 1 eat
	{
		from, to := generateFromTo([]int{3, 1, 2, 0})
		moveFirstField(false, from, to, valerijhegaj)

		from, to = generateFromTo([]int{5, 3, 4, 2})
		moveFirstField(false, from, to, valerijhegaj)
		moveFirstField(false, from, to, aboba)

		from, to = generateFromTo([]int{2, 2, 3, 3})
		moveFirstField(false, from, to, aboba)
		moveFirstField(true, from, to, valerijhegaj)

		from, to = generateFromTo([]int{5, 3, 4, 2})
		moveFirstField(false, from, to, valerijhegaj)

		from, to = generateFromTo([]int{4, 0, 2, 2, 4, 4})
		moveFirstField(true, from, to, aboba)

		from, to = generateFromTo([]int{2, 4, 3, 3})
		moveFirstField(true, from, to, valerijhegaj)

		from, to = generateFromTo([]int{4, 4, 2, 2})
		moveFirstField(true, from, to, aboba)
	}

	//----------------------test7---------------------------------------
	//is authorized
	{
		isAuth, err := valerijhegaj.IsAuthorized()
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if !isAuth {
			t.Error(format.ErrorInt(1, 0))
		}

		byba := apiParser.User{
			Username: "byba",
			PORT:     4444,
			Password: password1,
		}

		isAuth, err = byba.IsAuthorized()
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if isAuth {
			t.Error(format.ErrorInt(0, 1))
		}
	}

	gameName2, password2 := "test8", "1"
	field2 := core.NewStandard8x8Field()
	moveField2 := moveCreator(&field2, gameName2)
	//----------------------test8---------------------------------------
	// subscribe
	{
		statusCode, err := valerijhegaj.CreateGame(
			gameName2, password2, defines.Settings{},
		)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if statusCode != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, statusCode))
		}

		statusCode, _, err = aboba.SubscribeGame(gameName2)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if statusCode != http.StatusForbidden {
			t.Error(format.ErrorInt(http.StatusForbidden, statusCode))
		}

		aboba2 := apiParser.User{
			Username: "aboba2", Password: "123", PORT: 4444,
		}
		statusCode, _, err = aboba2.SubscribeGame(gameName2)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if statusCode != http.StatusUnauthorized {
			t.Error(format.ErrorInt(http.StatusUnauthorized, statusCode))
		}

		statusCode, _, err = aboba.SubscribeGame(gameName2 + "evil")
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if statusCode != http.StatusNotFound {
			t.Error(format.ErrorInt(http.StatusNotFound, statusCode))
		}

		statusCode, err = valerijhegaj.LogInGame(gameName2, password2)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if statusCode != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, statusCode))
		}
		statusCode, err = aboba.LogInGame(gameName2, password2)
		if err != nil {
			t.Error(format.ErrorString("without errors", err.Error()))
		}
		if statusCode != http.StatusCreated {
			t.Error(format.ErrorInt(http.StatusCreated, statusCode))
		}

		go func() {
			statusCode, rawSave, err := aboba.SubscribeGame(gameName2)
			if err != nil {
				t.Error(format.ErrorString("without errors", err.Error()))
			}
			if statusCode != http.StatusOK {
				t.Error(format.ErrorInt(http.StatusOK, statusCode))
			}

			save, err := saveLoad.NewSaveFromRawSave(rawSave)
			if err != nil {
				t.Error(format.ErrorString("without errors", err.Error()))
			}

			if !core.IsEqual(&save.Field, &field2) {
				t.Error(format.ErrorField(&field2, &save.Field))
			}
			if save.TurnGamerID != 1 {
				t.Error(format.ErrorInt(1, save.TurnGamerID))
			}
			if save.Winner != -1 {
				t.Error(format.ErrorInt(-1, save.Winner))
			}
		}()
		from, to := generateFromTo([]int{2, 0, 3, 1})
		moveField2(true, from, to, valerijhegaj)
	}
}
