import { ScreenReaderOnly } from './screen_reader_only'
import { DeleteIcon } from './delete_icon'

export const Field = ({
  name,
  label,
  type = 'text',
  formData,
  className,
  visuallyHideLabel = false,
  required = true,
  index = null,
  onClickDelete,
  helpContents = null,
  ...props
}) => {
  const baseId = `form-field-${name}`
  const id = index == null ? baseId : `${baseId}-${index}`

  const defaultValue =
    (formData && formData.getAll(name)[index || 0]) || undefined
  const labelEl = (
    <label className="govuk-label" htmlFor={id}>
      {label}
    </label>
  )

  const helpEl = helpContents && (
    <div className="govuk-hint">{helpContents}</div>
  )

  return (
    <>
      {visuallyHideLabel ? (
        <ScreenReaderOnly>{labelEl}</ScreenReaderOnly>
      ) : (
        labelEl
      )}
      {visuallyHideLabel ? (
        <ScreenReaderOnly>{helpEl}</ScreenReaderOnly>
      ) : (
        helpEl
      )}
      <span style={{ position: 'relative' }}>
        <input
          className={`${
            type === 'file' ? 'govuk-file-upload' : 'govuk-input'
          } ${className}`}
          id={id}
          name={name}
          type={type}
          required={required}
          defaultValue={defaultValue}
          {...props}
        />
        {onClickDelete ? (
          <span
            style={{
              position: 'absolute',
              display: 'flex', // so that the icon is positioned the same in Chrome & FF
              top: -4,
              right: 10,
              height: 20,
            }}
          >
            <DeleteIcon onClick={onClickDelete} altText="Delete substance" />
          </span>
        ) : null}
      </span>
    </>
  )
}
