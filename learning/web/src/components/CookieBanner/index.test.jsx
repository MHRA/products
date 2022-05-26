/**
 * @jest-environment jsdom
 */

import React from "react"
import CookieBanner from "./index"
import renderer from 'react-test-renderer';

describe(CookieBanner, () => {
  it("should render 🍪", () => {
    const component = renderer.create(<CookieBanner />).toJSON()
    expect(component).toMatchSnapshot()
  })
})
