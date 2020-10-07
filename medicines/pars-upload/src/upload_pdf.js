import { useState } from 'react'
import { FormGroup } from './form'
import { Layout } from './layout'
import { H1, H2 } from './typography'
import { BackLink } from './back-link'
import { Field } from './field'
import { Button } from './button'
import { SummaryListWithoutActions } from './summary_list'

export const UploadPdf = ({ currentStepData, goBack, submit, flowName }) => {
  const [formIsValid, setFormIsValid] = useState(true)

  const onSubmit = (event) => {
    event.preventDefault()

    setFormIsValid(true)
    const formData = new FormData(event.target)

    submit(formData)
  }

  const onContinue = (event) => {
    event.preventDefault()
    submit(currentStepData)
  }

  const goToPrevPage = (event) => {
    event.preventDefault()
    goBack()
  }

  const onInvalid = () => {
    setFormIsValid(false)
  }

  const checkFileType = (e) => {
    let el = e.target
    let fileTypeValid = el.value.endsWith('.pdf')
    el.setCustomValidity(fileTypeValid ? '' : 'Only PDF files are allowed')
  }

  const title =
    flowName === 'update' ? 'Upload a replacement PDF' : 'Upload your PDF'

  return (
    <Layout
      title={formIsValid ? title : `Error: ${title}`}
      intro={<BackLink href="/" onClick={goToPrevPage} />}
    >
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
      <form onSubmit={onSubmit} onInvalid={onInvalid}>
        <FormGroup>
          <Field
            name="file"
            label="File"
            type="file"
            accept="application/pdf"
            onInput={checkFileType}
          />
        </FormGroup>
        <Button type="submit">Continue</Button>
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
