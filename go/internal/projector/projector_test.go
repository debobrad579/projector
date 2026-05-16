package projector_test

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"testing"

	"github.com/debobrad579/projector/internal/projector"
)

func writeConfig(t *testing.T) string {
	t.Helper()

	dir := t.TempDir()
	path := filepath.Join(dir, "projector.json")

	bytes, err := json.Marshal(struct {
		Projector map[string]map[string]string `json:"projector"`
	}{
		Projector: map[string]map[string]string{
			"/": {
				"foo": "bar1",
				"bar": "baz",
			},
			"/foo": {
				"foo": "bar2",
			},
			"/foo/bar": {
				"foo": "bar3",
			},
		},
	})
	if err != nil {
		t.Fatal(err)
	}

	if err := os.WriteFile(path, bytes, 0644); err != nil {
		t.Fatal(err)
	}

	return path
}

func getProjector(
	t *testing.T,
	pwd string,
) projector.Projector {
	t.Helper()

	return projector.New(&projector.Config{
		Pwd:    pwd,
		Config: writeConfig(t),
	})
}

func TestGetValueAll(t *testing.T) {
	expected := map[string]string{
		"foo": "bar3",
		"bar": "baz",
	}

	if got := getProjector(t, "/foo/bar").GetValueAll(); !reflect.DeepEqual(got, expected) {
		t.Fatalf("expected '%v' but got '%v'", expected, got)
	}
}

func TestGetValue(t *testing.T) {
	projector := getProjector(t, "/foo/bar")

	if got, ok := projector.GetValue("foo"); !ok || got != "bar3" {
		t.Errorf("expected 'bar3' but got '%v'", got)
	}

	if got, ok := projector.GetValue("bar"); !ok || got != "baz" {
		t.Errorf("expected 'baz' but got '%v'", got)
	}
}

func TestSetValue(t *testing.T) {
	projector := getProjector(t, "/foo/bar")

	projector.SetValue("foo", "bar")
	if got, ok := projector.GetValue("foo"); !ok || got != "bar" {
		t.Fatalf("expected 'bar' but got '%v'", got)
	}

	projector.SetValue("bar", "baz2")
	if got, ok := projector.GetValue("bar"); !ok || got != "baz2" {
		t.Fatalf("expected 'baz2' but got '%v'", got)
	}

	projector.SetValue("baz", "bar")
	if got, ok := projector.GetValue("baz"); !ok || got != "bar" {
		t.Fatalf("expected 'bar' but got '%v'", got)
	}
}

func TestRemoveValue(t *testing.T) {
	projector := getProjector(t, "/foo/bar")

	projector.RemoveValue("foo")
	if got, ok := projector.GetValue("foo"); !ok || got != "bar2" {
		t.Fatalf("expected 'bar2' but got '%v'", got)
	}

	projector.RemoveValue("bar")
	if got, ok := projector.GetValue("bar"); !ok || got != "baz" {
		t.Fatalf("expected 'baz' but got '%v'", got)
	}
}
