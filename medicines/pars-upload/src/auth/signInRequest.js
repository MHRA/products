import { Button } from '../button'
import { Para, H1 } from '../typography'
import { Layout } from '../layout'

export const SignInRequest = ({ signIn, error }) => (
  <Layout>
    <H1>
      <abbr
        title="Public Assessment Reports"
        style={{ textDecoration: 'none' }}
      >
        PARs
      </abbr>{' '}
      upload
    </H1>

    <Para>If you are a medical writer, you can sign in to upload PARs.</Para>
    <Para>
      <Button type="button" onClick={signIn}>
        Sign in
      </Button>
    </Para>
    {error ? (
      <div
        className="govuk-error-summary"
        aria-labelledby="error-summary-title"
        role="alert"
        tabindex="-1"
        data-module="govuk-error-summary"
      >
        <h2 className="govuk-error-summary__title" id="error-summary-title">
          Sign in problem
        </h2>
        <div className="govuk-error-summary__body">
          <ul className="govuk-list govuk-error-summary__list">
            <li>{error}</li>
          </ul>
        </div>
      </div>
    ) : null}
  </Layout>
)
