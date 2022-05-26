import React from "react"
import Layout from "./index"
import renderer from "react-test-renderer"

const location = { pathname: "test" }

describe(Layout, () => {
  it("should render", () => {
    const component = renderer.create(
        <Layout
          location={location}
          title={"Hola"}
          children={<></>}
          withSidebar={true}
        />
      )
      .toJSON()
    expect(component).toMatchSnapshot()
  })
})
