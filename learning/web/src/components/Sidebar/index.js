import React, { Component } from "react"
import { graphql, StaticQuery } from "gatsby"
import styled from "styled-components"
import { rhythm } from "../../utils/typography"
import Link from "../../components/Link"

import { GoThreeBars, GoX } from "react-icons/go"
import { sizes, media } from "../../utils/theme"
import { black, mhraBlue } from "../../utils/colors"
import uuid from "uuid/v4"

const SidebarStyled = styled.ul`
  display: inline-block;
  list-style: none;
  margin: 0;
  padding: 0 ${rhythm(3 / 4)} ${rhythm(1)};
  border-bottom: 0.125rem solid ${mhraBlue};
  width: 100%;
  font-size: 1.125rem;
  font-weight: bold;
  color: ${black};

  ${media.desktop`
    border-bottom:none;
  `}

  &.hidden {
    display: none;
  }

  ul {
    margin: 1.2em 0 0;
    list-style: none;
    &.modules {
      a {
        font-weight: normal;
        font-size: 1rem;
      }
    }
  }

  li {
    position: relative;
    line-height: 1.25rem;
    &.current {
      &:before {
        content: " ";
        background: ${mhraBlue};
        position: absolute;
        height: 100%;
        width: 0.25rem;
        left: ${rhythm((3 / 4) * -1)};
      }
    }
  }

  /* a {
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  } */
`

const Burger = styled.a`
  position: absolute;
  top: ${rhythm(1)};
  right: ${rhythm(1)};
  cursor: pointer;
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

          const getItems = entry => {
            if (entry.items && entry.items.length) {
              return <ul className="modules">{renderEntries(entry.items)}</ul>
            }
          }

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
                  {getItems(entry)}
                </li>
              )
            })
          }

          const loop = modules => {
            const currentModule = modules.filter(entry => {
              return location.pathname.split("/").includes(entry.module)
            })
            return renderEntries(currentModule)
          }

          return (
            <>
              <Burger
                onClick={this.toggleOpen}
                className={width < sizes.desktop ? undefined : "hidden"}
              >
                {open ? (
                  <GoX size={"2em"}></GoX>
                ) : (
                  <GoThreeBars size={"2em"}></GoThreeBars>
                )}
              </Burger>
              <SidebarStyled className={open ? "" : "hidden"}>
                {loop(modules)}
              </SidebarStyled>
            </>
          )
        }}
      />
    )
  }
}

export default Sidebar
