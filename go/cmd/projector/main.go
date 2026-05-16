package main

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/debobrad579/projector/internal/projector"
)

func main() {
	options, err := projector.GetOptions()
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	cfg, err := projector.GetConfig(options)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	proj := projector.New(cfg)

	switch cfg.Operation {
	case projector.PrintOperation:
		if len(cfg.Args) == 0 {
			data, err := json.Marshal(proj.GetValueAll())
			if err != nil {
				fmt.Println(err)
				os.Exit(1)
			}
			fmt.Println(string(data))
		} else if value, ok := proj.GetValue(cfg.Args[0]); ok {
			fmt.Println(value)
		}
	case projector.AddOperation:
		proj.SetValue(cfg.Args[0], cfg.Args[1])
		proj.Save()
	case projector.RemoveOperation:
		proj.RemoveValue(cfg.Args[0])
		proj.Save()
	}
}
