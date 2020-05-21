import { Layout } from './layout'
import { H1 } from './typography'
import { Button } from './button'
import { BackLink } from './back-link'

export const ReviewSubmission = ({ steps, currentStepIndex, submit }) => {
  const pageData = steps.slice(0, currentStepIndex)

  console.log('XXX', pageData)

  return (
    <Layout intro={<BackLink href="/" />}>
      <H1>Check your answers before sending the report</H1>

      <Button type="button" onClick={submit}>
        Continue
      </Button>
    </Layout>
  )
}
