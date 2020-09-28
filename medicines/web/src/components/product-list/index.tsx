import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';
import { IProduct } from '../../model/product';
import { pluralise } from '../../services/content-helpers';
import { mobileBreakpoint } from '../../styles/dimensions';
import { errorRed } from '../../styles/colors';

const StyledProductList = styled.nav`
  h2 {
    font-size: 1.5rem;
    margin-top: 0;
  }

  ul {
    justify-content: space-between;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  ul > li {
    padding-top: 10px;
  }

  a {
    font-weight: bold;
  }

  .product-name {
    margin-bottom: 30px;
  }

  .product-name a {
    text-decoration: underline;
    font-size: 1.1875rem;
    font-weight: normal;
  }
`;

const TechnicalErrorMessage = styled.p`
  background-color: ${errorRed};
  padding: 20px;
`;

interface IIndex {
  title: string;
  products: IProduct[];
  isLoading?: boolean;
  errorFetchingResults?: boolean;
}

const ProductList: React.FC<IIndex> = ({
  title,
  products,
  isLoading,
  errorFetchingResults,
}) => {
  if (errorFetchingResults) {
    return (
      <StyledProductList>
        <TechnicalErrorMessage>
          Sorry - the site is experiencing technical issues right now. Please
          try again later.
        </TechnicalErrorMessage>
      </StyledProductList>
    );
  }

  const searchLink = (itemName: string) => {
    return `/product?product=${encodeURIComponent(itemName)}`;
  };

  const getResultListItems = () => {
    return (
      <>
        {products && products.length ? (
          products.map((product) => {
            return (
              <li key={product.name} className="product-name">
                <Link href={searchLink(product.name)}>
                  <a>
                    {product.name}{' '}
                    {product.count && (
                      <>
                        ({product.count}{' '}
                        {pluralise('file', 'files', product.count)})
                      </>
                    )}
                  </a>
                </Link>
              </li>
            );
          })
        ) : (
          <li>No results for {title}</li>
        )}
      </>
    );
  };
  return (
    <StyledProductList>
      <h2>{title}</h2>
      <ul>{isLoading ? <li>Loading results...</li> : getResultListItems()}</ul>
    </StyledProductList>
  );
};

export default ProductList;
