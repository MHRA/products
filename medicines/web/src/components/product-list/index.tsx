import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';
import { IProduct } from '../../model/product';
import { pluralise } from '../../services/content-helpers';
import { mobileBreakpoint } from '../../styles/dimensions';

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

interface IIndex {
  title: string;
  products: IProduct[];
}

const ProductList: React.FC<IIndex> = ({ title, products }) => {
  if (products === undefined || products.length === 0) {
    return <></>;
  }

  const searchLink = (itemName: string) => {
    return `/product?product=${encodeURIComponent(itemName)}`;
  };

  return (
    <StyledProductList>
      <h3>{title}</h3>
      <ul>
        {products.map((product) => {
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
        })}
      </ul>
    </StyledProductList>
  );
};

export default ProductList;
