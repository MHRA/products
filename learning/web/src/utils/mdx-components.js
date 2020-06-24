import React from "react"
import Link from "../components/Link"
import styled from "styled-components"
import { black } from "../utils/colors"

const StyledH1 = styled.h1`
  color: ${black};
`

const StyledSup = styled.sup`
  & + sup {
    padding-left: 10px;
  }
`

const H1 = props => <StyledH1 {...props} />
const A = props => <Link to={props.href} {...props} />
const Sup = props => <StyledSup {...props} />

export const components = {
  h1: H1,
  a: A,
  sup: Sup,
}
