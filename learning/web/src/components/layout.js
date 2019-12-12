import React from "react"
import { Link } from "gatsby"

import { rhythm } from "../utils/typography"
import SvgMhraLogo from "./Logos/mhra-logo"
import SvgAgencyDigitalLogo from "./Logos/agency-digital-logo"
import styled from "styled-components"
import { media, visuallyHidden } from "../utils/theme"
import Sidebar from "./Sidebar"

const maxWidth = `${1024 / 16}em`
const paddingTopBottom = rhythm(1.5)
const paddingLeftRight = rhythm(3 / 4)

const Header = styled.header`
  padding: ${paddingTopBottom} 0 0 0;
  border-bottom: 4px solid rgb(15, 18, 144);

  h1 {
    margin-bottom: 0;
    margin-top: ${paddingTopBottom};
    padding: 0 ${paddingLeftRight} 0.5rem;
  }

  a {
    display: inline-block;
    padding: 0 ${paddingLeftRight} 0.5rem;
  }

  .visually-hidden {
    ${visuallyHidden}
  }

  ${media.desktop`
    padding: ${paddingTopBottom} ${paddingLeftRight} 0;
    border-bottom: none;
    h1 {
      border-bottom: 4px solid rgb(15, 18, 144);
    }
    h1,a {
      padding: 0 0 0.5em;
    }
  `};
`

const HeaderLogo = styled.picture`
  display: block;
  max-width: ${rhythm(8)};
`

const Main = styled.main`
  ${media.desktop`
    padding: ${paddingTopBottom} ${paddingLeftRight} 0;
  `}
`

const Content = styled.div`
  flex: 2;
  padding: 0 ${paddingLeftRight};

  ${media.desktop`
    padding: 0 1.25rem 0 0;
  `};
`

const LayoutStyled = styled.div`
  border-top: 4px solid rgb(15, 18, 144);
`

const Wrapper = styled.div`
  margin: 0 auto;
  max-width: ${maxWidth};
`

const Footer = styled.footer`
  background-color: #ebebeb;
  padding: ${paddingTopBottom} ${paddingLeftRight};
  margin-top: 4em;
`

const FlexWrapper = styled.div`
  ${media.desktop`
    display: flex;
    flex: 1;
  `}
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
    <span className="visually-hidden">Home</span>
  </Link>
)

class Layout extends React.Component {
  render() {
    const { location, title, children, withSidebar } = this.props
    const rootPath = `${__PATH_PREFIX__}/`

    return (
      <LayoutStyled>
        <Header>
          <Wrapper>
            <HeaderLogoLink />
            <h1>{title}</h1>
          </Wrapper>
        </Header>
        <Main>
          <Wrapper>
            <FlexWrapper>
              {withSidebar && location.pathname !== rootPath && (
                <Sidebar location={location} />
              )}
              <Content>{children}</Content>
            </FlexWrapper>
          </Wrapper>
        </Main>
        <Footer>
          <Wrapper>
            <FooterLogo>
              <SvgAgencyDigitalLogo />
            </FooterLogo>
            <FooterNav>
              <ul>
                <li>
                  <p>
                    <Link to="/cookies">Cookie Policy</Link>
                  </p>
                </li>
                <li>
                  <p>
                    <Link to="/privacy">Privacy Policy</Link>
                  </p>
                </li>
                <li>
                  <p>
                    <Link to="/accessibility">Accessibility Statement</Link>
                  </p>
                </li>
                <li>
                  <p>
                    Built by the Medicines &amp; Healthcare products Regulatory
                    Agency © {new Date().getFullYear()}
                  </p>
                </li>
              </ul>
            </FooterNav>
          </Wrapper>
        </Footer>
      </LayoutStyled>
    )
  }
}

export default Layout
