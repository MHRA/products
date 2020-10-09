import Head from 'next/head'

export const Layout = ({ title, intro = null, children }) => (
  <div className="govuk-width-container">
    <Head>
      <title>{title}</title>
      <meta name="robots" content="noindex, no follow" />
    </Head>

    {intro}

    <main className="govuk-main-wrapper" id="main-content" role="main">
      <div className="govuk-grid-row">
        <div className="govuk-grid-column-two-thirds">{children}</div>
      </div>
    </main>
  </div>
)
