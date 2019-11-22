import React from 'react';
import styled from 'styled-components';
import { primaryColor } from '../../styles/colors';
import { baseSpaceSizeCss } from '../../styles/dimensions';

const StyledPdf = styled.section`
  border-top: 1px solid ${primaryColor};
  border-bottom: 1px solid ${primaryColor};
  padding: calc(${baseSpaceSizeCss} / 2) 0;

  & h2 {
    margin-top: 0;
    color: ${primaryColor};
  }

  & ul {
    padding-left: calc(${baseSpaceSizeCss} / 2);
    margin: 0;
  }
`;

const title = `Help viewing PDF's`;
const help = 'Help viewing PDF files';
const download = 'Download Acrobat reader for free';
const adobe = 'Adobe text conversion tools';

const Pdf: React.FC = () => (
  <StyledPdf>
    <h2>{title}</h2>
    <ul>
      <li>
        <a href="#">{help}</a>
      </li>
      <li>
        <a href="#">{download}</a>
      </li>
      <li>
        <a href="#">{adobe}</a>
      </li>
    </ul>
  </StyledPdf>
);

export default Pdf;
