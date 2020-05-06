const Index = () => {
  return (
    <form
    //   method="POST"
    //   action="http://localhost:8000/pars"
    //   enctype="multipart/form-data"
    >
      <Field name="product_names" label="Product names" />
      <Field name="title" label="Title" />
      <Field name="file" label="File" type="file" />
      <Field name="pl_number" label="PL Number" />
      <Field name="active_substances" label="Active substances" />
      <Field name="author" label="Author" />
      <Field name="keywords" label="Keywords" />

      <button class="govuk-button" data-module="govuk-button">
        Save and continue
      </button>
    </form>
  );
};

const Field = ({ name, label, required = true }) => {
  const id = `form-field-${name}`;

  return (
    <div className="govuk-form-group">
      <label className="govuk-label" htmlFor={id}>
        {label}
      </label>
      <input
        className="govuk-input"
        id={id}
        name={name}
        type="text"
        required={required}
      />
    </div>
  );
};

export default Index;
