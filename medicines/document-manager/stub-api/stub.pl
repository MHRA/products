use Dancer2;
use XML::Simple qw(:strict);
use UUID::Tiny ':std';
set serializer => 'XML';

my %documents;
my %jobs_statuses;

get '/jobs/:job' => sub {
    my $job_id = route_parameters->get('job');
    if (exists($jobs_statuses{$job_id})) {
        my $self = shift;
        $self->{serializer_engine}->{xml_options}->{serialize} =
            {RootName => 'document', NoAttr => 1};

        my $status = $jobs_statuses{$job_id};

        if ($status->{status} eq 'accepted') {
            $jobs_statuses{$job_id}->{status} = 'done';
        }

        status 'ok';
        return {id => $job_id, status => $status->{status}};
    } else {
        status 'not_found';
        return '';
    }
};

del '/documents/:document' => sub {
    my $document_id = route_parameters->get('document');

    if (exists($documents{$document_id})) {
        delete($documents{$document_id});

        my $job_id = create_uuid_as_string(UUID_V4);
        $jobs_statuses{$job_id} = {document_id => $document_id, status => 'accepted'};

        status 'accepted';
        return '';
    } else {
        status 'not_found';
        return '';
    }
};

post '/documents' => sub {
    my $self = shift;
    $self->{serializer_engine}->{xml_options}->{serialize} =
        {RootName => 'document', NoAttr => 1};

    my $document = XMLin(
        request->body,
        KeyAttr    => [],
        ForceArray => ['keyword', 'products', 'active_substance'],
        GroupTags  => {
            keywords          => 'keyword',
            active_substances => 'active_substance',
            'products'        => 'product'
        }
    );

    my @expected_fields = ('id', 'name', 'type', 'author', 'products', 'pl_number', 'active_substances', 'file_source', 'file_path');

    for my $expected_field (@expected_fields) {
        if (!defined($document->{$expected_field})) {
            status 'unprocessable_entity';
            return {error => "Expected '$expected_field' in request."};
        }
    }

    if (defined($documents{$document->{id}})) {
        status 'conflict';
        return {error => "Document $document->{id} already exists."};
    }

    my $job_id = create_uuid_as_string(UUID_V4);
    $jobs_statuses{$job_id} = {document_id => $document->{id}, status => 'accepted'};
    $documents{$document->{id}} = $document;

    status 'accepted';
    return {job_id => $job_id, document_id => $document->{id}};
};

dance;
