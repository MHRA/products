const fs = require(`fs`)
const path = require(`path`)
const minimatch = require(`minimatch`)
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

exports.onPostBuild = async ({ graphql, reporter }) => {
  // write the .pa11yci urls
  const result = await graphql(
    `
      {
        allSitePage(sort: { fields: path, order: ASC }) {
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

  const withoutTrailingSlash = path =>
    path === `/` ? path : path.replace(/\/$/, ``)

  const allSitePage = result.data.allSitePage.edges

  const excludes = [
    `/dev-404-page`,
    `/404.html`,
    `/offline-plugin-app-shell-fallback`,
  ]

  const urls = allSitePage
    .filter(
      page =>
        !excludes.some(excludedRoute =>
          minimatch(withoutTrailingSlash(page.node.path), excludedRoute)
        )
    )
    .map(page => {
      return `http://localhost:9000${page.node.path}`
    })

  fs.writeFileSync(".pa11yci", JSON.stringify({ urls: urls }, "", 2))

  reporter.success("Building pa11y urls for a11y checks ♿️")
}
