import { Layout } from './layout'
import { H1, H2 } from './typography'
import { BackLink } from './back-link'
import { Field } from './field'
import { Button } from './button'
import { FormGroup } from './form'

export const GetParToUpdate = ({ currentStepData, goBack, submit }) => {
  const onSubmit = (event) => {
    event.preventDefault()
    const formData = new FormData(event.target)
    submit(formData)
  }

  return (
    <Layout
      title="Search for an existing Public Assessment Report"
      intro={<BackLink href="/" onClick={goBack} />}
    >
      <H1>Search for an existing Public Assessment Report</H1>
      <H2>Search for an existing report</H2>
      <form onSubmit={onSubmit}>
        <FormGroup>
          <Field
            title="URL"
            pattern="https:\/\/[a-zA-Z0-9.]+\/docs\/[a-zA-Z0-9]+"
            name="par_url"
            label="Please insert URL"
            formData={currentStepData}
            helpContents={
              <span>
                In order to find the right document, please search for the
                existing document on{' '}
                <a
                  href="https://products.mhra.gov.uk"
                  target="_blank"
                  rel="noreferrer"
                >
                  products.mhra.gov.uk
                </a>
              </span>
            }
          />
        </FormGroup>
        <Button type="submit">Continue</Button>
      </form>
    </Layout>
  )
}
