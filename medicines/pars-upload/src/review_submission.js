import { Layout } from './layout'
import { H1, Para, H3 } from './typography'
import { Button } from './button'
import { BackLink } from './back-link'
import { ErrorSummary } from './error_summary'
import { SummaryListWithoutActions } from './summary_list'

export const ReviewSubmission = ({
  steps,
  currentStepIndex,
  submit,
  submissionError,
  goBack,
}) => {
  const pageData = steps.slice(0, currentStepIndex)

  const goToPrevPage = (event) => {
    event.preventDefault()
    goBack()
  }

  return (
    <Layout intro={<BackLink href="/" onClick={goToPrevPage} />}>
      {submissionError ? (
        <ErrorSummary>
          <Para>An unexpected error occurred submitting the form.</Para>
          <Para>Please check your internet connection and try again.</Para>
        </ErrorSummary>
      ) : null}

      <H1>Check your answers before sending the report</H1>

      {pageData.map(({ type, data }, i) => {
        const key = `${type}-${i}`

        switch (type) {
          case 'product':
            return <ProductSummary key={key} data={data} />
          case 'file':
            return <FileSummary key={key} data={data} />
          default:
            return null
        }
      })}

      <Button type="button" onClick={submit}>
        Accept and send
      </Button>
    </Layout>
  )
}

const ProductSummary = ({ data }) => (
  <SummaryWrapper title={data.get('title')}>
    <SummaryListWithoutActions
      items={[
        {
          key: 'Brand/Generic name',
          value: data.get('product_name'),
        },
        {
          key: 'Strength',
          value: data.get('strength'),
        },
        {
          key: 'Pharmaceutical dose form',
          value: data.get('pharmaceutical_dose'),
        },
        {
          key: `Active substance${
            data.getAll('active_substance').length > 1 ? 's' : ''
          }`,
          value: data.getAll('active_substance').join(', '),
        },
        {
          key: 'License number',
          value: data.get('license_number'),
        },
      ]}
    />
  </SummaryWrapper>
)

const FileSummary = ({ data }) => (
  <SummaryWrapper title="Document">
    <SummaryListWithoutActions
      items={[
        {
          key: 'Document name',
          value: data.get('file').name,
        },
      ]}
    />
  </SummaryWrapper>
)

const SummaryWrapper = ({ title, children }) => (
  <div className="govuk-!-margin-bottom-9">
    <H3 component="h2">{title}</H3>

    {children}
  </div>
)
