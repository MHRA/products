import Link from 'next/link';

export const BackLink = ({ href }) => (
  <Link href={href}>
    <a className="govuk-back-link">Back</a>
  </Link>
);
