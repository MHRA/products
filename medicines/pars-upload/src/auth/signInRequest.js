import { Button } from '../button'
import { Para, H1 } from '../typography'
import { Layout } from '../layout'

export const SignInRequest = ({ signIn }) => (
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
  </Layout>
)
