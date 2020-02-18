import React from "react"
import SEO from "./index"
import { shallow } from "enzyme"
import * as Gatsby from "gatsby"

describe(SEO, () => {
  it("should render", () => {
    const useStaticQuery = jest.spyOn(Gatsby, "useStaticQuery")
    useStaticQuery.mockImplementation(() => ({
      site: {
        siteMetadata: {
          author: "Bob Kane",
          description:
            "All brand new adventures of THE BATMAN and ROBIN, the boy wonder",
          title: "Detective Comics #27",
        },
      },
    }))
    const component = shallow(
      <SEO description={"Amazing"} lang={"en"} title={"😀"} />
    )
    expect(component).toMatchSnapshot()
  })
})
