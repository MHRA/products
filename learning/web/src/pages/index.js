import React from "react"
import { graphql } from "gatsby"

import Layout from "../components/Layout"
import SEO from "../components/SEO"
import { rhythm } from "../utils/typography"
import styled from "styled-components"
import { mhraBlue10, anchorColour, mhraBlue } from "../utils/colors"
import { GoChevronRight } from "react-icons/go"
import Link from "../components/Link"

const HomepageLink = styled.div`
  a {
    color: ${anchorColour};
    background-color: ${mhraBlue10};
    display: flex;
    align-items: center;
    height: ${rhythm(4)};
    justify-content: left;
    padding: 0 ${rhythm(1.4)};
    text-decoration: none;
    font-size: 1.2em;
    &:hover {
      padding-top: 0.25rem;
      color: ${mhraBlue};
      border-bottom: 0.25rem solid ${mhraBlue};
    }
  }
  margin-bottom: ${rhythm(1)};
`

const Icon = styled.span`
  display: flex;
  flex-direction: row-reverse;
  flex: 1;
`

class ModulesIndex extends React.Component {
  render() {
    const { data } = this.props
    const siteTitle = data.site.siteMetadata.title
    const modules = data.allModulesJson.nodes

    return (
      <Layout location={this.props.location} title={siteTitle}>
        <SEO title="Learning modules" />
        <p>We’ve produced a series of learning modules for healthcare professionals
          responsible for prescribing, supplying or administering medicines.
          These can be used by trainees, those looking to update or refresh their
          knowledge or clinicians moving into a new area. The modules cover aspects of
          medicines regulation as well as the risks of commonly-prescribed specific
          classes of medicines.</p>
        <p>These e-learning modules are under review. The content in the module has
          not been updated since it was created, and healthcare professionals should
          use caution and consider content alongside more updated resources and the
          Summary of Product Characteristics for each medicine. During the period of
          review, some technical functions may not work.</p>
        <p>We also host or link to other learning modules
          from <Link to="https://www.gov.uk/government/publications/e-learning-modules-medicines-and-medical-devices">gov.uk</Link>.</p>
        {modules.map(({ name: title, link, id }) => {
          return (
            <HomepageLink key={id}>
              <Link to={link}>
                {title}
                <Icon>
                  <GoChevronRight size={"1.5em"} />
                </Icon>
              </Link>
            </HomepageLink>
          )
        })}
      </Layout>
    )
  }
}

export default ModulesIndex

export const pageQuery = graphql`
  query {
    site {
      siteMetadata {
        title
      }
    }
    allModulesJson {
      nodes {
        id
        name
        link
        items {
          name
          link
        }
      }
    }
  }
`
