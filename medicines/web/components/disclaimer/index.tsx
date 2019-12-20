import React, { MouseEvent, useState } from 'react';
import styled from 'styled-components';
import {
  black,
  mhra70,
  mhraGray,
  mhraGray10,
  mhraGray90,
  mhraWhite,
  primaryColor,
} from '../../styles/colors';
import { mobileBreakpoint } from '../../styles/dimensions';

interface IDisclaimerProps {
  onDisclaimerAgree: (event: MouseEvent<HTMLButtonElement>) => void;
  searchTerm: string;
}

const DisclaimerWrapper = styled.div`
  border-radius: 1em;
  border: 1px solid ${mhraGray90};
  background-color: ${mhraGray10};
  padding: 4rem 5rem;

  @media ${mobileBreakpoint} {
    border-radius: 0.5rem;
    padding: 0 1rem 1rem;
  }
`;

const StyledDisclaimer = styled.section`
  margin-top: 3.125rem;
  margin-bottom: 3.125rem;

  h3 {
    margin-top: 0;
    margin-bottom: 2.2rem;
    margin-top: 2rem;
  }

  p {
    font-size: 1.188rem;
    line-height: 1.981rem;
  }

  div p:last-of-type {
    margin-bottom: 0;
  }

  form div {
    display: flex;
    margin-top: 3.24rem;
  }

  form input,
  form label {
    cursor: pointer;
  }

  input[type='checkbox'] {
    appearance: none;
    border: 1px solid ${black};
    display: block;
    height: 1.25rem;
    margin-right: 1.25rem;
    width: 1.25rem;
    min-width: 1.25rem;
  }

  input[type='checkbox']:checked {
    background-image: url('data:image/svg+xml;utf8, <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M173.898 439.404l-166.4-166.4c-9.997-9.997-9.997-26.206 0-36.204l36.203-36.204c9.997-9.998 26.207-9.998 36.204 0L192 312.69 432.095 72.596c9.997-9.997 26.207-9.997 36.204 0l36.203 36.204c9.997 9.997 9.997 26.206 0 36.204l-294.4 294.401c-9.998 9.997-26.207 9.997-36.204-.001z"/></svg>');
    background-size: calc(100% - 4px) calc(100% - 4px);
    background-position: center center;
    background-repeat: no-repeat;
  }

  button[type='submit'] {
    display: block;
    cursor: pointer;
    color: ${mhraWhite};
    background-color: ${primaryColor};
    align-self: flex-end;
    max-width: 50%;
    border-radius: 0.375rem;
    text-decoration: none;
    -webkit-appearance: none;
    border: solid 1px ${mhra70};
    padding: 0.5em 1em;
    margin-top: 1em;

    &:disabled {
      background-color: ${mhraGray};
      border: solid 1px ${mhraGray};
      cursor: not-allowed;
    }

    &:hover:enabled {
      background-color: ${mhra70};
    }
  }

  label {
    font-size: 1.125rem;
  }
`;

const Disclaimer: React.FC<IDisclaimerProps> = props => {
  const [agreed, setAgreed] = useState(false);

  const handleOnCheck = (): void => {
    setAgreed(!agreed);
  };

  return (
    <StyledDisclaimer>
      <p>
        Please read the following information and tick the box to proceed to
        view the product information in pdf format.
      </p>
      <DisclaimerWrapper>
        <h3>Disclaimer:</h3>
        <p>
          I understand that this information is a copy of the Summary of Product
          Characteristics and patient information leaflet for a medicine, which
          outline the conditions under which the medicine should be used and
          information on its known safety.
        </p>
        <p>
          I understand that this information may be updated several times during
          the product’s lifecycle, and that there could be differences between
          the information shown here and other information in the public domain.
        </p>
        <p>
          I understand that the MHRA is unable to offer medical advice and that
          if a patient has any questions about a medicine they are taking they
          should contact their doctor or pharmacist. Patients should not stop
          taking any prescribed medicines without first speaking to a healthcare
          professional. Suspected adverse reactions to a medicine can be
          reported to us on a{' '}
          <a
            target="_blank"
            rel="noopener noreferrer"
            href="https://yellowcard.mhra.gov.uk/"
          >
            Yellow Card.
          </a>
        </p>
        <p>
          I understand that the MHRA has used its best endeavours in publishing
          this information, but accept that the information may not be the most
          up to date version for this product.
        </p>

        <p>
          To view details for <em>{props.searchTerm}</em>, please read and
          accept the disclaimer.
        </p>
        <form>
          <div>
            <input
              type="checkbox"
              name="agree"
              id="agree-checkbox"
              onChange={handleOnCheck}
            />
            <label htmlFor="agree-checkbox">
              I have read and understand the disclaimer.
            </label>
          </div>
          <button
            type="submit"
            disabled={!agreed}
            onClick={props.onDisclaimerAgree}
          >
            Agree
          </button>
        </form>
      </DisclaimerWrapper>
    </StyledDisclaimer>
  );
};

export default Disclaimer;
