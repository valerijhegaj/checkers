package main

//import (
//	"checkers/core"
//	"checkers/grafInterface"
//	"fyne.io/fyne/v2"
//	"fyne.io/fyne/v2/app"
//)
//
////consol
////func main() {
////	var Interface _interface.Interface
////	var Core core.GameCore
////	exiter := make(chan int)
////	Interface.Init(exiter, Core)
////	<-exiter
////}
//
//func main() {
//	a := app.New()
//	w := a.NewWindow("Checkers")
//	var c core.GameCore
//	w.Resize(fyne.NewSize(500, 500))
//	var interactor grafInterface.Interface
//	interactor.Init(&a, &w, &c)
//	interactor.Begin(&interactor.MainMenu)
//	w.ShowAndRun()
//}
