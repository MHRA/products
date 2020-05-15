import { Layout } from '../layout'
import { H1 } from '../typography'
import { Button } from '../button'
import { BackLink } from '../back-link'
import { ReviewSubmission } from '../review_submission'
import { Wizard } from '../wizard'
import { Field } from '../field'
import { Products } from '../products_form'

const ParUpload = () => (
  <Wizard
    initialSteps={[
      { type: 'product', component: Products },
      { type: 'file', component: UploadPdf },
      { type: 'review', component: ReviewSubmission },
    ]}
    success={() => <p>Yay</p>}
  />
)

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
