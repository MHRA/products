FROM perl:5.26

RUN cpanm Carton && mkdir /usr/src/stub
WORKDIR /usr/src/stub

ONBUILD COPY cpanfile /usr/src/stub
ONBUILD RUN carton install

ONBUILD COPY . /usr/src/stub

CMD ["perl", "./stub.pl"]