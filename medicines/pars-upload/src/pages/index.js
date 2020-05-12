import { H1, Para, H2 } from '../typography';
import { Button } from '../button';
import { RadioList } from '../form';

const Index = () => {
  const onSubmit = (event) => {
    event.preventDefault();

    const data = new FormData(event.target);

    console.log(data.get('what-you-up-to'));
  };

  return (
    <>
      <H1>
        <abbr
          title="Public Assessment Reports"
          style={{ textDecoration: 'none' }}
        >
          PARs
        </abbr>{' '}
        upload
      </H1>

      <form onSubmit={onSubmit}>
        <Para>
          Your report can have one or multiple products associated with them,
          please add all of those on the following fields.
        </Para>

        <RadioList
          title="What are you doing today?"
          name="what-you-up-to"
          options={[
            { value: 'upload-new', label: 'Upload a new document' },
            { value: 'update-existing', label: 'Update an existing document' },
          ]}
          required
        />

        <Button>Continue</Button>
      </form>
    </>
  );
};

export default Index;
