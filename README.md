# gerard
Gerard has been re-imagined to be a compiled rust application (previously used the node runtime). Before compiling ake sure you set up the following:
- Recursively pull submodules to ensure locate command with work properly
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
MODDED_PORT="placeholder"
UNMODDED_PORT="placeholder"
```
- Download [Zig 0.14.1](https://ziglang.org/download/) (Other versions may fail to compile) and build `mclocate` (libs\mclocate). For best results, run:
```
cd libs/mclocate
zig build --release=Fast
```
It is important that these two environment files maintain the quotes as shown above.

# Automation & QoL
This binary can technically run in any directory that contains the environment files as all assets are compiled into the binary. For best speed and size, you should build the project with `cargo build --release`. Most non-slash commands work based off of a lookup table system. If you would like to change any of the content in the lookup tables, you should look into `scripts/generate.py` which is used to create both the summon images and miscellaneous lookup tables.

As this is a discord bot, it is best to run in the background. Assuming you have `tmux` installed, you can run the `start_gerard.sh` file in the `scripts` directory. This directory also has scripts to see the process and kill it. You are free to change these files based on your needs.

# What was removed from the original?
- Gerard no longer supports most of the 'genshit' commands
- Gerard now only uses wolfram for math calls. The '!solve' command will no longer be supported
- You can no longer search youtube with Gerard. This was experimental and never worked well or was used
- Removed lcbackup command from gerard, this was never needed and can just be done by request instead
- 'john...' commands removed, not needed or used
- boom command removed, this was only a thing for learning the `discord.js` api in the past
- Removed the size command; we don't watch the associated streamer anymore