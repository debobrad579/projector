package projector

import (
	"encoding/json"
	"io"
	"maps"
	"os"
	"path/filepath"
	"slices"
)

type Projector interface {
	GetValueAll() map[string]string
	GetValue(key string) (string, bool)
	SetValue(key, value string)
	RemoveValue(key string)
}

type projector struct {
	data   map[string]map[string]string
	config *Config
}

func New(config *Config) (Projector, error) {
	file, err := os.OpenFile(config.Config, os.O_CREATE|os.O_RDONLY, 0644)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	fileData, err := io.ReadAll(file)
	if err != nil {
		return nil, err
	}

	var data map[string]map[string]string

	if err = json.Unmarshal(fileData, &data); err != nil {
		return nil, err
	}

	if data == nil {
		data = make(map[string]map[string]string)
	}

	return &projector{data: data, config: config}, nil
}

func (p *projector) GetValueAll() map[string]string {
	paths := []string{}
	for curr, prev := p.config.Pwd, ""; curr != prev; prev, curr = curr, filepath.Dir(curr) {
		if _, ok := p.data[curr]; ok {
			paths = append(paths, curr)
		}
	}

	result := make(map[string]string)
	for _, path := range slices.Backward(paths) {
		maps.Copy(result, p.data[path])
	}

	return result
}

func (p *projector) GetValue(key string) (string, bool) {
	for curr, prev := p.config.Pwd, ""; curr != prev; prev, curr = curr, filepath.Dir(curr) {
		if dir, ok := p.data[curr]; ok {
			if value, ok := dir[key]; ok {
				return value, true
			}
		}
	}

	return "", false
}

func (p *projector) SetValue(key, value string) {
	if _, ok := p.data[p.config.Pwd]; !ok {
		p.data[p.config.Pwd] = make(map[string]string)
	}

	p.data[p.config.Pwd][key] = value
}

func (p *projector) RemoveValue(key string) {
	if dir, ok := p.data[p.config.Pwd]; ok {
		delete(dir, key)
	}
}
