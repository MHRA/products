import { Layout } from './layout'
import { H1, Para } from './typography'
import { Button } from './button'
import { BackLink } from './back-link'
import { ErrorSummary } from './error_summary'

export const ReviewSubmission = ({
  steps,
  currentStepIndex,
  submit,
  submissionError,
}) => {
  const pageData = steps.slice(0, currentStepIndex)

  console.log('XXX', pageData)

  return (
    <Layout intro={<BackLink href="/" />}>
      {submissionError ? (
        <ErrorSummary>
          <Para>An unexpected error occurred submitting the form.</Para>
          <Para>Please check your internet connection and try again.</Para>
        </ErrorSummary>
      ) : null}

      <H1>Check your answers before sending the report</H1>

      <Button type="button" onClick={submit}>
        Continue
      </Button>
    </Layout>
  )
}
