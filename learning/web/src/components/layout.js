import React from "react"
import { Link } from "gatsby"

import { rhythm } from "../utils/typography"
import SvgMhraLogo from "./Logos/mhra-logo"
import SvgAgencyDigitalLogo from "./Logos/agency-digital-logo"
import styled from "styled-components"
import { media } from "../utils/theme"
import Sidebar from "./Sidebar"

const maxWidth = rhythm(44)
const paddingTopBottom = rhythm(1.5)
const paddingLeftRight = rhythm(3 / 4)

const Header = styled.header`
  padding: ${paddingTopBottom} 0 0 0;
  max-width: ${maxWidth};
  margin: 0 auto;
`

const Content = styled.div`
  ${media.desktop`
    display: flex;
    flex: 1;
    max-width: ${maxWidth};
    margin: ${rhythm(1)} auto 0;
  `}
`

const Main = styled.main`
  flex: 2;
`

const Body = styled.div`
  border-top: 4px solid rgb(15, 18, 144);
`

const Aside = styled.aside`
  flex: 0 0 ${rhythm(12)};
  margin: ${rhythm(1)} 0 ${rhythm(2)};

  ${media.desktop`
    padding-right: ${rhythm(1)};
  `};
`

const Wrapper = styled.div`
  margin-left: auto;
  margin-right: auto;
  max-width: ${maxWidth};
`

const Footer = styled.footer`
  background-color: #ebebeb;
  padding: ${paddingTopBottom} ${paddingLeftRight};
  margin-top: 4em;
`

const FooterNav = styled.nav`
  ul {
    margin: 0;
    padding: ${rhythm(0.6)} 0;
    list-style: none;
    display: flex;
    flex-direction: column;
  }
  li {
    padding-right: 1.875rem;
  }

  a {
    color: #000;
  }

  ${media.desktop`
    ul {
      flex-direction: row;
    }
  `}
`

const HeaderLogoLink = () => (
  <Link style={{ display: "inline-block" }} to={`/`}>
    <picture>
      <SvgMhraLogo style={{ maxWidth: rhythm(8) }} />
    </picture>
  </Link>
)

class Layout extends React.Component {
  render() {
    const { location, title, children, withSidebar } = this.props
    const rootPath = `${__PATH_PREFIX__}/`
    let header

    if (location.pathname === rootPath) {
      header = (
        <>
          <HeaderLogoLink />
          <h1
            style={{
              marginBottom: paddingTopBottom,
              marginTop: paddingTopBottom,
            }}
          >
            {title}
          </h1>
        </>
      )
    } else {
      header = (
        <>
          <HeaderLogoLink />
        </>
      )
    }
    return (
      <Body>
        <div style={{ padding: `0 ${paddingLeftRight}` }}>
          <Header>{header}</Header>

          <Content>
            {withSidebar && location.pathname !== rootPath && (
              <Aside>
                <Sidebar location={location} />
              </Aside>
            )}
            <Main>{children}</Main>
          </Content>
        </div>

        <Footer>
          <Wrapper>
            <picture>
              <SvgAgencyDigitalLogo style={{ maxWidth: rhythm(10) }} />
            </picture>
            <FooterNav>
              <ul>
                <li>
                  <p>
                    <Link to="cookies">Cookie Policy</Link>
                  </p>
                </li>
                <li>
                  <p>
                    <Link to="privacy">Privacy Policy</Link>
                  </p>
                </li>
                <li>
                  <p>
                    <Link to="accessibility">Accessibility Statement</Link>
                  </p>
                </li>
                <li>
                  <p>
                    Built by the&nbsp; Medicines &amp; Healthcare products
                    Regulatory Agency © {new Date().getFullYear()}
                  </p>
                </li>
              </ul>
            </FooterNav>
          </Wrapper>
        </Footer>
      </Body>
    )
  }
}

export default Layout
