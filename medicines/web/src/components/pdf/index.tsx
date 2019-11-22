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
const helpLabel = 'Help viewing PDF files';
const helpUrl = 'https://helpx.adobe.com/support/acrobat.html';
const downloadLabel = 'Download Acrobat reader for free';
const downloadUrl = 'http://www.adobe.com/products/acrobat/readstep2.html';
const toolsLabel = 'Adobe text conversion tools';
const toolsUrl =
  'http://www.adobe.com/products/acrobat/access_onlinetools.html';

const Pdf: React.FC = () => (
  <StyledPdf>
    <h2>{title}</h2>
    <ul>
      <li>
        <a href={helpUrl} target="_new">
          {helpLabel}
        </a>
      </li>
      <li>
        <a href={downloadUrl} target="_new">
          {downloadLabel}
        </a>
      </li>
      <li>
        <a href={toolsUrl} target="_new">
          {toolsLabel}
        </a>
      </li>
    </ul>
  </StyledPdf>
);

export default Pdf;
