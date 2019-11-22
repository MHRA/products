import React from 'react';
import styled from 'styled-components';
import {
  black,
  mhraBlue10,
  mhraBlue80,
  primaryColor,
  white,
} from '../../styles/colors';
import {
  baseFontSize,
  baseSpace,
  headingSFontSize,
  largePaddingSize,
  tinyPaddingSize,
} from '../../styles/dimensions';
import { primaryFont } from '../../styles/fonts';

const StyledDrugList = styled.section`
  ul {
    list-style: none;
    padding: 0;
  }

  li {
    display: -webkit-box;
    display: -moz-box;
    display: box;

    -webkit-box-orient: horizontal;
    -moz-box-orient: horizontal;
    box-orient: horizontal;

    background-color: ${mhraBlue10};
    padding: ${baseSpace};
    margin: ${baseSpace} 0;
  }

  li p {
    padding: 0;
    margin: 0;
  }

  li a {
    color: ${black};
    text-decoration: none;
  }

  li .left {
    -webkit-box-flex: 0;
    -moz-box-flex: 0;
    box-flex: 0;
  }

  li .left .icon {
    background-color: ${mhraBlue80};
    color: ${white};
    font-size: ${headingSFontSize};
    font-weight: bold;
    font-family: ${primaryFont};
    padding: 10px 0;
    text-align: center;
    width: 50px;
  }

  li .right {
    -webkit-box-flex: 1;
    -moz-box-flex: 1;
    box-flex: 1;

    padding: 0 ${baseFontSize};
    vertical-align: top;
  }

  li .right .drug-name {
    font-size: ${headingSFontSize};
    font-weight: bold;
    padding-bottom: ${tinyPaddingSize};
  }

  li .right .metadata {
    font-size: ${baseFontSize};
  }

  li .right .context {
    font-size: ${headingSFontSize};
    padding-top: ${largePaddingSize};
  }

  em {
    font-weight: bold;
    font-style: normal;
  }
`;

export interface IDocument {
  activeSubstance: string;
  context: string;
  docType: string;
  fileSize: string;
  lastUpdated: string;
  name: string;
  url: string;
}

const DrugList = (props: { drugs: IDocument[] }) => (
  <StyledDrugList>
    <ul>
      {props.drugs.map((drug, i) => (
        <li key={i}>
          <div className="left">
            <p className="icon">{drug.docType.toUpperCase()}</p>
          </div>
          <div className="right">
            <a href={drug.url}>
              <p className="drug-name">
                {drug.name} ({drug.fileSize} KB)
              </p>
              <p className="metadata">
                Last updated: {drug.lastUpdated}
                <br />
                Active substance: {drug.activeSubstance}
              </p>
              <p
                className="context"
                dangerouslySetInnerHTML={{ __html: drug.context }}
              />
            </a>
          </div>
        </li>
      ))}
    </ul>
  </StyledDrugList>
);

export default DrugList;
