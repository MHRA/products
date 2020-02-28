use Dancer2;
use Dancer2::Plugin::Auth::HTTP::Basic::DWIW;
use Storable 'dclone';
use UUID::Tiny ':std';
use XML::Simple qw(:strict);

set serializer => 'XML';

my %documents;
my %jobs;

die "You need to set STUB_USERNAME & STUB_PASSWORD in your environment"
    unless (defined $ENV{'STUB_USERNAME'} && defined $ENV{'STUB_PASSWORD'});

http_basic_auth_handler check_login => sub {
    my ($user, $pass) = @_;

    return $user eq $ENV{'STUB_USERNAME'} && $pass eq $ENV{'STUB_PASSWORD'};
};

get '/jobs/:job' => http_basic_auth required => sub {
    my $self = shift;
    $self->{'app'}->{'serializer_engine'}->{'xml_options'}->{'serialize'} = {RootName => 'job', NoAttr => 1};

    my $job_id = route_parameters->get('job');
    if (exists($jobs{$job_id})) {
        my $status = $jobs{$job_id};
        my $resp = dclone $status;

        if ($status->{status} eq 'Accepted') {
            $jobs{$job_id}->{status} = 'Done';
        }

        status 'ok';
        return $resp;
    } else {
        status 'not_found';
        return '';
    }
};

del '/documents/:document' => http_basic_auth required => sub {
    my $self = shift;
    $self->{'app'}->{'serializer_engine'}->{'xml_options'}->{'serialize'} = {RootName => 'job', NoAttr => 1};

    my $document_id = route_parameters->get('document');

    my $job_id = create_uuid_as_string(UUID_V4);
    my $job = {
        id     => $job_id,
        status => 'Accepted'
    };

    unless (exists($documents{$document_id})) {
        $job->{status}        = 'Error';
        $job->{error_code}    = 0x0;
        $job->{error_message} = 'The document requested could not be deleted.';
    } else {
        delete($documents{$document_id});
    }

    $jobs{$job_id} = $job;

    status 'accepted';
    return $jobs{$job_id};
};

post '/documents' => http_basic_auth required => sub {
    my $self = shift;
    $self->{'app'}->{'serializer_engine'}->{'xml_options'}->{'serialize'} = {RootName => 'job', NoAttr => 1};

    my $doc = XMLin(
        request->body,
        KeyAttr    => [],
        ForceArray => ['keyword', 'products', 'active_substance'],
        GroupTags  => {
            keywords          => 'keyword',
            active_substances => 'active_substance',
            products          => 'product'
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
        id     => $job_id,
        status => 'Accepted'
    };
    $jobs{$job_id} = $job;
    $documents{$doc->{id}} = $doc;

    status 'accepted';
    return $job;
};

get '/healthz' => http_basic_auth required => sub {
    status 'ok';
    return {healthy => true};
};

dance;
