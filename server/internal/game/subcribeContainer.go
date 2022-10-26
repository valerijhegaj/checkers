package game

import "sync"

func NewSubscribeSystem() SubscriberSystem {
	return newMapSubscribersGuarded()
}

type SubscriberSystem interface {
	Subscribe(observer func()) func()
	NotifyAll()
}

//--------------------------------------------------------------------

func newMapSubscribers() *mapSubscribers {
	return &mapSubscribers{observers: make(map[int]func())}
}

type mapSubscribers struct {
	mu             sync.RWMutex
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

//--------------------------------------------------------------------

func newMapSubscribersGuarded() *mapSubscribersGuarded {
	return &mapSubscribersGuarded{mapSubscribers: *newMapSubscribers()}
}

type mapSubscribersGuarded struct {
	mapSubscribers
	mu sync.RWMutex
}

func (c *mapSubscribersGuarded) Subscribe(observer func()) func() {
	c.mu.Lock()
	unsubscribe := c.mapSubscribers.Subscribe(observer)
	c.mu.Unlock()

	return func() {
		c.mu.Lock()
		unsubscribe()
		c.mu.Unlock()
	}
}

func (c *mapSubscribersGuarded) NotifyAll() {
	c.mu.RLock()
	c.mapSubscribers.NotifyAll()
	c.mu.RUnlock()
}
