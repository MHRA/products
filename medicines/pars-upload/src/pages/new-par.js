import { useState, useRef } from 'react';
import { Layout } from '../layout';
import { Para, H1 } from '../typography';
import { Button } from '../button';
import { BackLink } from '../back-link';
import { FormGroup } from '../form';
import { ScreenReaderOnly } from '../screen_reader_only';

const ParUpload = () => {
  const formRef = useRef();
  const [pageIndex, setPageIndex] = useState(0);
  const [products, setProducts] = useState([]);

  const formData = products[pageIndex];

  const [activeSubstancesCount, setNumActiveSubstances] = useState(() =>
    formData ? formData.getAll('active_substance').length : 1,
  );

  const saveCurrentPage = () => {
    const formData = new FormData(formRef.current);

    setProducts((products) => {
      const newProducts = [...products];
      newProducts[pageIndex] = formData;
      return newProducts;
    });
  };

  const onSubmit = (event) => {
    event.preventDefault();

    saveCurrentPage();

    console.log('submitting form', formData);
  };

  const onAddAnotherProduct = (event) => {
    event.preventDefault();

    const isValid = formRef.current.reportValidity();

    if (isValid) {
      saveCurrentPage();

      setPageIndex((i) => i + 1);
    }
  };

  const goToPage = (newPageIndex) => {
    saveCurrentPage();

    setPageIndex(newPageIndex);
  };

  const onClickBack =
    pageIndex > 0
      ? (event) => {
          event.preventDefault();
          goToPage(pageIndex - 1);
        }
      : null;

  return (
    <Layout intro={<BackLink href="/" onClick={onClickBack} />}>
      <H1>New Public Assessment Report</H1>

      <Para>
        Your report can have one or multiple products associated with it. Please
        add one product at a time.
      </Para>

      <PreviousProductsSummary
        products={products.slice(0, pageIndex)}
        goToPage={goToPage}
      />

      <form key={pageIndex} onSubmit={onSubmit} ref={formRef}>
        <Field name="product_name" label="Product name" formData={formData} />
        <Field name="strength" label="Strength" formData={formData} />
        <Field
          name="pharmaceutical_dose"
          label="Pharmaceutical dose form"
          formData={formData}
        />
        {range(activeSubstancesCount).map((i) => (
          <Field
            key={i}
            name="active_substance"
            label="Active substance"
            index={i}
            formData={formData}
          />
        ))}
        <Button
          secondary
          type="button"
          onClick={() => {
            setNumActiveSubstances((n) => n + 1);
          }}
        >
          Add another active substance
        </Button>
        <LicenseNumber />
        <Button secondary type="button" onClick={onAddAnotherProduct}>
          Add another product
        </Button>{' '}
        <Button>Continue</Button>
      </form>
    </Layout>
  );
};

const PreviousProductsSummary = ({ products, goToPage }) => {
  if (!products.length) {
    return null;
  }

  return (
    <dl className="govuk-summary-list">
      {products.map((data, i) => (
        <div key={i} className="govuk-summary-list__row">
          <dt className="govuk-summary-list__key">
            {data.get('product_name')}
          </dt>
          {/* <dd className="govuk-summary-list__value">Sarah Philips</dd> */}
          <dd className="govuk-summary-list__actions">
            <a
              href="#"
              className="govuk-link"
              onClick={(event) => {
                event.preventDefault();
                goToPage(i);
              }}
            >
              Edit<span className="govuk-visually-hidden"> product</span>
            </a>
          </dd>
        </div>
      ))}
    </dl>
  );
};

const Field = ({
  name,
  label,
  type = 'text',
  formData,
  required = true,
  index = null,
}) => {
  const baseId = `form-field-${name}`;
  const id = index == null ? baseId : `${baseId}-${index}`;

  const defaultValue = formData && formData.getAll(name)[index || 0];

  return (
    <FormGroup>
      <label className="govuk-label" htmlFor={id}>
        {label}
      </label>
      <input
        className={type === 'file' ? 'govuk-file-upload' : 'govuk-input'}
        id={id}
        name={name}
        type={type}
        required={required}
        defaultValue={defaultValue}
      />
    </FormGroup>
  );
};

const LicenseNumber = () => (
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
        required
      >
        <option value="Product license">PL</option>
        <option value="PLPI">HR</option>
        <option value="THR">THR</option>
      </select>{' '}
      <ScreenReaderOnly>
        <label htmlFor="license_part_one">First chunk</label>
      </ScreenReaderOnly>
      <input
        className="govuk-input govuk-input--width-5"
        id="license_part_one"
        name="license_part_one"
        type="text"
        pattern="[0-9]{5}"
        title="5 digits"
        required
      />
      {' / '}
      <ScreenReaderOnly>
        <label htmlFor="license_part_two">Second chunk</label>
      </ScreenReaderOnly>
      <input
        className="govuk-input govuk-input--width-5"
        id="license_part_two"
        name="license_part_two"
        type="text"
        pattern="[0-9]{4}"
        title="4 digits"
        required
      />
    </fieldset>
  </FormGroup>
);

const range = (x) => {
  const nums = [];

  for (let i = 0; i < x; i += 1) {
    nums.push(i);
  }

  return nums;
};

export default ParUpload;
