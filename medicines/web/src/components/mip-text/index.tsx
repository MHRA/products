import React from 'react';
import styled from 'styled-components';
import { mhraGray } from '../../styles/colors';
import { baseFontSize } from '../../styles/fonts';

const StyledMipText = styled.section`
  p {
    font-size: ${baseFontSize};
    line-height: 1.315;
  }

  p:first-of-type {
    margin-top: 0;
  }
`;

const MipText: React.FC = () => (
  <StyledMipText>
    <p>
      Every medicine pack includes a patient information leaflet (PIL), which
      provides information on using the medicine safely. PILs are based on the
      Summaries of Product Characteristics (SPCs) which are a description of a
      medicinal product’s properties and the conditions attached to its use.
    </p>
    <p>
      This information is arranged in an A-Z list by active substance. You can
      also use the search box on the left to find a medicine by generic name or
      brand name.
    </p>
    <p>
      This is the most up-to-date information for a medicine according to its
      licence history. PILS can be updated several times through the product’s
      lifecycle, so the PIL found with products when purchased or dispensed
      could be different.
    </p>
    <p>
      MHRA holds data for medicines that are licensed at national level. Some
      medicines are licensed centrally by the European Medicines Agency (EMA).
      For product information on these medicines, please consult the&nbsp;
      <a
        href="http://www.ema.europa.eu/ema/index.jsp?curl=pages/includes/medicines/medicines_landing_page.jsp&mid=WC0b01ac058001ce7e"
        title="European Medicines Agency"
      >
        European Medicines Agency
      </a>
    </p>
    <p>
      Product information for cancelled marketing authorisations will stay on
      the website for a year after they are cancelled. Market authorisation
      holders have 6 months to exhaust stocks after they cancel their product so
      the PILs will still be available during this period, with an extra 6
      months for patients who still have the product in their cupboards.
    </p>
    <p>
      If you have any questions or comments on this list of product information,
      please contact our&nbsp;
      <a
        href="/Contactus/CustomerServices/index.htm"
        title="Customer Services Team"
      >
        Customer Services Team
      </a>
    </p>
  </StyledMipText>
);

export default MipText;
