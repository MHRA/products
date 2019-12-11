import React from "react"
import { Link as GatsbyLink } from "gatsby"
import styled from "styled-components"
import { anchorColour, mhraBlue } from "../../utils/colors"

const GatsbyLinkStyled = styled(GatsbyLink)`
  color: ${anchorColour};
  text-decoration: none;
  &:hover,
  &:active {
    color: ${mhraBlue};
    text-decoration: underline;
  }
`
const Footnote = styled.a`
  color: ${anchorColour};
  &:hover,
  &:active {
    color: ${mhraBlue};
    text-decoration: underline;
  }
`

const Link = ({ children, to, activeClassName, partiallyActive, ...other }) => {
  // Tailor the following test to your environment.
  // This example assumes that any internal link (intended for Gatsby)
  // will start with exactly one slash, and that anything else is external.
  const internal = /^\/(?!\/)/.test(to)
  const footnote = /^#(?!\/)/.test(to)
  // Use Gatsby Link for internal links, and <a> for others
  if (internal) {
    return (
      <GatsbyLinkStyled
        to={to}
        activeClassName={activeClassName}
        partiallyActive={partiallyActive}
        {...other}
      >
        {children}
      </GatsbyLinkStyled>
    )
  } else if (footnote) {
    return (
      <Footnote href={to} {...other}>
        {children}
      </Footnote>
    )
  }
  return (
    <a href={to} {...other}>
      {children}
    </a>
  )
}

export default Link
