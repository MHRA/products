import React from "react"
import { graphql } from "gatsby"

import { MDXRenderer } from "gatsby-plugin-mdx"
import Layout from "../components/layout"
import SEO from "../components/seo"
import { components } from "../utils/mdx-components"
import { rhythm } from "../utils/typography"
import { MDXProvider } from "@mdx-js/react"

import styled from "styled-components"

const HR = styled.hr`
  margin-bottom: ${rhythm(1)};
`

const H1 = styled.h1`
  margin-top: ${rhythm(1)};
`

class ModuleTemplate extends React.Component {
  render() {
    const post = this.props.data.mdx
    const siteTitle = this.props.data.site.siteMetadata.title

    return (
      <Layout location={this.props.location} title={siteTitle} withSidebar>
        <SEO
          title={post.frontmatter.title}
          description={post.frontmatter.description || post.excerpt}
        />
        <article>
          <header>
            <H1>{post.frontmatter.title}</H1>
          </header>
          <MDXProvider components={components}>
            <MDXRenderer>{post.body}</MDXRenderer>
          </MDXProvider>

          <HR />
        </article>
      </Layout>
    )
  }
}

export default ModuleTemplate

export const pageQuery = graphql`
  query ModuleBySlug($slug: String!) {
    site {
      siteMetadata {
        title
      }
    }
    mdx(fields: { slug: { eq: $slug } }) {
      id
      body
      frontmatter {
        title
      }
    }
  }
`
