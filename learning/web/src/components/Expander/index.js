import React, { useState } from "react"
import styled from "styled-components"
import { rhythm } from "../../utils/typography"
import { mhraBlue, white } from "../../utils/colors"

const ExpanderStyled = styled.div`
  margin-bottom: ${rhythm(0.5)};
`

const Button = styled.button`
  color: ${white};
  font-weight: normal;
  font-size: 0.875rem;
  background-color: ${mhraBlue};
  padding: 0.5em 1em;
  border: 0;
  border-radius: 0.25rem;
  cursor: pointer;
`

const Body = styled.div`
  display: block;
  padding: ${rhythm(1)};
  &.hidden {
    display: none;
  }
`

const Expander = ({ title, children }) => {
  const [open, setOpen] = useState(false)

  const toggleOpen = e => {
    e.preventDefault()
    setOpen(!open)
  }
  return (
    <ExpanderStyled>
      <Button onClick={toggleOpen}>{title}</Button>
      <Body className={open ? undefined : "hidden"}>{children}</Body>
    </ExpanderStyled>
  )
}
export default Expander
