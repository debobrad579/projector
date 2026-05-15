package main

import (
	"fmt"
	"log"

	"github.com/debobrad579/projector/internal/projector"
)

func main() {
	options, err := projector.GetOptions()
	if err != nil {
		log.Fatalf("%v", err)
	}

	cfg, err := projector.GetConfig(options)
	if err != nil {
		log.Fatalf("%v", err)
	}

	fmt.Printf("%+v", cfg)
}
