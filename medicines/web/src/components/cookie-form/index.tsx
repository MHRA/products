import React from 'react';
import styled from 'styled-components';
import Cookies from 'universal-cookie';

// @ts-ignore
import cookiesTable from '../../copy/cookies-table.md';
import { mhra, mhraWhite, primaryColor } from '../../styles/colors';
import { baseSpace } from '../../styles/dimensions';

const StyledCookieForm = styled.section`
  font-size: 19px;
  line-height: 28px;
  padding: ${baseSpace};
  padding-top: 0;

  label {
    display: inline-block;
    padding: calc(${baseSpace} * 2) 50px calc(${baseSpace} * 2) 0;
    input {
      margin-right: 30px;
    }
  }

  button {
    appearance: none;
    color: ${mhraWhite};
    background-color: ${primaryColor};
    border-radius: 5px;
    border: 1px solid ${mhra};
    display: block;
    padding: 10px 20px;
    cursor: pointer;
  }
`;

const StyledContentWrapper = styled.div`
  font-size: 19px;
  line-height: 28px;

  table {
    border-collapse: collapse;
    width: 100%;
  }

  thead {
    border-bottom: 1px solid;
  }

  th {
    text-align: start;
  }

  table,
  th,
  td {
    padding: 1rem;
  }
`;

const AccessibleHeading = styled.h3`
  visibility: hidden;
  width: 0;
  height: 0;
  margin: 0;
`;

const cookieDomain = (process.env.ROOT_URL_DOMAIN as string) || 'localhost';

interface ICookieForm {
  storageAllowed: boolean;
  setStorageAllowed: any;
}

const CookieForm: React.FC<ICookieForm> = (props) => {
  // NOTE: By "cookies", we mean "cookies and similar technologies". This includes
  // local storage, session storage, etc.

  const [cookieToggle, setCookieToggle] = React.useState(props.storageAllowed);

  const handleCookieFormSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (cookieToggle) {
      // The second true is to stop the "We don't have the user's permission to set a
      // value in local storage" error.
      props.setStorageAllowed(true, true);
    } else {
      props.setStorageAllowed(false, true);

      // Delete *everything*.
      window.localStorage.clear();
      window.sessionStorage.clear();
      const cookies = new Cookies();
      for (const cookieName of Object.keys(cookies.getAll())) {
        cookies.set(cookieName, {}, { path: '/', expires: new Date() });
        cookies.remove(cookieName, { path: '/', domain: cookieDomain });
      }
    }

    // Navigate to the home page. Seems to be the only way to stop analytics.
    window.location.href = '/';
  };

  const handleCookiesOn = () => {
    setCookieToggle(true);
  };

  const handleCookiesOff = () => {
    setCookieToggle(false);
  };

  return (
    <StyledCookieForm>
      <AccessibleHeading>Toggle cookie permissions</AccessibleHeading>
      <form onSubmit={handleCookieFormSubmit}>
        <p>
          <label htmlFor="cookie-on">
            <input
              type="radio"
              name="cookie"
              id="cookie-on"
              value="on"
              onChange={handleCookiesOn}
              checked={cookieToggle}
              aria-checked={cookieToggle}
            />
            On
          </label>
          <label htmlFor="cookie-off">
            <input
              type="radio"
              name="cookie"
              id="cookie-off"
              value="off"
              onChange={handleCookiesOff}
              checked={!cookieToggle}
              aria-checked={cookieToggle}
            />
            Off
          </label>
        </p>
        <StyledContentWrapper
          dangerouslySetInnerHTML={{ __html: cookiesTable }}
        />
        <p>
          <button>Save your preferences</button>
        </p>
      </form>
    </StyledCookieForm>
  );
};

export default CookieForm;
