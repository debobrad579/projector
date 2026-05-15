package config

import (
	"errors"
	"fmt"
	"os"
	"path"
)

type Operation string

const (
	AddOperation    Operation = "add"
	RemoveOperation Operation = "remove"
	PrintOperation  Operation = "print"
)

type Config struct {
	args      []string
	operation Operation
	config    string
	pwd       string
}

func getPwd(pwd string) (string, error) {
	if pwd != "" {
		return pwd, nil
	}

	return os.Getwd()
}

func getConfigPath(config string) (string, error) {
	if config != "" {
		return config, nil
	}

	configHome, err := os.UserConfigDir()
	if err == nil {
		return path.Join(configHome, "projector", "projector.json"), nil
	}

	home, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}

	return path.Join(home, ".projector.json"), nil
}

func getOperation(args []string) Operation {
	if len(args) == 0 {
		return PrintOperation
	}

	if args[0] == "add" {
		return AddOperation
	}

	if args[0] == "remove" {
		return RemoveOperation
	}

	return PrintOperation
}

func getArgs(operation Operation, args []string) ([]string, error) {
	switch operation {
	case AddOperation:
		if len(args) != 3 {
			return []string{}, fmt.Errorf("expected 2 arguments but got %d", len(args)-1)
		}
		return args[1:], nil
	case RemoveOperation:
		if len(args) != 2 {
			return []string{}, fmt.Errorf("expected 1 argument but got %d", len(args)-1)
		}
		return args[1:], nil
	case PrintOperation:
		if len(args) > 1 {
			return []string{}, fmt.Errorf("expected 0 or 1 argument but got %d", len(args)-1)
		}
		return args, nil
	default:
		return []string{}, errors.New("invalid operation")
	}
}

func GetConfig(opts *Options) (*Config, error) {
	pwd, err := getPwd(opts.Pwd)
	if err != nil {
		return nil, err
	}

	config, err := getConfigPath(opts.Config)
	if err != nil {
		return nil, err
	}

	operation := getOperation(opts.Args)

	args, err := getArgs(operation, opts.Args)
	if err != nil {
		return nil, err
	}

	return &Config{
		args:      args,
		operation: operation,
		config:    config,
		pwd:       pwd,
	}, nil
}
