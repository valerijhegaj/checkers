package main

import (
	"fmt"
	"math/rand"
	"sync"
	"sync/atomic"
	"time"

	"checkers/bot"
	"checkers/core"
	"checkers/gamer"
)

type mind struct {
	kingCost    float64
	checkerCost float64
}

func test(level int) (
	float64,
	float64,
) {
	fmt.Println("init population")
	var population [8]mind
	population[0] = mind{1, 1}
	for i := 1; i < 8; i++ {
		population[i] = mind{rand.Float64() * 10, 1}
	}

	for generation := 0; generation < 20; generation++ {
		fmt.Println("Step")
		winners := grandCompare(population, level)
		population = createChilds(winners)
	}
	winners := grandCompare(population, level)
	return winners[0].kingCost, winners[0].checkerCost
}

func createChilds(winners [3]mind) [8]mind {
	var population [8]mind
	for i := range winners {
		population[i] = winners[i]
	}
	population[3] = mind{1, 1}
	population[4] = mind{rand.Float64() * 10, 1}
	population[5] = cross(winners[0], winners[1])
	population[6] = cross(winners[1], winners[2])
	population[7] = cross(winners[0], winners[2])
	return population
}

func cross(l, r mind) mind {
	return mind{(l.kingCost+r.kingCost)/2 + rand.Float64()*4 - 2, 1}
}

func grandCompare(population [8]mind, level int) [3]mind {
	var wg sync.WaitGroup
	var score [8]atomic.Int32
	for i := 0; i < 8; i++ {
		for j := 0; j < 8; j++ {
			if i == j {
				continue
			}
			wg.Add(1)
			i := i
			j := j
			mindi := population[i]
			mindj := population[j]
			go func() {
				for c := 0; c < 3; c++ {
					cmp := compare(mindi, mindj, level, level)
					if cmp == 1 {
						score[i].Add(2)
					} else if cmp == -1 {
						score[j].Add(2)
					} else {
						score[i].Add(1)
						score[j].Add(1)
					}
					fmt.Println(i, j, cmp)
				}
				wg.Done()
			}()
		}
	}
	wg.Wait()
	var top1, top2, top3 int
	for i := 1; i < 8; i++ {
		if score[i].Load() > score[top1].Load() {
			top3 = top2
			top2 = top1
			top1 = i
		} else if score[i].Load() > score[top2].Load() {
			top3 = top2
			top2 = i
		} else if score[i].Load() > score[top3].Load() {
			top3 = i
		}
	}
	return [3]mind{
		population[top1],
		population[top2],
		population[top3],
	}
}

func compare(l, r mind, leftLevel, rightLevel int) int {
	var bots [2]bot.Bot
	bots[0] = bot.Bot{
		bot.NewMinMaxV2(
			leftLevel,
			l.kingCost,
			l.checkerCost,
		),
	}
	bots[1] = bot.Bot{
		bot.NewMinMaxV2(
			rightLevel,
			r.kingCost,
			r.checkerCost,
		),
	}

	var c core.GameCore
	c.InitField(core.NewStandard8x8Field())
	c.InitTurnGamerId(0)

	var gamers [2]gamer.Gamer
	gamers[0] = gamer.Gamer{0, &c}
	gamers[1] = gamer.Gamer{1, &c}
	var i int
	for i = 0; i < 300; i++ {
		isFinished, winner := gamers[0].GetWinner()
		if isFinished {
			nullGamer := gamer.Gamer{0, nil}
			if winner == gamers[0] {
				fmt.Println(i)
				return 1
			} else if winner == nullGamer {
				fmt.Println(i)
				return 0
			} else if winner == gamers[1] {
				fmt.Println(i)
				return -1
			}
		}
		if gamers[0].IsTurn() {
			bots[0].Move(gamers[0])
		} else {
			bots[1].Move(gamers[1])
		}
	}
	fmt.Println(i)
	return 0
}

func main() {
	var wg sync.WaitGroup
	c := 10
	wg.Add(2 * c)
	start := time.Now()
	t := func() {
		x := compare(mind{2, 1}, mind{1, 1}, 3, 4)
		if x == 1 {
			fmt.Println("Yes")
		} else if x == 0 {
			fmt.Println("~~~")
		} else {
			fmt.Println("No")
		}
		wg.Done()
	}
	tr := func() {
		x := compare(mind{1, 1}, mind{2, 1}, 4, 3)
		if x == 1 {
			fmt.Println("NO")
		} else if x == 0 {
			fmt.Println("~~~")
		} else {
			fmt.Println("Yes")
		}
		wg.Done()
	}
	for i := 0; i < c; i++ {
		go t()
		go tr()
	}
	wg.Wait()
	fmt.Println(time.Now().Sub(start))

}

// for level 3 1 cmp ij ji - 6.3461017059905505 1
//1 1 2.535405005150605 1 5.318121490659735 1 4.5798202260313765 1

//10 cmp 4 lvl 3 1 vs 1 1 = 8 wins 2 draws
