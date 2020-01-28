import React, { FormEvent } from 'react';
import styled from 'styled-components';
import Cookies from 'universal-cookie';

import { useLocalStorage } from '../../hooks';
import {
  black,
  mhra70,
  mhraBlue90,
  mhraGray,
  mhraWhite,
  primaryColor,
  white,
} from '../../styles/colors';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';

const StyledCookieForm = styled.section`
  label {
    padding: ${baseSpace};
    padding-right: calc(${baseSpace} + 10px);
    input {
      margin-right: 30px;
    }
  }
`;

interface ICookieForm {
  storageAllowed: boolean;
  setStorageAllowed: any;
}

const CookieForm: React.FC<ICookieForm> = props => {
  // NOTE: By "cookies", we mean "cookies and similar technologies". This includes
  // local storage, session storage, etc.

  const handleCookiesOn = () => {
    props.setStorageAllowed(true, true);
  };

  const handleCookiesOff = () => {
    props.setStorageAllowed(false);

    // Delete *everything*
    window.localStorage.clear();
    window.sessionStorage.clear();
    const cookies = new Cookies();
    for (const cookieName of Object.keys(cookies.getAll())) {
      cookies.remove(cookieName);
    }

    // Reload the page. Seems to be the only way to stop analytics.
    window.location.reload();
  };

  return (
    <StyledCookieForm>
      <label htmlFor="cookie-on">
        <input
          type="radio"
          name="cookie"
          id="cookie-on"
          checked={props.storageAllowed}
          onChange={handleCookiesOn}
          role="button"
        />
        On
      </label>
      <label htmlFor="cookie-off">
        <input
          type="radio"
          name="cookie"
          id="cookie-off"
          checked={!props.storageAllowed}
          onChange={handleCookiesOff}
          role="button"
        />
        Off
      </label>
    </StyledCookieForm>
  );
};

export default CookieForm;
