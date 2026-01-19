package state

import (
	"sync"
)

// StateType represents a reactive state value that can be observed.
// Similar to @State in SwiftUI.
type StateType[T any] struct {
	mu        sync.RWMutex
	value     T
	observers []func(T)
}

// New creates a new StateType with the given initial value.
func New[T any](value T) *StateType[T] {
	return &StateType[T]{
		value:     value,
		observers: make([]func(T), 0),
	}
}

// Value returns the current value of the state.
func (s *StateType[T]) Value() T {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.value
}

// Set updates the state value and notifies all observers.
func (s *StateType[T]) Set(value T) {
	s.mu.Lock()
	oldValue := s.value
	s.value = value
	observers := make([]func(T), len(s.observers))
	copy(observers, s.observers)
	s.mu.Unlock()

	// Notify observers if value changed
	if !equal(oldValue, value) {
		for _, observer := range observers {
			observer(value)
		}
	}
}

// OnChange registers a callback that will be called when the state value changes.
func (s *StateType[T]) OnChange(callback func(T)) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.observers = append(s.observers, callback)
}

// BindingType represents a two-way binding to a value.
// Similar to @Binding in SwiftUI.
type BindingType[T any] struct {
	get func() T
	set func(T)
}

// NewBinding creates a new binding with get and set functions.
func NewBinding[T any](get func() T, set func(T)) *BindingType[T] {
	return &BindingType[T]{
		get: get,
		set: set,
	}
}

// Get returns the current value.
func (b *BindingType[T]) Get() T {
	return b.get()
}

// Set updates the value.
func (b *BindingType[T]) Set(value T) {
	b.set(value)
}

// BindingFromState creates a binding from a StateType.
func BindingFromState[T any](s *StateType[T]) *BindingType[T] {
	return NewBinding(
		func() T { return s.Value() },
		func(v T) { s.Set(v) },
	)
}

// equal is a helper function to compare two values.
// For now, we use a simple comparison. This can be enhanced.
func equal[T any](a, b T) bool {
	// Simple pointer comparison for now
	// In a real implementation, you might want to use reflection
	// or require types to implement a comparable interface
	return any(a) == any(b)
}
