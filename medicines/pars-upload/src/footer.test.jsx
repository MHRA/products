import React from 'react'
import renderer from 'react-test-renderer'
import { Footer } from './footer'

describe(Footer, () => {
  it('should render', () => {
    const component = renderer.create(<Footer />).toJSON()
    expect(component).toMatchSnapshot()
  })
})
