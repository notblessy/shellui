package main

import (
	"fmt"

	"github.com/notblessy/shellui/core/app"
	"github.com/notblessy/shellui/core/scene"
	"github.com/notblessy/shellui/core/state"
	"github.com/notblessy/shellui/core/view"
	"github.com/notblessy/shellui/widget/button"
	"github.com/notblessy/shellui/widget/text"
)

// CounterApp demonstrates state management in shellui.
type CounterApp struct{}

func (a *CounterApp) Body() scene.Scene {
	return scene.WindowGroup(
		NewCounterView(),
	).Title("Counter Example").Size(400, 300)
}

// CounterView demonstrates reactive state.
type CounterView struct {
	view.ViewBaseType
	Count *state.StateType[int]
}

func NewCounterView() *CounterView {
	return &CounterView{
		Count: state.New(0),
	}
}

func (cv *CounterView) Body() view.View {
	return view.VStack(
		text.Text(fmt.Sprintf("Count: %d", cv.Count.Value())),
		view.HStack(
			button.Button("-", func() {
				cv.Count.Set(cv.Count.Value() - 1)
			}),
			view.Spacer(),
			button.Button("+", func() {
				cv.Count.Set(cv.Count.Value() + 1)
			}),
		),
	)
}

func main() {
	app.Run(&CounterApp{})
}
