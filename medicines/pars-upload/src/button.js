import Link from 'next/link';
import classes from 'classnames';

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
);

export const ButtonLink = ({ href, children }) => (
  <Link href={href}>
    <a
      role="button"
      draggable="false"
      className="govuk-button govuk-button--start"
      data-module="govuk-button"
    >
      {children}
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
    </a>
  </Link>
);
