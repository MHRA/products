import { Button } from '../button'
import { Para, H1 } from '../typography'
import { Layout } from '../layout'

export const SignInRequest = ({ signIn, error }) => (
  <Layout>
    {error ? (
      <div
        className="govuk-error-summary"
        aria-labelledby="error-summary-title"
        role="alert"
        tabIndex="-1"
        data-module="govuk-error-summary"
      >
        <h2 className="govuk-error-summary__title" id="error-summary-title">
          There is a problem
        </h2>
        <div className="govuk-error-summary__body">
          <ul className="govuk-list govuk-error-summary__list">
            <li>{error}</li>
          </ul>
        </div>
      </div>
    ) : null}
    <H1>
      <abbr
        title="Public Assessment Reports"
        style={{ textDecoration: 'none' }}
      >
        Public Assessment Report
      </abbr>{' '}
      upload
    </H1>

    <Para>
      To access this service you need to sign in with your MHRA email address
      and password. If you can not access it, please contact your line manager.
    </Para>
    <Para>
      <Button type="button" onClick={signIn}>
        Sign in
      </Button>
    </Para>
  </Layout>
)
