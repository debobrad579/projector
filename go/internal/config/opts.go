package config

import "github.com/hellflame/argparse"

type Options struct {
	Args   []string
	Pwd    string
	Config string
}

func GetOptions() (*Options, error) {
	parser := argparse.NewParser("projector", "save and get values", &argparse.ParserConfig{
		DisableDefaultShowHelp: true,
	})

	args := parser.Strings("a", "args", &argparse.Option{
		Positional: true,
	})
	pwd := parser.String("p", "pwd", &argparse.Option{
		Default: "",
	})
	config := parser.String("c", "config", &argparse.Option{
		Default: "",
	})

	if err := parser.Parse(nil); err != nil {
		return nil, err
	}

	return &Options{Args: *args, Pwd: *pwd, Config: *config}, nil
}
