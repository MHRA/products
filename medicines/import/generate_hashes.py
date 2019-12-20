import json
from pathlib import Path
import hashlib

p = Path('.')

def get_sha(file):
    file_contents = file.read_bytes()
    return hashlib.sha1(file_contents).hexdigest()

sha_dict = {}

for pdf in p.glob('**/*.pdf'):
    sha = get_sha(pdf)
    if pdf.stem not in sha_dict:
        sha_dict[pdf.stem] = []
    sha_dict[pdf.stem].append(sha)

print(json.dumps(sha_dict, indent=2))