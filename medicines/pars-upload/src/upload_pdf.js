import { Layout } from './layout'
import { H1, H2 } from './typography'
import { BackLink } from './back-link'
import { Field } from './field'
import { Button } from './button'
import { SummaryListWithoutActions } from './summary_list'

export const UploadPdf = ({ currentStepData, goBack, submit, flowName }) => {
  const onSubmit = (event) => {
    event.preventDefault()

    const formData = new FormData(event.target)

    submit(formData)
  }

  const onContinue = () => {
    submit(currentStepData)
  }

  const goToPrevPage = (event) => {
    event.preventDefault()
    goBack()
  }

  const title =
    flowName === 'update' ? 'Upload a replacement PDF' : 'Upload your PDF'

  return (
    <Layout title={title} intro={<BackLink href="/" onClick={goToPrevPage} />}>
      <H1>{title}</H1>
      {currentStepData && (
        <>
          <CurrentlyUploadedFile
            file={currentStepData.get('file')}
            continueAction={onContinue}
          />
          <H2>Upload new file instead</H2>
        </>
      )}
      <form onSubmit={onSubmit}>
        <Field name="file" label="File" type="file" accept="application/pdf" />

        <Button>Continue</Button>
      </form>
    </Layout>
  )
}

const CurrentlyUploadedFile = ({ file, continueAction }) => (
  <div>
    <H2>Current file</H2>
    <SummaryListWithoutActions
      items={[
        {
          key: 'Document name',
          value: file.name,
        },
      ]}
    />
    <Button onClick={continueAction}>Continue</Button>
  </div>
)
