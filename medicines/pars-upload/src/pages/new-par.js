import { useState } from 'react'
import { ReviewSubmission } from '../review_submission'
import { Wizard } from '../wizard'
import { Products } from '../products_form'
import { UploadPdf } from '../upload_pdf'
import { Success } from '../success'

const ParUpload = ({ auth }) => {
  const [submissionState, setSubmissionState] = useState()
  const [submittedAt, setSubmissionTime] = useState(() => new Date())

  const onComplete = async (steps) => {
    setSubmissionState('submitting')
    setSubmissionTime(new Date())

    try {
      const combined = combineFormDatas(
        steps.map(({ data }) => data).filter((data) => data)
      )

      const token = auth ? auth.token : 'auth-token'
      const username = auth ? auth.username : 'test-user@example.com'

      const response = await fetch(process.env.NEXT_PUBLIC_PARS_UPLOAD_URL, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${token}`,
          Username: username,
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

export default ParUpload
