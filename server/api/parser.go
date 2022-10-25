package api

import (
	"encoding/json"
	"log"

	"checkers/logic/core"
	"checkers/server/pkg/defines"
)

type Helper struct {
	Password string            `json:"password,omitempty"`
	UserName string            `json:"username,omitempty"`
	MaxAge   int               `json:"max_age,omitempty"`
	GameName string            `json:"gamename,omitempty"`
	Settings defines.Settings  `json:"settings,omitempty"`
	From     core.Coordinate   `json:"from"`
	Way      []core.Coordinate `json:"to"`
}

func Parse(data []byte) (
	Helper,
	error,
) {

	r := string(data)
	log.Println(r)
	var h Helper
	err := json.Unmarshal(data, &h)
	return h, err
}

func UnParse(data Helper) []byte {
	rawData, _ := json.Marshal(data)
	return rawData
}
