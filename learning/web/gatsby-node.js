const fs = require(`fs`)
const path = require(`path`)
const { createFilePath } = require(`gatsby-source-filesystem`)

exports.createPages = async ({ graphql, actions }) => {
  const { createPage } = actions

  const pageTemplate = path.resolve(`./src/templates/page.js`)
  const result = await graphql(
    `
      {
        allMdx {
          edges {
            node {
              fields {
                slug
              }
              frontmatter {
                title
              }
            }
          }
        }
      }
    `
  )

  if (result.errors) {
    throw result.errors
  }

  // Create module pages.
  const allPages = result.data.allMdx.edges

  allPages.forEach((page, index) => {
    createPage({
      path: page.node.fields.slug,
      component: pageTemplate,
      context: {
        slug: page.node.fields.slug,
        previous: null,
        next: null,
      },
    })
  })
}

exports.onCreateNode = ({ node, actions, getNode }) => {
  const { createNodeField } = actions

  if (node.internal.type === "Mdx") {
    const value = createFilePath({ node, getNode })
    createNodeField({
      name: "slug",
      node,
      value: `${value}`,
    })
  }
}

exports.onPostBuild = async ({ graphql }) => {
  // write the .pa11yci urls
  const result = await graphql(
    `
      {
        allSitePage {
          edges {
            node {
              path
            }
          }
        }
      }
    `
  )

  if (result.errors) {
    throw result.errors
  }

  const allSitePage = result.data.allSitePage.edges
  const urls = allSitePage
    .filter(page => {
      return (
        !page.node.path.includes("offline-plugin") &&
        !page.node.path.includes("dev-404-page")
      )
    })
    .map(page => {
      return `http://localhost:9000${page.node.path}`
    })

  fs.writeFileSync(".pa11yci", JSON.stringify({ urls: urls }, "", 2))
}
