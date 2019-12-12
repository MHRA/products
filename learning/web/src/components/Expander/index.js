import React, { useState } from "react"
import styled from "styled-components"
import { rhythm } from "../../utils/typography"
import { mhraBlue, white, mhraGray10, black } from "../../utils/colors"
import { MdClose } from "react-icons/md"

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

const CloseButton = styled.button`
  position: absolute;
  top: 0;
  right: 0;
  color: ${black};
  background: none;
  padding: 1em;
  border: 0;
  cursor: pointer;
`

const Body = styled.div`
  position: relative;
  display: block;
  background-color: ${mhraGray10};
  padding: 3.3125rem;
  margin: 1em 0;
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
      <Body className={open ? undefined : "hidden"}>
        <CloseButton onClick={toggleOpen}>
          <MdClose />
        </CloseButton>
        {children}
      </Body>
    </ExpanderStyled>
  )
}
export default Expander
