pub fn compute_sha_hash(bytes: &[u8]) -> String {
  let mut m = sha1::Sha1::new();
  m.update(bytes);
  m.digest().to_string()
}
