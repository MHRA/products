import React from "react"
import { Link } from "gatsby"

import { rhythm } from "../utils/typography"
import SvgMhraLogo from "./Logos/mhra-logo"
import SvgAgencyDigitalLogo from "./Logos/agency-digital-logo"
import styled from "styled-components"
import { media, sizes } from "../utils/theme"
import Sidebar from "./Sidebar"

const maxWidth = `${sizes.desktop / 16}em`
const paddingTopBottom = rhythm(1.5)
const paddingLeftRight = rhythm(3 / 4)

const Header = styled.header`
  padding: ${paddingTopBottom} 0 0 0;
  max-width: ${maxWidth};
  margin: 0 auto;
  h1 {
    margin-bottom: ${paddingTopBottom};
    margin-top: ${paddingTopBottom};
  }
  a {
    display: inline-block;
  }
`

const HeaderLogo = styled.picture`
  display: block;
  max-width: ${rhythm(8)};
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

const LayoutStyled = styled.div`
  border-top: 4px solid rgb(15, 18, 144);
`

const LayoutGutter = styled.div`
  padding: 0 ${paddingLeftRight};
`

const Aside = styled.aside`
  flex: 0 0 ${rhythm(12)};
  margin: ${rhythm(1)} 0 ${rhythm(2)};

  ${media.desktop`
    padding-right: ${rhythm(1)};
  `};
`

const FooterContentWrapper = styled.div`
  margin: 0 auto;
  max-width: ${maxWidth};
`

const Footer = styled.footer`
  background-color: #ebebeb;
  padding: ${paddingTopBottom} ${paddingLeftRight};
  margin-top: 4em;
`

const FooterLogo = styled.picture`
  display: block;
  max-width: ${rhythm(10)};
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
  <Link to={`/`}>
    <HeaderLogo>
      <SvgMhraLogo />
    </HeaderLogo>
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
          <h1>{title}</h1>
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
      <LayoutStyled>
        <LayoutGutter>
          <Header>{header}</Header>

          <Content>
            {withSidebar && location.pathname !== rootPath && (
              <Aside>
                <Sidebar location={location} />
              </Aside>
            )}
            <Main>{children}</Main>
          </Content>
        </LayoutGutter>

        <Footer>
          <FooterContentWrapper>
            <FooterLogo>
              <SvgAgencyDigitalLogo />
            </FooterLogo>
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
          </FooterContentWrapper>
        </Footer>
      </LayoutStyled>
    )
  }
}

export default Layout
