import { Button } from '../button';
import { Para, H1 } from '../typography';
import { Layout } from '../layout';
import { signIn } from '../auth/authPopup';

export const SignInRequest = ({ onSignIn }) => (
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

    <form
      onSubmit={(event) => {
        event.preventDefault();
        signIn().then(onSignIn);
      }}
    >
      <Para>If you are a medical writer, you can sign in to upload PARs.</Para>
      <Para>
        <Button>Sign in</Button>
      </Para>
    </form>
  </Layout>
);
