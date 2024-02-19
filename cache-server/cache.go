package main

import (
	"fmt"
	"sync"
)

type cache struct {
	keyMap map[string][]byte
	lock   sync.RWMutex
}

func NewCache() cache {
	return cache{
		keyMap: make(map[string][]byte),
	}
}

func (c *cache) Get(key string) ([]byte, error) {
	c.lock.RLock()
	defer c.lock.RUnlock()

	if value, ok := c.keyMap[key]; ok {
		return value, nil
	}

	return nil, fmt.Errorf("key %s not found", key)
}

func (c *cache) Set(key string, value []byte) {
	c.lock.Lock()
	defer c.lock.Unlock()

	c.keyMap[key] = value
}

func (c *cache) Remove(key string) {
	delete(c.keyMap, key)
}
