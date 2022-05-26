import React from "react"
import SEO from "./index"
import renderer from "react-test-renderer"
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
    const component = renderer
      .create(<SEO description={"Amazing"} lang={"en"} title={"😀"} />)
      .toJSON()
    expect(component).toMatchSnapshot()
  })
})
