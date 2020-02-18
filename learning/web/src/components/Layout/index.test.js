import React from "react"
import Layout from "./index"
import { shallow } from "enzyme"

const location = { pathname: "test" }

describe(Layout, () => {
  it("should render", () => {
    const component = shallow(
      <Layout
        location={location}
        title={"Hola"}
        children={<></>}
        withSidebar={true}
      />
    )
    expect(component).toMatchSnapshot()
  })
})
