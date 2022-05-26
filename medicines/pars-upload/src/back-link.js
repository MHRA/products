import Link from 'next/link'
import React from 'react'

export const BackLink = ({ href, onClick }) => {
  // The Link element below adds the href attribute
  // eslint-disable-next-line jsx-a11y/anchor-is-valid
  const anchor = <a className="govuk-back-link">Back</a>

  return onClick ? (
    React.cloneElement(anchor, { onClick, href })
  ) : (
    <Link href={href}>{anchor}</Link>
  )
}
