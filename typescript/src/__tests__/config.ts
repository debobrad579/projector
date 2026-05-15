import { getConfig } from "../config"

test("print all", () => {
  const config = getConfig({ args: [] })
  expect(config.operation).toEqual("print")
  expect(config.args).toEqual([])
})

test("print key", () => {
  const config = getConfig({ args: ["foo"] })
  expect(config.operation).toEqual("print")
  expect(config.args).toEqual(["foo"])
})

test("print error", () => {
  expect(() => {
    getConfig({ args: ["foo", "bar"] })
  }).toThrow("expected 0 or 1 arguments but got 2")
})

test("add key value", () => {
  const config = getConfig({ args: ["add", "foo", "bar"] })
  expect(config.operation).toEqual("add")
  expect(config.args).toEqual(["foo", "bar"])
})

test("add error", () => {
  expect(() => {
    getConfig({ args: ["add", "foo"] })
  }).toThrow("expected 2 arguments but got 1")
})

test("remove key", () => {
  const config = getConfig({ args: ["remove", "foo"] })
  expect(config.operation).toEqual("remove")
  expect(config.args).toEqual(["foo"])
})

test("remove error", () => {
  expect(() => {
    getConfig({ args: ["remove", "foo", "bar"] })
  }).toThrow("expected 1 argument but got 2")
})
