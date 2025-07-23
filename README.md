# gerard
Gerard has been re-imagined to be a compiled rust application (previously used the node runtime). As a result, running gerard is just as easy as before, but faster! Instead of downloading npm, just download rust and cargo, and run `cargo build --release`. There are no submodules due to cargo being an excellent package manager! Just make sure you set up the following:
- A service account through Google Cloud, with its credentials stored in `service_account.json` in the root project directory. This will look something like:
```JSON
{
  "type": "placeholder",
  "project_id": "placeholder",
  "private_key_id": "placeholder",
  "private_key": "placeholder",
  "client_email": "placeholder",
  "client_id": "placeholder",
  "auth_uri": "placeholder",
  "token_uri": "placeholder",
  "auth_provider_x509_cert_url": "placeholder",
  "client_x509_cert_url": "placeholder",
  "universe_domain": "placeholder"
}
```
- A fully defined `.env` file in the root directory with the following environment variables:
```env
DISCORD_TOKEN="placeholder"
ROT_CHANNEL_ID="placeholder"
REAL_AMOGUS_ID="placeholder"
GOOGLE_CREDS="placeholder"
GOOGLE_KEY="placeholder"
DICT_KEY="placeholder"
THESAURUS_KEY="placeholder"
WOLFRAM_SIMPLE="placeholder"
```

It is important that these two files maintain the quotes as shown above.