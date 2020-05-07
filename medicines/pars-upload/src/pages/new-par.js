import { Para, H1 } from "../typography";
import { Button } from "../button";
import { useState } from "react";

const ParUpload = () => {
  const [pageNumber, setPageNumber] = useState(0);
  const [formState, setFormState] = useState(null);
  const onSubmit = (event) => {
    event.persist();

    console.log("submit", event);

    const formData = new FormData(event.target);

    setFormState(formData);

    setPageNumber(1);
  };

  if (typeof window != "undefined") {
    window.formData = formState;
  }

  console.log("form data", formState ? formState.get("file") : null);

  return pageNumber == 0 ? (
    <>
      <H1>New Public Assessment Report</H1>

      <Para>
        Your report can have one or multiple products associated with them.
        Please add one product at a time.
      </Para>

      <form onSubmit={onSubmit}>
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
  ) : (
    <Para>Hello</Para>
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
