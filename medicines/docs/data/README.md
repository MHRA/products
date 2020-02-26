# Data

Details are included here of how the data that powers the products site is stored and updated.

## Overview

The main data used by the products site relates predominantly to 3 types of document:

- Summaries of Product Characteristics (SPC)
- Patient Information Leaflet (PIL)
- Public Assessment Report (PAR)

Documents, their metadata and their contents are searchable and can be accessed and opened through the site.

Consider checking out the [architecture of the site](../architecture#progress) and the [configuration files](../../../infrastructure/) to create these resources for each environment.

## Storage

All SPC, PIL and PAR files are stored as blobs in an Azure blob container along with associated metadata fields, including:

- Substance name - e.g. "CLOPIDOGREL BESILATE"
- Doc type - e.g. “SPC”
- Facets - key words, e.g. “CLOPIDOGREL BESILATE”
- Product name - e.g. “CADIX 75MG FILM-COATED TABLETS”
- File name - original name of the file, e.g. “CON1552021698726”
- PL number - e.g. "PL403780095"
- Title - title of the document, e.g. “spc-doc_PL 40378-0095”
- Date created - upload date of document, e.g. “2019-03-08T05:08:00+00:00”
- Release state - whether the document is released, e.g. “Y”
- Revision label - version number of the file, e.g. “2”
  This index is defined in [code](https://github.com/MHRA/products/blob/master/medicines/search/src/index.rs).

Each file is stored as a blob with a name that is a hash of the file contents, e.g. `000f6ec9a52b3230d5f880e55fb33a405a1c83d3` and has a unique URI through which it can be accessed.

## Updates

SPC and PIL files are updated on a weekly basis, as the result of the batch process run on the Sentinel server. Currently, all resultant files and their metadata are manually copied and uploaded to a new container after each batch update, but this is moving towards an automatic and incremental update.

PAR files are updated on a weekly basis, as the result of a manual upload process to Sharepoint. New PAR files are uploaded and a spreadsheet is updated with a new line, containing the associated metadata.

The importer within this repository processes these files and metadata, matching the file name with the “file name” field in the metadata row and then attaching the metadata to each file, before uploading it to Azure blob storage.

## Search

Each file is discoverable through the search functionality of the products site. In order to achieve this, the Azure search service is used. There are three main parts to this service:

- Index - like a table, this has defined fields and stores a big list of entries for everything that is searchable through the service. Search results are returned from here
- Indexer - this runs periodically and fills the index with searchable entries
- Data source - this is used by the indexer to define a searchable source of data. In the case of the products site, this is the Azure blob container that holds all of the files.

Every time the contents of the blob container are updated, the indexer must run over the container again in order for these changes to be reflected in the search index.

The search mode used currently is of the "any" type, in which multiple search terms and used in an "or" fashion - a search for "ibuprofen gel" will return results for both "ibuprofen" and "gel", with matches that contain both terms appearing higher.

## Retention

Files will be retained and stored in the blob container indefinitely.

Files that should no longer be indexed and displayed on the site will be soft deleted - tagged with an "is_deleted" flag, which causes the indexer to remove them from the index.

## Security

Read-only access to the individual blob files is available to anyone who has the file’s URI, which is surfaced through the search service.

The ability to update files is restricted, only open to those with privileged access to the Azure account. Similarly, any updates to the index can only be carried out by authorised users.

## Data sensitivity

No restricted or customer-sensitive data is used on the site.

## Availability

Azure blob containers used in this project are configured to be of type RAGRS (Read-access Geo-redundant Storage), with a level of availability over a given year of 99.99999999999999% (16 9's), [as documented](https://docs.microsoft.com/en-us/azure/storage/common/storage-redundancy-grs#read-access-geo-redundant-storage).

## Redundancy and recovery

The contents of [RAGRS Azure blob containers](https://docs.microsoft.com/en-us/azure/storage/common/storage-redundancy-grs#read-access-geo-redundant-storage) are automatically replicated to a second region, so that in the case of failure or lack of availability of the primary, data can be copied over or directly read from the secondary.

## Analytics

Google Analytics is used on the site in order to track site usage and inform future improvements. Analytics are only collected for users who opt in to the cookie policy that is displayed on the site.
