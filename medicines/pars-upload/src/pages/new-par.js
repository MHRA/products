import { Layout } from '../layout'
import { H1 } from '../typography'
import { Button } from '../button'
import { BackLink } from '../back-link'
import { ReviewSubmission } from '../review_submission'
import { Wizard } from '../wizard'
import { Field } from '../field'
import { Products } from '../products_form'

const ParUpload = () => {
  const onComplete = async (steps) => {
    const combined = combineFormDatas(
      steps.map(({ data }) => data).filter((data) => data)
    )

    console.log(combined)

    const token = 'token'

    await fetch('http://localhost:8000/pars', {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
      },
      body: combined,
    })
  }

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
