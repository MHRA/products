import React from 'react';
import ReactGA from 'react-ga';
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
        <ReactGA.OutboundLink
          eventLabel={helpLabel}
          to={helpUrl}
          target="_blank"
        >
          {helpLabel}
        </ReactGA.OutboundLink>
      </li>
      <li>
        <ReactGA.OutboundLink
          eventLabel={downloadLabel}
          to={downloadUrl}
          target="_blank"
        >
          {downloadLabel}
        </ReactGA.OutboundLink>
      </li>
      <li>
        <ReactGA.OutboundLink
          eventLabel={toolsLabel}
          to={toolsUrl}
          target="_blank"
        >
          {toolsLabel}
        </ReactGA.OutboundLink>
      </li>
    </ul>
  </StyledPdf>
);

export default Pdf;
