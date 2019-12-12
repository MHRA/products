import React from "react"
import { Link as GatsbyLink } from "gatsby"
import styled from "styled-components"
import { anchorColour, mhra } from "../../utils/colors"
import { FiExternalLink } from "react-icons/fi"

const GatsbyLinkStyled = styled(GatsbyLink)`
  color: ${anchorColour};
  &:hover,
  &:active {
    color: ${mhra};
  }
`
const Footnote = styled.a`
  color: ${anchorColour};
  &:hover,
  &:active {
    color: ${mhra};
  }
`

const ExternalLink = styled.a`
  color: ${anchorColour};
  &:hover,
  &:active {
    color: ${mhra};
  }
  span {
    display: inline-block;
    padding-left: 0.3em;
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
    <ExternalLink href={to} target="_blank" {...other}>
      {children}
      <span>
        <FiExternalLink />
      </span>
    </ExternalLink>
  )
}

export default Link
