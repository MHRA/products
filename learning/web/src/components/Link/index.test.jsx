import React from "react"
import Link from "./index"
import renderer from "react-test-renderer"

describe(Link, () => {
  it("should render", () => {
    const component = renderer.create(
        <Link
          children={<> </>}
          to={"infinity"}
          activeClassName={"test"}
          partiallyActive={true}
        />
      )
      .toJSON()
    expect(component).toMatchSnapshot()
  })
})
