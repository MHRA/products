import React from 'react';
import styled from 'styled-components';
import { primaryColor } from '../../styles/colors';
import { baseSpace } from '../../styles/dimensions';

const StyledPdf = styled.section`
  border-top: 1px solid ${primaryColor};
  border-bottom: 1px solid ${primaryColor};
  padding: calc(${baseSpace} / 2) 0;

  & h2 {
    margin-top: 0;
    color: ${primaryColor};
  }

  & ul {
    padding-left: calc(${baseSpace} / 2);
    margin: 0;
  }
`;

const title = `Help viewing PDFs`;
const help = 'Help viewing PDF files';
const download = 'Download Acrobat reader for free';
const adobe = 'Adobe text conversion tools';

const Pdf: React.FC = () => (
  <StyledPdf>
    <h2>{title}</h2>
    <ul>
      <li>
        <a href="https://helpx.adobe.com/support/acrobat.html" target="_blank">
          {help}
        </a>
      </li>
      <li>
        <a
          href="http://www.adobe.com/products/acrobat/readstep2.html"
          target="_blank"
        >
          {download}
        </a>
      </li>
      <li>
        <a
          href="http://www.adobe.com/products/acrobat/access_onlinetools.html"
          target="_blank"
        >
          {adobe}
        </a>
      </li>
    </ul>
  </StyledPdf>
);

export default Pdf;
