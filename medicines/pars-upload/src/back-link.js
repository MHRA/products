import Link from 'next/link';

export const BackLink = ({ href }) => (
  <Link href={href}>
    <a class="govuk-back-link">Back</a>
  </Link>
);
