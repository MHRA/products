import { useState } from 'react'
import { Layout } from '../layout'
import { H1, Para } from '../typography'
import { Button, ButtonLink } from '../button'
import { BackLink } from '../back-link'
import { ReviewSubmission } from '../review_submission'
import { Wizard } from '../wizard'
import { Field } from '../field'
import { Products } from '../update_products_form'
import { SummaryListWithoutActions } from '../summary_list'

const ParUpload = ({ auth }) => {
  const [submissionState, setSubmissionState] = useState()
  const [submittedAt, setSubmissionTime] = useState(() => new Date())

  const onComplete = async (steps) => {
    setSubmissionState('submitting')
    setSubmissionTime(new Date())

    try {
      const par_to_delete = getIdOfParToUpdate(steps)
      const combined = combineFormDatas(
        steps
          .filter(({ type }) => type != 'get_par')
          .map(({ data }) => data)
          .filter((data) => data)
      )

      const token = auth ? auth.token : 'auth-token'
      const username = auth ? auth.username : 'test-user@example.com'

      const response = await fetch(
        `${process.env.NEXT_PUBLIC_PARS_UPLOAD_URL}/${par_to_delete}`,
        {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${token}`,
            Username: username,
          },
          body: combined,
        }
      )

      if (!response.ok) {
        throw new Error(
          `Error response from server: ${response.status} ${response.statusText}`
        )
      }

      setSubmissionState('success')
    } catch (error) {
      setSubmissionState('error')
      console.error('Error submitting form: ', error)
      window.scrollTo({
        top: 0,
        behavior: 'smooth',
      })
    }
  }

  switch (submissionState) {
    default:
      return (
        <Wizard
          initialSteps={[
            { type: 'get_par', component: GetParToUpdate },
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
        <Success
          name={auth ? auth.account.name : ''}
          submittedAt={submittedAt}
        />
      )
  }
}

const getIdOfParToUpdate = (steps) => {
  let updateParStep = steps.find(({ type, data }) => type == 'get_par' && data)
  if (updateParStep) {
    let url = updateParStep.data.get('par_url')
    return url.split('/').slice(-1).pop()
  }
}

const combineFormDatas = (data) => {
  const formData = new FormData()

  for (let i = 0; i < data.length; i++) {
    const page = data[i]

    for (const [name, value] of page) {
      formData.append(name, value)
    }
  }

  return formData
}

const GetParToUpdate = ({ goBack, submit }) => {
  const onSubmit = (event) => {
    event.preventDefault()
    const formData = new FormData(event.target)
    submit(formData)
  }

  const goToPrevPage = (event) => {
    event.preventDefault()
    goBack()
  }

  return (
    <Layout
      title="Search for an existing Public Assessment Report"
      intro={<BackLink href="/" onClick={goToPrevPage} />}
    >
      <H1>Search for an existing report</H1>

      <form onSubmit={onSubmit}>
        <Field
          title="URL"
          pattern="https:\/\/[a-zA-Z0-9.]+\/docs\/[a-zA-Z0-9]+"
          name="par_url"
          label="Please insert URL"
        />
        <Button>Search</Button>
      </form>
    </Layout>
  )
}

const UploadPdf = ({ goBack, submit }) => {
  const onSubmit = (event) => {
    event.preventDefault()
    const formData = new FormData(event.target)
    submit(formData)
  }

  const goToPrevPage = (event) => {
    event.preventDefault()
    goBack()
  }

  return (
    <Layout
      title="Upload your PDF"
      intro={<BackLink href="/" onClick={goToPrevPage} />}
    >
      <H1>Upload your PDF</H1>

      <form onSubmit={onSubmit}>
        <Field name="file" label="File" type="file" />

        <Button>Continue</Button>
      </form>
    </Layout>
  )
}

const Success = ({ name, submittedAt }) => (
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

export default ParUpload
