import React from "react"
import styled from "styled-components"

const StyledH1 = styled.h1`
  color: red;
`

const StyledA = styled.a`
  color: #000;
`

const H1 = props => <StyledH1 {...props} />
const A = props => <StyledA {...props} />

export const components = {
  h1: H1,
  a: A,
}
