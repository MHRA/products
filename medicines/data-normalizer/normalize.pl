use v5.22;
use warnings;

my ($type, $input, $output) = @ARGV;
my @valid_types = qw(SPCPIL PAR);

if (!defined $type || !defined $input || !defined $output) {
    say @ARGV;
    die "Please use the following syntax: $0 TYPE INPUT OUTPUT.\n" .
        "Type should be one of the following: " . join(', ', @valid_types) . ".\n" .
        "INPUT should be the location of the input TSV (tab-separated value) file.\n" .
        "OUTPUT should be the location to write the CSV to.\n";
}

die "Unsupported type: $type" unless grep { $type eq $_ } @valid_types;

open my $ifh, '+<', $input or die "Input file, $input, could not be read.";
open my $ofh, '>', $output or die "Output file, $output, already exists and/or is not writeable.";

my $tsv = do {
    local $/;
    <$ifh>
};

close $ifh;

if ($type eq 'SPCPIL') {
    my $header_row =
        "dDocName,dDocType,dDocTitle,dSecurityGroup,dRevLabel,dCreateDate,dReleaseState,xProductName,xSubstanceName,xSecondLevel\n";

    my $csv = $header_row . (
        $tsv
            =~ s/20(\d{2})-(\d{2})-(\d{2}) (\d{2}\:\d{2})\:\d{2}\.\d{3}/$2\/$3\/$1 $4/gr
            =~ s/\b(([^\t,]+\,)+[^\t,]+)\b/\"$1\"/gr
            =~ s/\,?\t/,/gr
    );

    print $ofh $csv;
    close $ofh;
} elsif ($type eq 'PAR') {
    say 'PAR normalization not implemented yet.';
}