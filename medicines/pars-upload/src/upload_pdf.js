import { Layout } from './layout'
import { H1 } from './typography'
import { BackLink } from './back-link'
import { Field } from './field'
import { Button } from './button'

export const UploadPdf = ({ goBack, submit }) => {
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
