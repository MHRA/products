import Link from "../Link"
import React, { useEffect } from "react"
import styled from "styled-components"
import { mhra, mhra10, mhraGray10 } from "../../utils/colors"
import { media } from "../../utils/theme"

const StyledCookieBanner = styled.aside`
  padding: 0 0.625rem 1.25rem;
  color: ${mhra};
  background-color: ${mhraGray10};

  div {
    margin: auto;
    max-width: 53.75rem;
  }

  p {
    font-size: 1rem;
    line-height: 24px;
    padding-top: 1rem;
  }

  button {
    appearance: none;
    background-color: ${mhra10};
    border-radius: 5px;
    border: 1px solid ${mhra};
    color: ${mhra};
    display: block;
    padding: 0.5rem 1rem;
    font-size: 0.75rem;
  }

  button:hover,
  button:focus,
  button:active {
    background-color: ${mhra};
    color: ${mhra10};
  }

  ${media.desktop`
    p {
      font-size: 1.1875rem;
      line-height: 1.75rem;
    }

    button {
      font-size: 1rem;
    }
  `}
`

const CookieBanner = () => {
  const banner = "showCookieBanner"
  const [cookieBanner, setCookieBanner] = React.useState(false)

  const handleOnClick = event => {
    event.preventDefault()
    window.localStorage.setItem(banner, String(false))
    setCookieBanner(false)
  }

  useEffect(() => {
    const showBanner = window.localStorage.getItem(banner)
    showBanner === "false" ? setCookieBanner(false) : setCookieBanner(true)
  })

  return cookieBanner ? (
    <StyledCookieBanner>
      <div>
        <p>
          MHRA uses cookies which are essential for the site to work. We also
          use non-essential cookies to help us improve our services. Any data
          collected is anonymised. To know more about our policies, please go to
          our&nbsp;
          <Link to="/cookies">cookie policy page</Link>
          .&nbsp;By continuing to use this site, you agree to our use of
          cookies.
        </p>
        <button onClick={handleOnClick}>Accept cookies</button>
      </div>
    </StyledCookieBanner>
  ) : (
    <> </>
  )
}

export default CookieBanner
