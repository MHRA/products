import React, { Component } from "react"
import { graphql, StaticQuery } from "gatsby"
import styled from "styled-components"
import Link from "../../components/Link"

import { MdArrowDropUp, MdArrowDropDown } from "react-icons/md"
import { sizes, media } from "../../utils/theme"
import { rhythm } from "../../utils/typography"
import { mhraBlue, anchorColour, white, mhraGray10 } from "../../utils/colors"
import uuid from "uuid/v4"

const Aside = styled.aside`
  flex: 0 0 ${rhythm(14)};
  margin: 0 0 ${rhythm(2)};
  background-color: ${mhraGray10};
  ${media.desktop`
    background: none;
    padding-right: ${rhythm(1)};
  `};
`

const SidebarStyled = styled.ul`
  list-style: none;
  margin: 0;
  padding-bottom: 1em;
  border-bottom: 4px solid ${mhraBlue};
  width: 100%;
  font-weight: normal;
  font-size: 1rem;

  &.hidden {
    display: none;
  }

  li {
    position: relative;
    margin: 0.5rem 0;
    line-height: 1.25rem;
    a {
      padding: 0.5rem 1.25rem;
      display: block;
      cursor: pointer;
      text-decoration: none;
    }
    &.current {
      border-left: 6px solid ${anchorColour};
      a {
        font-weight: bold;
        padding-left: 0.875rem;
      }
    }
  }

  ${media.desktop`
    border-bottom:none;
    margin: 0.375rem 0;
    li {
      a {
        padding: 0.375rem 1.25rem;
      }
      &:active,
      &:hover {
        background-color: ${white};
        a {
          text-decoration: underline;
        }
      }
    }
  `}
`

const MobileNav = styled.div`
  display: flex;
  flex-direction: row-reverse;
`

const MobileNavButton = styled.button`
  background: ${mhraBlue};
  color: ${white};
  display: flex;
  padding: 0.6em 0.6em 0.6em 1.4em;
  border: 0;
  line-height: 2em;
  cursor: pointer;
  span {
    margin-right: 0.5rem;
  }

  &.hidden {
    display: none;
  }
`

class Sidebar extends Component {
  constructor(props) {
    super(props)
    this.state = {
      open: false,
      width: typeof window !== `undefined` ? window.innerWidth : null,
      desktop: false,
    }
    this.handleResize = this.handleResize.bind(this)
  }

  handleResize = () => {
    this.setState({
      width: window.innerWidth,
    })
    if (this.state.width > sizes.desktop) {
      this.setState({ open: true })
    }
  }

  toggleOpen = e => {
    e.preventDefault()
    this.setState({ open: !this.state.open })
  }

  componentDidMount() {
    window.addEventListener("resize", this.handleResize)
    this.handleResize()
  }

  componentWillUnmount() {
    window.removeEventListener("resize", this.handleResize)
  }

  render() {
    const { location } = this.props
    const { open, width } = this.state

    return (
      <StaticQuery
        query={graphql`
          query sidebarContentQuery {
            allModulesJson {
              nodes {
                id
                name
                link
                module
                items {
                  name
                  link
                }
              }
            }
          }
        `}
        render={data => {
          const modules = data.allModulesJson.nodes

          const renderEntries = entries => {
            return entries.map(entry => {
              const { link, module, name, id } = entry
              const current = location.pathname === `${link}`
              return (
                <li
                  key={id ? id : uuid()}
                  className={!module && current ? "current" : undefined}
                >
                  {!module && link ? (
                    <Link to={`${link}`}>{name}</Link>
                  ) : (
                    <>{name}</>
                  )}
                </li>
              )
            })
          }

          const getCurrentModuleItems = modules => {
            const currentModule = modules.filter(entry => {
              return location.pathname.split("/").includes(entry.module)
            })[0]
            if (currentModule.items && currentModule.items.length) {
              return renderEntries(currentModule.items)
            }
          }

          return (
            <Aside>
              <MobileNav>
                <MobileNavButton
                  className={width < sizes.desktop ? undefined : "hidden"}
                  onClick={this.toggleOpen}
                >
                  <span>Contents</span>
                  {open ? (
                    <MdArrowDropUp size={"2em"}></MdArrowDropUp>
                  ) : (
                    <MdArrowDropDown size={"2em"}></MdArrowDropDown>
                  )}
                </MobileNavButton>
              </MobileNav>
              <SidebarStyled className={open ? "" : "hidden"}>
                {getCurrentModuleItems(modules)}
              </SidebarStyled>
            </Aside>
          )
        }}
      />
    )
  }
}

export default Sidebar
