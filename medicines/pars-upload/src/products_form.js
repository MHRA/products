import { useState, useRef } from 'react'
import { FormGroup } from './form'
import { ScreenReaderOnly } from './screen_reader_only'
import { Layout } from './layout'
import { Para, H1 } from './typography'
import { BackLink } from './back-link'
import { Field } from './field'
import { Button } from './button'

export const Products = ({
  currentStepData,
  currentStepIndex,
  steps,
  submit,
  repeatPage,
  savePageState,
  goBack,
  goToPage: go,
}) => {
  const formRef = useRef()

  const [activeSubstancesCount, setNumActiveSubstances] = useState(() =>
    currentStepData ? currentStepData.getAll('active_substance').length : 1
  )

  const getFormData = () => {
    const formData = new FormData(formRef.current)
    formData.append('title', product_title(formData))
    formData.append('license_number', license_number(formData))

    return formData
  }

  const onSubmit = (event) => {
    event.preventDefault()

    const formData = getFormData()

    console.log('submitting form', formData)
    submit(formData)
  }

  const onAddAnotherProduct = (event) => {
    event.preventDefault()

    const isValid = formRef.current.reportValidity()

    if (isValid) {
      repeatPage(getFormData())
    }
  }

  const goToPage = (newPageIndex) => {
    savePageState(getFormData())

    go(newPageIndex)
  }

  return (
    <Layout intro={<BackLink href="/" onClick={goBack} />}>
      <H1>New Public Assessment Report</H1>

      <Para>
        Your report can have one or multiple products associated with it. Please
        add one product at a time.
      </Para>

      <PreviousProductsSummary
        products={steps.filter(({ type, data }) => type === 'product' && data)}
        goToPage={goToPage}
      />

      <form onSubmit={onSubmit} ref={formRef}>
        <FormGroup>
          <Field
            name="brand_generic_name"
            label="Brand/Generic name"
            formData={currentStepData}
          />
        </FormGroup>
        <FormGroup>
          <Field name="strength" label="Strength" formData={currentStepData} />
        </FormGroup>
        <FormGroup>
          <Field
            name="pharmaceutical_dose"
            label="Pharmaceutical dose form"
            formData={currentStepData}
          />
        </FormGroup>
        {range(activeSubstancesCount).map((i) => (
          <FormGroup key={i}>
            <Field
              name="active_substance"
              label="Active substance(s)"
              index={i}
              formData={currentStepData}
            />
          </FormGroup>
        ))}
        <Button
          secondary
          type="button"
          onClick={() => {
            setNumActiveSubstances((n) => n + 1)
          }}
        >
          Add another active substance
        </Button>
        <LicenseNumber formData={currentStepData} />
        <Button secondary type="button" onClick={onAddAnotherProduct}>
          Add another product
        </Button>{' '}
        <Button>Continue</Button>
      </form>
    </Layout>
  )
}

const PreviousProductsSummary = ({ products, goToPage }) => {
  if (!products.length) {
    return null
  }

  return (
    <dl className="govuk-summary-list">
      {products.map(({ data, index }) => (
        <div key={index} className="govuk-summary-list__row">
          <dt className="govuk-summary-list__key">{product_title(data)}</dt>
          <dd className="govuk-summary-list__actions">
            <a
              href="#"
              className="govuk-link"
              onClick={(event) => {
                event.preventDefault()
                goToPage(index)
              }}
            >
              Edit<span className="govuk-visually-hidden"> product</span>
            </a>
          </dd>
        </div>
      ))}
    </dl>
  )
}

const LicenseNumber = ({ formData }) => (
  <FormGroup>
    <fieldset className="govuk-fieldset">
      <legend className="govuk-fieldset__legend govuk-fieldset__legend--s">
        <h2 className="govuk-fieldset__heading">Licence number</h2>
      </legend>
      <ScreenReaderOnly>
        <label htmlFor="license_number_type">Type</label>
      </ScreenReaderOnly>
      <select
        className="govuk-select"
        id="license_number_type"
        name="license_number_type"
        defaultValue={
          (formData && formData.get('license_number_type')) || undefined
        }
        required
      >
        <option value="PL">PL</option>
        <option value="PLPI">HR</option>
        <option value="THR">THR</option>
      </select>{' '}
      <Field
        className="govuk-input--width-5"
        name="license_part_one"
        label="First five digits"
        pattern="[0-9]{5}"
        title="5 digits"
        visuallyHideLabel
        formData={formData}
      />
      {' / '}
      <Field
        className="govuk-input--width-5"
        name="license_part_two"
        label="Last four digits"
        pattern="[0-9]{4}"
        title="4 digits"
        visuallyHideLabel
        formData={formData}
      />
    </fieldset>
  </FormGroup>
)

const product_title = (formData) =>
  [
    formData.get('brand_generic_name'),
    formData.get('strength'),
    formData.get('pharmaceutical_dose'),
    license_number(formData),
  ]
    .filter((x) => x)
    .join(', ')

const license_number = (formData) => {
  const license_type = formData.get('license_number_type')
  const part_one = formData.get('license_part_one')
  const part_two = formData.get('license_part_two')

  return `${license_type} ${part_one}/${part_two}`
}

const range = (x) => {
  const nums = []

  for (let i = 0; i < x; i += 1) {
    nums.push(i)
  }

  return nums
}
