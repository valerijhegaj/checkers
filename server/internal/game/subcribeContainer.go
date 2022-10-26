package game

func NewSubscribeSystem() SubscriberSystem {
	return newMapSubscribers()
}

type SubscriberSystem interface {
	Subscribe(observer func()) func()
	NotifyAll()
}

func newMapSubscribers() *mapSubscribers {
	return &mapSubscribers{observers: make(map[int]func())}
}

type mapSubscribers struct {
	observers      map[int]func()
	lastObserverID int
}

func (c *mapSubscribers) Subscribe(observer func()) func() {
	thisObserverID := c.lastObserverID
	c.lastObserverID++
	c.observers[thisObserverID] = observer

	return func() {
		delete(c.observers, thisObserverID)
	}
}

func (c *mapSubscribers) NotifyAll() {
	for _, observer := range c.observers {
		go observer()
	}
}
