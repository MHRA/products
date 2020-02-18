import React from "react"
import CookieBanner from "./index"
import { shallow } from "enzyme"

describe(CookieBanner, () => {
  it("should render 🍪", () => {
    const component = shallow(<CookieBanner />)
    expect(component).toMatchSnapshot()
  })
})
