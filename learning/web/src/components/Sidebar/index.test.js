import React from "react"
import Sidebar from "./index"
import { shallow } from "enzyme"
import * as Gatsby from "gatsby"

describe(Sidebar, () => {
  it("should render", () => {
    const StaticQuery = jest.spyOn(Gatsby, "StaticQuery")
    StaticQuery.mockImplementation(() => ({
      site: {
        siteMetadata: {
          author: "Bob Kane",
          description:
            "All brand new adventures of THE BATMAN and ROBIN, the boy wonder",
          title: "Detective Comics #27",
        },
      },
    }))
    const component = shallow(<Sidebar />)
    expect(component).toMatchSnapshot()
  })
})
