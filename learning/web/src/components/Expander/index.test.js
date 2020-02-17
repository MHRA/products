import React from "react"
import Expander from "./index"
import { shallow } from "enzyme"

describe(Expander, () => {
  it("should render 🍪", () => {
    const component = shallow(<Expander title={"Hola"} children={<></>} />)
    expect(component).toMatchSnapshot()
  })
})
