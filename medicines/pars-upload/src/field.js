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

  return (
    <>
      {visuallyHideLabel ? (
        <ScreenReaderOnly>{labelEl}</ScreenReaderOnly>
      ) : (
        labelEl
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
              top: -8,
              right: 10,
            }}
          >
            <DeleteIcon onClick={onClickDelete} altText="Delete substance" />
          </span>
        ) : null}
      </span>
    </>
  )
}
