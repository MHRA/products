import Link from 'next/link'
import classes from 'classnames'
import css from './button.module.css'

export const Button = ({ children, secondary = false, ...props }) => (
  <button
    className={classes('govuk-button', {
      'govuk-button--secondary': secondary,
    })}
    data-module="govuk-button"
    {...props}
  >
    {children}
  </button>
)

export const ButtonLink = ({ href, children, startButton = false }) => (
  <Link href={href}>
    {/* The Link element above adds the href attribute */}
    {/* eslint-disable-next-line jsx-a11y/anchor-is-valid */}
    <a
      role="button"
      draggable="false"
      className={classes('govuk-button', {
        'govuk-button--start': startButton,
      })}
      data-module="govuk-button"
    >
      {children}
      {startButton ? (
        <svg
          className="govuk-button__start-icon"
          xmlns="http://www.w3.org/2000/svg"
          width="17.5"
          height="19"
          viewBox="0 0 33 40"
          aria-hidden="true"
          focusable="false"
        >
          <path fill="currentColor" d="M0 0h13l20 20-20 20H0l20-20z" />
        </svg>
      ) : null}
    </a>
  </Link>
)

// For things which should be a button for accessibility reasons but
// design-wise should look like a link
export const ButtonWithLinkStyles = ({ children, className, ...props }) => (
  <button
    type="button"
    className={classes(css.button, 'govuk-link', className)}
    {...props}
  >
    {children}
  </button>
)
