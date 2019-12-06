import React from "react"
import { Link, graphql } from "gatsby"

import Layout from "../components/layout"
import SEO from "../components/seo"
import { rhythm } from "../utils/typography"

class ModuleIndex extends React.Component {
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
        {modules.map(({ name: title, link, description }) => {
          return (
            <article key={link}>
              <header>
                <h3
                  style={{
                    marginBottom: rhythm(1 / 4),
                  }}
                >
                  <Link style={{ boxShadow: `none` }} to={link}>
                    {title}
                  </Link>
                </h3>
              </header>
              <section>
                <p>{description}</p>
              </section>
            </article>
          )
        })}
      </Layout>
    )
  }
}

export default ModuleIndex

export const pageQuery = graphql`
  query {
    site {
      siteMetadata {
        title
      }
    }
    allMarkdownRemark(sort: { fields: [frontmatter___title] }) {
      edges {
        node {
          excerpt
          fields {
            slug
          }

          frontmatter {
            title
            description
          }
        }
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
