import { Para, H1 } from "../typography";
import { Button } from "../button";

const ParUpload = () => {
  return (
    <>
      <H1>New Public Assessment Report</H1>

      <Para>
        Your report can have one or multiple products associated with them.
        Please add one product at a time.
      </Para>

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

        <Button>Continue</Button>
      </form>
    </>
  );
};

const Field = ({ name, label, type = "text", required = true }) => {
  const id = `form-field-${name}`;

  return (
    <div className="govuk-form-group">
      <label className="govuk-label" htmlFor={id}>
        {label}
      </label>
      <input
        className={type === "file" ? "govuk-file-upload" : "govuk-input"}
        id={id}
        name={name}
        type={type}
        required={required}
      />
    </div>
  );
};

export default ParUpload;
