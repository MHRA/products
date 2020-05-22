import { useState } from 'react'
import { Layout } from '../layout'
import { H1, Para } from '../typography'
import { Button } from '../button'
import { BackLink } from '../back-link'
import { ReviewSubmission } from '../review_submission'
import { Wizard } from '../wizard'
import { Field } from '../field'
import { Products } from '../products_form'

const ParUpload = () => {
  const [submissionState, setSubmissionState] = useState()

  const onComplete = async (steps) => {
    setSubmissionState('submitting')

    try {
      const combined = combineFormDatas(
        steps.map(({ data }) => data).filter((data) => data)
      )

      const token = 'token'

      await fetch(process.env.PARS_UPLOAD_URL, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${token}`,
        },
        body: combined,
      })

      setSubmissionState('success')
    } catch (e) {
      setSubmissionState('error')
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
        />
      )
    case 'submitting':
      return <Para>Loading</Para>
    case 'success':
      return <Para>Success!</Para>
    case 'error':
      return <Para>Error :-(</Para>
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

const UploadPdf = ({ submit }) => {
  return (
    <Layout intro={<BackLink href="/" />}>
      <H1>Upload your PDF</H1>

      <form
        onSubmit={(event) => {
          event.preventDefault()
          const formData = new FormData(event.target)
          submit(formData)
        }}
      >
        <Field name="file" label="File" type="file" />

        <Button>Continue</Button>
      </form>
    </Layout>
  )
}

export default ParUpload
