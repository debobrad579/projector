package main

import (
	"fmt"
	"log"

	"github.com/debobrad579/projector/internal/config"
)

func main() {
	options, err := config.GetOptions()
	if err != nil {
		log.Fatalf("%v", err)
	}

	fmt.Printf("%+v", options)
}
