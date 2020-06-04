import { Layout } from './layout'
import { Para } from './typography'
import { SummaryListWithoutActions } from './summary_list'
import { ButtonLink } from './button'

export const Success = ({ name, submittedAt }) => (
  <Layout title="Submission complete">
    <div className="govuk-panel govuk-panel--confirmation">
      <h1 className="govuk-panel__title">Submission complete</h1>
    </div>

    <Para>
      Your submission has been sent successfully. Your report should be visible
      on <a href="https://products.mhra.gov.uk/">products.mhra.gov.uk</a> within
      the next 10 minutes. If by that time the document is not visible or
      searchable on the website, please raise a ticket with{' '}
      <a href="mailto:it-helpdesk@mhra.gov.uk">it-helpdesk@mhra.gov.uk</a>
    </Para>

    <SummaryListWithoutActions
      items={[
        { key: 'Name', value: name },
        { key: 'Date', value: submittedAt.toLocaleDateString() },
        { key: 'Time', value: submittedAt.toLocaleTimeString() },
      ]}
      alignRight
    />

    <ButtonLink href="/">Submit another report</ButtonLink>
  </Layout>
)
