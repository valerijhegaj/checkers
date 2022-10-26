package ai

import (
	"reflect"

	core2 "checkers/logic/core"
)

type Heuristics interface {
	CalculateScore(gamerID int, field *core2.Field) float64
}

func NewSimpleAmount() Heuristics {
	return &AmountWithCosts{1, 1}
}

func NewAmountWithCosts(kingCost, checkerCost float64) Heuristics {
	return &AmountWithCosts{kingCost, checkerCost}
}

type AmountWithCosts struct {
	KingCost    float64
	CheckerCost float64
}

func (c *AmountWithCosts) CalculateScore(
	gamerID int, field *core2.Field,
) float64 {
	_, goodFigures := (*field).GetFigures(gamerID)
	_, badFigures := (*field).GetFigures(gamerID ^ 1)

	return c.finalCalculate(&goodFigures, &badFigures)
}

func (c *AmountWithCosts) finalCalculate(
	goodFigures *[]core2.Figure, badFigures *[]core2.Figure,
) float64 {
	ans := float64(0)
	for _, figure := range *goodFigures {
		ans += c.valueOf(figure)
	}
	for _, figure := range *badFigures {
		ans -= c.valueOf(figure)
	}
	return ans
}

func (c *AmountWithCosts) valueOf(figure core2.Figure) float64 {
	var value float64
	if reflect.TypeOf(figure) == reflect.TypeOf(core2.Checker{}) {
		value = c.CheckerCost
	} else {
		value = c.KingCost
	}
	return value
}
