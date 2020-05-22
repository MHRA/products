import { useState } from 'react'
import { Layout } from '../layout'
import { H1, Para } from '../typography'
import { Button, ButtonLink } from '../button'
import { BackLink } from '../back-link'
import { ReviewSubmission } from '../review_submission'
import { Wizard } from '../wizard'
import { Field } from '../field'
import { Products } from '../products_form'
import { SummaryListWithoutActions } from '../summary_list'

const ParUpload = ({ account }) => {
  const [submissionState, setSubmissionState] = useState()
  const [submittedAt, setSubmissionTime] = useState(() => new Date())

  const onComplete = async (steps) => {
    setSubmissionState('submitting')
    setSubmissionTime(new Date())

    try {
      const combined = combineFormDatas(
        steps.map(({ data }) => data).filter((data) => data)
      )

      const token = 'token' // TODO

      const response = await fetch(process.env.PARS_UPLOAD_URL, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${token}`,
        },
        body: combined,
      })

      if (!response.ok) {
        throw new Error(
          `Error response from server: ${response.status} ${response.statusText}`
        )
      }

      setSubmissionState('success')
    } catch (error) {
      setSubmissionState('error')
      console.error('Error submitting form: ', error)
    }
  }

  switch (submissionState) {
    default:
      return (
        <Wizard
          initialSteps={[
            { type: 'product', component: Products },
            { type: 'file', component: UploadPdf },
            { type: 'review', component: ReviewSubmission },
          ]}
          onComplete={onComplete}
          extraProps={{
            submissionError: submissionState === 'error',
          }}
        />
      )
    case 'success':
      return (
        <Success name={account ? account.name : ''} submittedAt={submittedAt} />
      )
  }
}

const combineFormDatas = (data) => {
  const formData = new FormData()

  for (let i = 0; i < data.length; i++) {
    const page = data[i]

    for (const [name, value] of page) {
      console.log(name, value)
      formData.append(name, value)
    }
  }

  return formData
}

const UploadPdf = ({ goBack, submit }) => {
  const onSubmit = (event) => {
    event.preventDefault()
    const formData = new FormData(event.target)
    submit(formData)
  }

  return (
    <Layout intro={<BackLink href="/" onClick={goBack} />}>
      <H1>Upload your PDF</H1>

      <form onSubmit={onSubmit}>
        <Field name="file" label="File" type="file" />

        <Button>Continue</Button>
      </form>
    </Layout>
  )
}

const Success = ({ name, submittedAt }) => (
  <Layout>
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

export default ParUpload
