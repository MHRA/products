module.exports = {
  pathPrefix: process.env.PATH_PREFIX || "/",
  siteMetadata: {
    title: `Learning Modules for Continuous Professional Development`,
    author: `MHRA`,
    description: `Medicines and Healthcare products Regulatory Agency Continuous Professional Development`,
  },
  plugins: [
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        path: `${__dirname}/content/modules`,
        name: `modules`,
      },
    },
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        path: `${__dirname}/content/assets`,
        name: `assets`,
      },
    },
    `gatsby-transformer-json`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `modules`,
        path: `${__dirname}/src/modules`,
      },
    },
    `gatsby-transformer-sharp`,
    `gatsby-plugin-sharp`,
    {
      resolve: `gatsby-plugin-google-analytics`,
      options: {
        ...(process.env.GOOGLE_ANALYTICS_TACKING_ID && {
          trackingId: process.env.GOOGLE_ANALYTICS_TACKING_ID,
        }),
        head: true,
      },
    },
    {
      resolve: "gatsby-plugin-google-tagmanager",
      options: {
        ...(process.env.GOOGLE_TAG_MANAGER_ID && {
          id: process.env.GOOGLE_TAG_MANAGER_ID,
        }),
      },
    },
    `gatsby-plugin-offline`,
    `gatsby-plugin-react-helmet`,
    {
      resolve: `gatsby-plugin-typography`,
      options: {
        pathToConfigModule: `src/utils/typography`,
      },
    },
    `gatsby-plugin-styled-components`,
    {
      resolve: `gatsby-plugin-mdx`,
      options: {
        extensions: [`.mdx`, `.md`],
        gatsbyRemarkPlugins: [
          {
            resolve: `gatsby-remark-images`,
            options: {
              maxWidth: 590,
              wrapperStyle: () => `margin: 0;`,
            },
          },
          `gatsby-remark-copy-linked-files`,
        ],
      },
    },
  ],
}
