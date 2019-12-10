import React from "react"
import { Link, graphql } from "gatsby"

import Layout from "../components/layout"
import SEO from "../components/seo"
import { rhythm } from "../utils/typography"
import styled from "styled-components"
import { mhraBlue10, mhraBlue90, black } from "../utils/colors"
import { GoChevronRight } from "react-icons/go"

const HomepageLink = styled.div`
  a {
    background-color: ${mhraBlue10};
    display: flex;
    align-items: center;
    min-height: ${rhythm(4)};
    justify-content: left;
    padding: 0 ${rhythm(1.4)};
    text-decoration: none;
    color: ${black};
    font-size: 1.2em;
    &:hover {
      padding-top: 4px;
      border-bottom: 4px solid ${mhraBlue90};
    }
  }
  margin-bottom: ${rhythm(1)};
`

const Icon = styled.span`
  display: flex;
  flex-direction: row-reverse;
  flex: 1;
`

class ModulesIndex extends React.Component {
  render() {
    const { data } = this.props
    const siteTitle = data.site.siteMetadata.title
    const modules = data.allModulesJson.nodes

    return (
      <Layout location={this.props.location} title={siteTitle}>
        <SEO title="Learning modules" />
        <p>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
          eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad
          minim veniam, quis nostrud exercitation ullamco laboris nisi ut
          aliquip ex ea commodo consequat. Duis aute irure dolor in
          reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
          pariatur. Excepteur sint occaecat cupidatat non proident, sunt in
          culpa qui officia deserunt mollit anim id est laborum.
        </p>
        {modules.map(({ name: title, link }) => {
          return (
            <HomepageLink>
              <Link key={link} to={link}>
                {title}
                <Icon>
                  <GoChevronRight size={"1.2em"} />
                </Icon>
              </Link>
            </HomepageLink>
          )
        })}
      </Layout>
    )
  }
}

export default ModulesIndex

export const pageQuery = graphql`
  query {
    site {
      siteMetadata {
        title
      }
    }
    allModulesJson {
      nodes {
        id
        name
        link
        items {
          name
          link
        }
      }
    }
  }
`
