import css from './review_submission.module.css'
import { Layout } from './layout'
import { H1, Para, H3 } from './typography'
import { Button, ButtonWithLinkStyles } from './button'
import { BackLink } from './back-link'
import { ErrorSummary } from './error_summary'
import { SummaryListWithoutActions } from './summary_list'

export const ReviewSubmission = ({
  steps,
  currentStepIndex,
  submit,
  submissionError,
  goBack,
  goToPage,
}) => {
  const pageData = steps.slice(0, currentStepIndex)

  const goToPrevPage = (event) => {
    event.preventDefault()
    goBack()
  }

  const title = 'Check your answers before sending the report'

  return (
    <Layout
      title={submissionError ? `Error: ${title}` : title}
      intro={<BackLink href="/" onClick={goToPrevPage} />}
    >
      {submissionError ? (
        <ErrorSummary>
          <Para>An unexpected error occurred submitting the form.</Para>
          <Para>Please check your internet connection and try again.</Para>
        </ErrorSummary>
      ) : null}

      <H1>{title}</H1>

      {pageData.map(({ type, data }, i) => {
        const key = `${type}-${i}`

        const goToFormPage = () => {
          goToPage(i)
        }

        switch (type) {
          case 'get_par':
            return (
              <ParToUpdateSummary
                key={key}
                data={data}
                goToFormPage={goToFormPage}
              />
            )
          case 'product':
            return (
              <ProductSummary
                key={key}
                data={data}
                goToFormPage={goToFormPage}
              />
            )
          case 'file':
            return (
              <FileSummary key={key} data={data} goToFormPage={goToFormPage} />
            )
          default:
            return null
        }
      })}

      <Button
        type="button"
        onClick={(event) => {
          event.preventDefault()
          submit(null)
        }}
      >
        Accept and send
      </Button>
    </Layout>
  )
}

const ProductSummary = ({ data, goToFormPage }) => (
  <SummaryWrapper title={data.get('title')} goToFormPage={goToFormPage}>
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
          key: 'Licence number',
          value: data.get('licence_number'),
        },
      ]}
    />
  </SummaryWrapper>
)

const ParToUpdateSummary = ({ data, goToFormPage }) => (
  <SummaryWrapper title="Par to update" goToFormPage={goToFormPage}>
    <SummaryListWithoutActions
      items={[
        {
          key: 'URL',
          value: (
            <a href={data.get('par_url')} target="_blank" rel="noreferrer">
              {data.get('par_url')}
            </a>
          ),
        },
      ]}
    />
  </SummaryWrapper>
)

const FileSummary = ({ data, goToFormPage }) => (
  <SummaryWrapper title="Document" goToFormPage={goToFormPage}>
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

const SummaryWrapper = ({ title, children, goToFormPage }) => {
  const onClickChange = (event) => {
    event.preventDefault()
    goToFormPage()
  }

  return (
    <div className="govuk-!-margin-bottom-9">
      <div className={css.flexRow}>
        <H3 component="h2">{title}</H3>

        <ButtonWithLinkStyles
          className={css.changeLink}
          onClick={onClickChange}
        >
          Change
        </ButtonWithLinkStyles>
      </div>

      {children}
    </div>
  )
}
