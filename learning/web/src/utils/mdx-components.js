import React from "react"
import styled from "styled-components"
import { black } from "../utils/colors"

const StyledH1 = styled.h1`
  color: ${black};
`

const StyledA = styled.a`
  color: ${black};
`

const H1 = props => <StyledH1 {...props} />
const A = props => <StyledA {...props} />

export const components = {
  h1: H1,
  a: A,
}
