import React from "react"
import Link from "./index"
import { shallow } from "enzyme"

const location = { pathname: "test" }

describe(Link, () => {
  it("should render 🍪", () => {
    const component = shallow(
      <Link
        children={<> </>}
        to={"infinity"}
        activeClassName={"test"}
        partiallyActive={true}
      />
    )
    expect(component).toMatchSnapshot()
  })
})
