export const H1 = ({ children, component: Component = 'h1' }) => (
  <Component className="govuk-heading-xl">{children}</Component>
)

export const H2 = ({ children, component: Component = 'h2' }) => (
  <Component className="govuk-heading-l">{children}</Component>
)

export const H3 = ({ children, component: Component = 'h3' }) => (
  <Component className="govuk-heading-m">{children}</Component>
)

export const H4 = ({ children, component: Component = 'h4' }) => (
  <Component className="govuk-heading-s">{children}</Component>
)

export const Para = ({ children }) => <p className="govuk-body">{children}</p>
