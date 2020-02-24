use Dancer2;
use XML::Simple qw(:strict);
use UUID::Tiny ':std';
use Storable 'dclone';
set serializer => 'XML';

my %documents;
my %jobs;

get '/jobs/:job' => sub {
    my $job_id = route_parameters->get('job');
    if (exists($jobs{$job_id})) {
        my $self = shift;
        $self->{serializer_engine}->{xml_options}->{serialize} = {RootName => 'job', NoAttr => 1};

        my $status = $jobs{$job_id};
        my $resp = dclone $status;

        if ($status->{status} eq 'accepted') {
            $jobs{$job_id}->{status} = 'done';
        }

        status 'ok';
        return $resp;
    } else {
        status 'not_found';
        return '';
    }
};

del '/documents/:document' => sub {
    my $document_id = route_parameters->get('document');

    if (exists($documents{$document_id})) {
        my $self = shift;
        $self->{serializer_engine}->{xml_options}->{serialize} = {RootName => 'job', NoAttr => 1};

        delete($documents{$document_id});

        my $job_id = create_uuid_as_string(UUID_V4);
        my $job = {
            job_id      => $job_id,
            job_uri     => uri_for("/jobs/$job_id"),
            document_id => $document_id,
            status      => 'accepted',
            type        => 'delete'
        };
        $jobs{$job_id} = $job;

        status 'accepted';
        return $jobs{$job_id};
    } else {
        status 'not_found';
        return '';
    }
};

post '/documents' => sub {
    my $self = shift;
    $self->{serializer_engine}->{xml_options}->{serialize} = {RootName => 'job', NoAttr => 1};

    my $doc = XMLin(
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
        if (!defined($doc->{$expected_field})) {
            status 'unprocessable_entity';
            return {error => "Expected '$expected_field' in request."};
        }
    }

    if (defined($documents{$doc->{id}})) {
        status 'conflict';
        return {error => "Document $doc->{id} already exists."};
    }

    my $job_id = create_uuid_as_string(UUID_V4);
    my $job = {
        job_id      => $job_id,
        job_uri     => uri_for("/jobs/$job_id"),
        document_id => $doc->{id},
        type        => 'check-in',
        status      => 'accepted'
    };
    $jobs{$job_id} = $job;
    $documents{$doc->{id}} = $doc;

    status 'accepted';
    return $job;
};

dance;
