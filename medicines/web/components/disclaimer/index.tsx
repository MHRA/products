import Link from 'next/link';
import React, { ChangeEvent } from 'react';
import styled from 'styled-components';
import { anchorColour, black, mhra } from '../../styles/colors';
import { baseSpace } from '../../styles/dimensions';

interface IDisclaimerProps {
  onDisclaimerCheck: (event: ChangeEvent<HTMLInputElement>) => void;
  searchTerm: string;
}

const StyledDisclaimer = styled.section`
  margin-top: 50px;
  margin-bottom: 50px;
  div {
    border-radius: 10px;
    background-color: rgba(254, 212, 50, 0.5);
    padding: ${baseSpace};
    margin-bottom: 30px;
  }

  h3 {
    margin-top: 0;
  }

  p {
    font-size: 0.875rem;
  }

  div p:last-of-type {
    margin-bottom: 0;
  }

  a {
    color: ${anchorColour};
    &:hover {
      color: ${mhra};
    }
  }

  form {
    display: flex;
  }

  input[type='checkbox'] {
    appearance: none;
    border: 1px solid ${black};
    display: block;
    height: 20px;
    margin-right: 20px;
    width: 20px;
  }

  input[type='checkbox']:checked {
    background-image: url('data:image/svg+xml;utf8, <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M173.898 439.404l-166.4-166.4c-9.997-9.997-9.997-26.206 0-36.204l36.203-36.204c9.997-9.998 26.207-9.998 36.204 0L192 312.69 432.095 72.596c9.997-9.997 26.207-9.997 36.204 0l36.203 36.204c9.997 9.997 9.997 26.206 0 36.204l-294.4 294.401c-9.998 9.997-26.207 9.997-36.204-.001z"/></svg>');
    background-size: calc(100% - 4px) calc(100% - 4px);
    background-position: center center;
    background-repeat: no-repeat;
  }

  label {
    font-size: 1.125rem;
  }
`;

const Disclaimer: React.FC<IDisclaimerProps> = props => (
  <StyledDisclaimer>
    <p>
      Please read the following information and tick the box to proceed to view
      the product information in pdf format.
    </p>
    <div>
      <h3>Disclaimer:</h3>
      <p>
        I understand that this information is a copy of the Summary of Product
        Characteristics and patient information leaflet for a medicine, which
        outline the conditions under which the medicine should be used and
        information on its known safety.
      </p>
      <p>
        I understand that this information may be updated several times during
        the product’s lifecycle, and that there could be differences between the
        of the information shown here and other information in the public
        domain.
      </p>
      <p>
        I understand that the MHRA is unable to offer medical advice and that if
        a patient has any questions about a medicine they are taking they should
        contact their doctor or pharmacist. Patients should not stop taking any
        prescribed medicines without first speaking to a healthcare
        professional. Suspected adverse reactions to a medicine can be reported
        to us on a{' '}
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
        this information, but accept that the information may not be the most up
        to date version for this product.
      </p>
    </div>
    <p>
      To view details for <em>{props.searchTerm}</em>, please read and accept
      the disclaimer.
    </p>
    <form>
      <input
        type="checkbox"
        name="agree"
        id="agree"
        onChange={props.onDisclaimerCheck}
      />
      <label htmlFor="agree">I have read and understand the disclaimer.</label>
    </form>
  </StyledDisclaimer>
);

export default Disclaimer;
