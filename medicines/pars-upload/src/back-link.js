import Link from 'next/link'

export const BackLink = ({ href, onClick }) => {
  const anchor = <a className="govuk-back-link">Back</a>

  return onClick ? (
    React.cloneElement(anchor, { onClick, href })
  ) : (
    <Link href={href}>{anchor}</Link>
  )
}
