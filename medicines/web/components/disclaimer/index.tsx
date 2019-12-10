import React from 'react';
import styled from 'styled-components';

const StyledDisclaimer = styled.section`
  background-color: yellow;
`;

const Disclaimer: React.FC = () => (
  <StyledDisclaimer>
    <h3>Disclaimer:</h3>
    <p>
      I understand that this information is a copy of the Summary of Product
      Characteristics and patient information leaflet for a medicine, which
      outline the conditions under which the medicine should be used and
      information on its known safety.
    </p>
    <p>
      I understand that this information may be updated several times within its
      shelf life, and that there could be differences between the version of the
      information shown here and other information in the public domain.
    </p>
    <p>
      I understand that the MHRA is unable to offer medical advice and that if a
      patient has any questions about a medicine they are taking they should
      contact their doctor or pharmacist. Patients should not stop taking any
      prescribed medicines without first speaking to a healthcare professional.
      Suspected adverse reactions to a medicine can be reported to us on a
      Yellow Card
    </p>
    <p>
      I understand that the MHRA has used its best endeavours in publishing this
      information, but accept that the information may not be the most up to
      date version for this product.
    </p>

    <form>
      <label htmlFor="agree">I have read and understand the disclaimer.</label>
      <input type="checkbox" name="agree" id="agree" />
    </form>
  </StyledDisclaimer>
);

export default Disclaimer;
