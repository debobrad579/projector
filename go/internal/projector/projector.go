package projector

import (
	"encoding/json"
	"errors"
	"maps"
	"os"
	"path/filepath"
	"slices"
)

type Projector interface {
	Save() error
	GetValueAll() map[string]string
	GetValue(key string) (string, bool)
	SetValue(key, value string)
	RemoveValue(key string)
}

type data struct {
	Projector map[string]map[string]string `json:"projector"`
}

type projector struct {
	data   data
	config *Config
}

func defaultProjector(config *Config) Projector {
	return &projector{data: data{Projector: make(map[string]map[string]string)}, config: config}
}

func New(config *Config) Projector {
	if _, err := os.Stat(config.Config); err != nil && errors.Is(err, os.ErrNotExist) {
		return defaultProjector(config)
	}

	fileData, err := os.ReadFile(config.Config)
	if err != nil {
		return defaultProjector(config)
	}

	var data data

	if err = json.Unmarshal(fileData, &data); err != nil {
		return defaultProjector(config)
	}

	if data.Projector == nil {
		data.Projector = make(map[string]map[string]string)
	}

	return &projector{data: data, config: config}
}

func (p *projector) Save() error {
	dir := filepath.Dir(p.config.Config)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	file, err := os.OpenFile(
		p.config.Config,
		os.O_CREATE|os.O_WRONLY|os.O_TRUNC,
		0644,
	)
	if err != nil {
		return err
	}
	defer file.Close()

	return json.NewEncoder(file).Encode(p.data)
}

func (p *projector) GetValueAll() map[string]string {
	paths := []string{}
	for curr, prev := p.config.Pwd, ""; curr != prev; prev, curr = curr, filepath.Dir(curr) {
		if _, ok := p.data.Projector[curr]; ok {
			paths = append(paths, curr)
		}
	}

	result := make(map[string]string)
	for _, path := range slices.Backward(paths) {
		maps.Copy(result, p.data.Projector[path])
	}

	return result
}

func (p *projector) GetValue(key string) (string, bool) {
	for curr, prev := p.config.Pwd, ""; curr != prev; prev, curr = curr, filepath.Dir(curr) {
		if dir, ok := p.data.Projector[curr]; ok {
			if value, ok := dir[key]; ok {
				return value, true
			}
		}
	}

	return "", false
}

func (p *projector) SetValue(key, value string) {
	if _, ok := p.data.Projector[p.config.Pwd]; !ok {
		p.data.Projector[p.config.Pwd] = make(map[string]string)
	}

	p.data.Projector[p.config.Pwd][key] = value
}

func (p *projector) RemoveValue(key string) {
	if dir, ok := p.data.Projector[p.config.Pwd]; ok {
		delete(dir, key)
	}
}
