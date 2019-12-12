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
  margin-top: ${rhythm(4)};
  margin-bottom: ${rhythm(2)};
`

const H2 = styled.h2`
  margin-top: 0.75rem;
`

class ModuleTemplate extends React.Component {
  render() {
    const post = this.props.data.mdx
    const modules = this.props.data.allModulesJson.nodes
    const currentModule = modules.filter(e => {
      return this.props.location.pathname.includes(e.module)
    })[0]

    return (
      <Layout
        location={this.props.location}
        title={currentModule.name}
        withSidebar
      >
        <SEO
          title={post.frontmatter.title}
          description={post.frontmatter.description || post.excerpt}
        />
        <article>
          <header>
            <H2>{post.frontmatter.title}</H2>
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
    mdx(fields: { slug: { eq: $slug } }) {
      id
      body
      frontmatter {
        title
      }
    }
    allModulesJson {
      nodes {
        name
        module
      }
    }
  }
`
