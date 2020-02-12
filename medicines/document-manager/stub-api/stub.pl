use Dancer2;
use Data::Dumper;
use XML::Simple qw(:strict);
set serializer => 'XML';

my %documents_statuses;

get '/documents/:document' => sub {
    my $document = route_parameters->get('document');
    if (exists($documents_statuses{$document})) {
        my $self = shift;
        $self->{'serializer_engine'}->{'xml_options'}->{'serialize'} =
            {RootName => 'document', NoAttr => 1};

        my $status = $documents_statuses{$document};

        if ($status eq 'fetching') {
            $documents_statuses{$document} = 'staged';
        } elsif ($status eq 'deleting') {
            $documents_statuses{$document} = 'deleted';
        } elsif ($status eq 'staged') {
            $documents_statuses{$document} = 'checked-in';
        }

        status 'ok';
        return {id => $document, status => $status};
    } else {
        status 'not_found';
        return '';
    }
};

del '/documents/:document' => sub {
    if (exists($documents_statuses{route_parameters->get('document')})) {
        $documents_statuses{route_parameters->get('document')} = 'deleting';
        status 'accepted';
        return '';
    } else {
        status 'not_found';
        return '';
    }
};

post '/documents' => sub {
    my $self = shift;
    $self->{'serializer_engine'}->{'xml_options'}->{'serialize'} =
        {RootName => 'document', NoAttr => 1};

    my $document = XMLin(
        request->body,
        KeyAttr    => [],
        ForceArray => ['keyword', 'active_substance'],
        GroupTags  => {
            keywords          => 'keyword',
            active_substances => 'active_substance'
        }
    );

    my @expected_fields = ('id', 'name', 'type', 'author', 'product_name', 'pl_number', 'active_substances', 'file_source', 'file_path');

    for my $expected_field (@expected_fields) {
        if (!defined($document->{$expected_field})) {
            status 'unprocessable_entity';
            return {error => "Expected '$expected_field' in request."};
        }
    }

    if (defined($documents_statuses{$document->{id}}) && $documents_statuses{$document->{id}} ne 'deleted') {
        status 'conflict';
        return {error => "Document $document->{id} already exists."};
    }

    $documents_statuses{$document->{id}} = 'fetching';

    status 'accepted';
    return '';
};

sub reset_statuses {
    %documents_statuses = (
        con10101010 => 'checked-in',
        con20202020 => 'deleted',
        con30303030 => 'fetching',
        con40404040 => 'deleting',
        con50505050 => 'failed'
    );
};

put '/documents/reset' => sub {
    reset_statuses;
};

reset_statuses;
dance;
