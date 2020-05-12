export const Layout = ({ intro = null, children }) => (
  <div className="govuk-width-container">
    {intro}

    <main className="govuk-main-wrapper" id="main-content" role="main">
      <div className="govuk-grid-row">
        <div className="govuk-grid-column-two-thirds">{children}</div>
      </div>
    </main>
  </div>
);
