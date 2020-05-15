export const FormGroup = ({ children }) => (
  <div className="govuk-form-group">{children}</div>
)

export const RadioList = ({
  title,
  options,
  name,
  required = false,
  onChange = null,
}) => (
  <FormGroup>
    <fieldset className="govuk-fieldset">
      <legend className="govuk-fieldset__legend govuk-fieldset__legend--l">
        <h2 className="govuk-fieldset__heading">{title}</h2>
      </legend>
      <div className="govuk-radios">
        {options.map(({ label, value }, i) => (
          <div className="govuk-radios__item" key={value}>
            <input
              className="govuk-radios__input"
              id={inputElementId(name, i)}
              name={name}
              type="radio"
              value={value}
              required={required}
              onChange={onChange}
            />
            <label
              className="govuk-label govuk-radios__label"
              htmlFor={inputElementId(name, i)}
            >
              {label}
            </label>
          </div>
        ))}
      </div>
    </fieldset>
  </FormGroup>
)

const inputElementId = (name, index) => `form-${name}-${index}`
