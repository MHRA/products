import React from "react"
import { Link as GatsbyLink } from "gatsby"
import styled, { css } from "styled-components"
import { anchorColour, mhra } from "../../utils/colors"
import { FiExternalLink } from "react-icons/fi"

const linkStyle = css`
  color: ${anchorColour};
  &:hover,
  &:active {
    color: ${mhra};
  }
`

const GatsbyLinkStyled = styled(GatsbyLink)`
  ${linkStyle}
`

const StyledLink = styled.a`
  ${linkStyle}
`

const ExternalLink = styled(StyledLink)`
  span {
    display: inline-block;
    padding-left: 0.3em;
  }
`

const Link = ({ children, to, activeClassName, partiallyActive, ...other }) => {
  const internal = /^\/(?!\/)/.test(to)
  const hash = /^#(?!\/)/.test(to)
  const file = /^.*\.(jpg|JPG|gif|GIF|doc|DOC|pdf|PDF)$/.test(to)

  if (internal && !file) {
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
  }

  if (hash) {
    return (
      <StyledLink href={to} {...other}>
        {children}
      </StyledLink>
    )
  }

  if (file) {
    return (
      <StyledLink href={to} {...other}>
        {children}
      </StyledLink>
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
