import { Data, Projector } from "../projector"

function createData(): Data {
  return {
    projector: {
      "/": {
        foo: "bar1",
        bar: "baz",
      },
      "/foo": {
        foo: "bar2",
      },
      "/foo/bar": {
        foo: "bar3",
      },
    },
  }
}

function getProjector(pwd: string, data = createData()) {
  return new Projector({ args: [], operation: "print", pwd, config: "" }, data)
}

test("getValueAll", () => {
  expect(getProjector("/foo/bar").getValueAll()).toEqual({
    bar: "baz",
    foo: "bar3",
  })
})

test("getValue", () => {
  expect(getProjector("/foo/bar").getValue("foo")).toEqual("bar3")
  expect(getProjector("/foo/bar").getValue("bar")).toEqual("baz")
  expect(getProjector("/foo").getValue("foo")).toEqual("bar2")
  expect(getProjector("/").getValue("foo")).toEqual("bar1")
})

test("setValue", () => {
  const data = createData()
  const projector = getProjector("/foo/bar", data)
  projector.setValue("foo", "bar")
  expect(projector.getValue("foo")).toEqual("bar")
  projector.setValue("bar", "baz2")
  expect(projector.getValue("bar")).toEqual("baz2")
  expect(getProjector("/", data).getValue("bar")).toEqual("baz")
})

test("removeValue", () => {
  const data = createData()
  const projector = getProjector("/foo/bar", data)
  projector.removeValue("foo")
  expect(projector.getValue("foo")).toEqual("bar2")
  expect(getProjector("/", data).getValue("foo")).toEqual("bar1")
  projector.removeValue("bar")
  expect(projector.getValue("bar")).toEqual("baz")
})
