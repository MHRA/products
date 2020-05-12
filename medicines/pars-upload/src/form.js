export const RadioList = ({ title, options, name, required = false }) => (
  <div className="govuk-form-group">
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
  </div>
);

const inputElementId = (name, index) => `form-${name}-${index}`;
