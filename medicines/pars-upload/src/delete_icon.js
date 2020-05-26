import css from './delete_icon.module.css'

export const DeleteIcon = ({ onClick, altText }) => (
  <button type="button" onClick={onClick} className={css.icon}>
    <span className="govuk-visually-hidden">{altText}</span>
  </button>
)
