import React from 'react';
import { JsonLd } from 'react-schemaorg';
import { Drug, ItemList, Substance } from 'schema-dts';

interface ISubstanceStructuredDataProps {
  substanceName: string;
}

interface ISubstanceListStructuredDataProps {
  substanceNames: string[];
}

interface IDrugStructuredDataProps {
  drugName: string;
}

interface IDrugListStructuredDataProps {
  drugNames: string[];
}

export const SubstanceStructuredData: React.FC<ISubstanceStructuredDataProps> = ({
  substanceName,
}) => {
  return (
    <JsonLd<Substance>
      item={{
        '@context': 'https://schema.org',
        '@type': 'Substance',
        name: substanceName,
      }}
    />
  );
};

export const SubstanceListStructuredData: React.FC<ISubstanceListStructuredDataProps> = ({
  substanceNames,
}) => {
  return (
    <JsonLd<ItemList>
      item={{
        '@context': 'https://schema.org',
        '@type': 'ItemList',
        itemListElement: substanceNames.map((substanceName, index) => {
          return {
            '@type': 'ListItem',
            position: index,
            item: {
              '@type': 'Substance',
              name: substanceName,
              url:
                'https://products.mhra.gov.uk/substance?substance=' +
                encodeURIComponent(substanceName),
            },
          };
        }),
      }}
    />
  );
};

export const DrugStructuredData: React.FC<IDrugStructuredDataProps> = ({
  drugName,
}) => {
  return (
    <JsonLd<Drug>
      item={{
        '@context': 'https://schema.org',
        '@type': 'Drug',
        name: drugName,
      }}
    />
  );
};

export const DrugListStructuredData: React.FC<IDrugListStructuredDataProps> = ({
  drugNames,
}) => {
  return (
    <JsonLd<ItemList>
      item={{
        '@context': 'https://schema.org',
        '@type': 'ItemList',
        itemListElement: drugNames.map((drugName, index) => {
          return {
            '@type': 'ListItem',
            position: index,
            item: {
              '@type': 'Drug',
              name: drugName,
              url:
                'https://products.mhra.gov.uk/product?product=' +
                encodeURIComponent(drugName),
            },
          };
        }),
      }}
    />
  );
};
