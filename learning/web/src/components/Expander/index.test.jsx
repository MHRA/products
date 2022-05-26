import React from "react"
import Expander from "./index"
import renderer from "react-test-renderer"

describe(Expander, () => {
  it("should render", () => {
    const component = renderer.create(<Expander title={"Hola"} children={<></>} />)
      .toJSON()
    expect(component).toMatchSnapshot()
  })
})
