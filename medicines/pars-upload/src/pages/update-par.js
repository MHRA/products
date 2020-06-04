import { useState } from 'react'
import { ReviewSubmission } from '../review_submission'
import { Wizard } from '../wizard'
import { Products } from '../update_products_form'
import { UploadPdf } from '../upload_pdf'
import { GetParToUpdate } from '../get_par_to_update'
import { Success } from '../success'

const ParUpload = ({ auth }) => {
  const [submissionState, setSubmissionState] = useState()
  const [submittedAt, setSubmissionTime] = useState(() => new Date())

  const onComplete = async (steps) => {
    setSubmissionState('submitting')
    setSubmissionTime(new Date())

    try {
      const par_to_delete = getIdOfParToUpdate(steps)
      const combined = combineFormDatas(steps)

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

export const getIdOfParToUpdate = (steps) => {
  let updateParStep = steps.find(({ type, data }) => type == 'get_par' && data)
  if (updateParStep) {
    let url = updateParStep.data.get('par_url')
    return url.split('/').slice(-1).pop()
  }
}

export const combineFormDatas = (steps) => {
  const data = steps
    .filter(({ type }) => type != 'get_par')
    .map(({ data }) => data)
    .filter((data) => data)

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
