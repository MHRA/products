import Link from 'next/link';
import React, { useEffect } from 'react';
import styled from 'styled-components';
import { mhra, mrha10 } from '../../styles/colors';
import { mobileBreakpoint } from '../../styles/dimensions';

const StyledCookieBanner = styled.aside`
  background-color: ${mrha10};
  padding: 0 10px 20px;

  div {
    margin: auto;
    max-width: 860px;
  }

  p {
    color: ${mhra};
    font-size: 1.1875rem;
    line-height: 1.4736842105;
  }

  button {
    appearance: none;
    background-color: ${mrha10};
    border-radius: 5px;
    border: 1px solid ${mhra};
    color: ${mhra};
    display: block;
    padding: 10px 20px;
  }

  button:hover,
  button:focus,
  button:active {
    background-color: ${mhra};
    color: ${mrha10};
  }

  @media ${mobileBreakpoint} {
    p {
      font-size: 1rem;
    }

    button {
      font-size: 0.75rem;
    }
  }
`;

const CookieBanner: React.FC = () => {
  const banner = 'showCookieBanner';
  const [cookieBanner, setCookieBanner] = React.useState(false);

  const handleOnClick = (
    event: React.MouseEvent<HTMLButtonElement, MouseEvent>,
  ): void => {
    event.preventDefault();
    window.localStorage.setItem(banner, String(false));
    setCookieBanner(false);
  };

  useEffect(() => {
    const showBanner = window.localStorage.getItem(banner);
    showBanner === 'false' ? setCookieBanner(false) : setCookieBanner(true);
  });

  return cookieBanner ? (
    <StyledCookieBanner>
      <div>
        <p>
          MHRA uses cookies which are essential for the site to work. We also
          use non-essential cookies to help us improve government digital
          services. Any data collected is anonymised. To know more about our
          policies, please go to our&nbsp;
          <Link href="cookies">
            <a>cookie policy page.</a>
          </Link>
          &nbsp;By continuing to use this site, you agree to our use of cookies.
        </p>
        <button onClick={handleOnClick}>Accept cookies</button>
      </div>
    </StyledCookieBanner>
  ) : (
    <> </>
  );
};

export default CookieBanner;
