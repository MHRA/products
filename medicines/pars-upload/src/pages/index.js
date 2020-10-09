import { useRouter } from 'next/router'
import { H1, Para } from '../typography'
import { Button } from '../button'
import { RadioList } from '../form'
import { Layout } from '../layout'

const Index = () => {
  const router = useRouter()

  const onSubmit = (event) => {
    event.preventDefault()

    const data = new FormData(event.target)

    router.push(data.get('what-you-up-to'))
  }

  const onChange = (event) => {
    router.prefetch(event.target.value)
  }

  return (
    <Layout title="Public Assessment Reports (PARs) upload">
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
            { value: '/new-par', label: 'Upload a new document' },
            { value: '/update-par', label: 'Update an existing document' },
          ]}
          required
          onChange={onChange}
        />

        <Button type="submit">Continue</Button>
      </form>
    </Layout>
  )
}

export default Index
