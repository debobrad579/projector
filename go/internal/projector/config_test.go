package projector_test

import (
	"reflect"
	"testing"

	"github.com/debobrad579/projector/internal/projector"
)

func TestConfig(t *testing.T) {
	tests := []struct {
		name         string
		args         []string
		operation    projector.Operation
		expectedArgs []string
	}{
		{
			name:         "print all",
			args:         []string{},
			operation:    projector.PrintOperation,
			expectedArgs: []string{},
		},
		{
			name:         "print key",
			args:         []string{"foo"},
			operation:    projector.PrintOperation,
			expectedArgs: []string{"foo"},
		},
		{
			name:         "add key value",
			args:         []string{"add", "foo", "bar"},
			operation:    projector.AddOperation,
			expectedArgs: []string{"foo", "bar"},
		},
		{
			name:         "remove key",
			args:         []string{"remove", "foo"},
			operation:    projector.RemoveOperation,
			expectedArgs: []string{"foo"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			config, err := projector.GetConfig(&projector.Options{Args: tt.args})
			if err != nil {
				t.Errorf("expected no error but got %v", err)
			}

			if config.Operation != tt.operation {
				t.Errorf("expected operation %v but got %v", tt.operation, config.Operation)
			}

			if !reflect.DeepEqual(config.Args, tt.expectedArgs) {
				t.Errorf("expected args %v but got %v", tt.expectedArgs, config.Args)
			}
		})
	}
}

func TestConfigErrors(t *testing.T) {
	tests := []struct {
		name          string
		args          []string
		expectedError string
	}{
		{
			name:          "print error",
			args:          []string{"foo", "bar"},
			expectedError: "expected 0 or 1 arguments but got 2",
		},
		{
			name:          "add error",
			args:          []string{"add", "foo"},
			expectedError: "expected 2 arguments but got 1",
		},
		{
			name:          "remove error",
			args:          []string{"remove", "foo", "bar"},
			expectedError: "expected 1 argument but got 2",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := projector.GetConfig(&projector.Options{Args: tt.args})
			if err == nil {
				t.Error("expected error no error")
				return
			}

			if err.Error() != tt.expectedError {
				t.Errorf("expected error %v but got %v", tt.expectedError, err.Error())
			}
		})
	}
}
