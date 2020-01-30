# Stub API

## Purpose

The stub API is built to unblock integration testing for Sentinel, **and has
no practical functionality**. It does not interact with the Sentinel server
or Azure Blob storage in any way, it simply uses in-memory storage to mock
the actual API functionality.

The intended usage pattern is as follows:

- Send a DELETE or POST request for a document;
- Continue to poll the GET endpoint for that document until the status has
updated.

In the background, the real API will asynchronously process the request and
update the status of the document as it progresses.

## Running

**NOTE: THIS STILL NEEDS TO BE CONTAINERIZED.**

You will need to have perl installed.

```
$ curl -LO http://xrl.us/cpanm
$ perl cpanm --installdeps .
$ perl stub.pl
```

The webserver listens on http://0.0.0.0:3000 by default.

## Endpoints

### GET /documents/:document

Retrieves the status for :document. Sample documents are con10101010, 
con20202020, con30303030, con40404040, and con50505050.

Statuses are:

- `fetching` - indicates that a document needs to be retrieved from the
source;
- `staged` - indicates that a document has been retrieved, and needs to be 
uploaded;
- `checked-in` - indicates that a document has been uploaded to Blob storage;
- `deleting` - indicates that a document needs to be deleted from Blob 
storage;
- `deleted` - indicates that a document has been deleted from Blob storage.

**Note: This endpoint is designed to be polled in order to get the
status of a request, so it mocks the progress of a request by automatically
progressing the status each time it is hit.**

A 200 will be returned on success, and a 404 if the requested document does
not exist.

### DELETE /documents/:document

Sends a delete request for :document. Sample documents are con10101010, 
con20202020, con30303030, con40404040, and con50505050.

A 202 will be returned on success, and a 404 if the requested document 
does not exist.

### POST /documents/

This expects an XML body shaped like the following:

```xml
<document>
  <id>con33333333</id>
  <name>Name of an SPC</name>
  <type>SPC</type>
  <author>theauthor</author>
  <product_name>Generic Statin</product_name>
  <keywords>
    <keyword>heart disease</keyword>
    <keyword>statin</keyword>
  </keywords>
  <pl_number>PL 12345/0010-0001</pl_number>
  <active_substances>
    <active_substance>statin</active_substance>
  </active_substances>
  <file_source>sentinel</file_source>
  <file_path>/docs/spc/con33333333.pdf</file_path>
</document>
```

It will return a 202 (and the document will have a status) if the shape
is correct, a 422 if there are missing required fields, and a 409 if the
ID already exists and is not in the deleted state.
