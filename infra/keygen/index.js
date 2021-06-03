import {Keyring} from "@polkadot/keyring";
import {cryptoWaitReady, mnemonicGenerate} from "@polkadot/util-crypto";
import fs from "fs";
// const fs = require('fs');

const keyring_aura = new Keyring({type: 'sr25519'});
const keyring_gran = new Keyring({type: 'ed25519'});
await cryptoWaitReady();
for (let i = 0; i < 100; i++) {
    let json_file = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "author_insertKey",
        "params": [
            "<aura/gran>",
            "<mnemonic phrase>",
            "<public key>"
        ]
    };

    let mnemonic = mnemonicGenerate();
    let aura_pair = keyring_aura.addFromMnemonic(mnemonic, {type: 'sr25519'});
    json_file.params = ["aura", mnemonic, "0x" + Buffer.from(aura_pair.publicKey).toString('hex')];
    fs.writeFile('../templates/polkadex-node-' + i + '-aura.json', JSON.stringify(json_file), {flag: 'w+'}, err => {
        if (err != null) {
            console.log(err);
        }
    });
    let gran_pair = keyring_aura.addFromMnemonic(mnemonic, {type: 'ed25519'});
    json_file.params = ["gran", mnemonic, "0x" + Buffer.from(gran_pair.publicKey).toString('hex')];
    fs.writeFile('../templates/polkadex-node-' + i + '-gran.json', JSON.stringify(json_file), {flag: 'w+'}, err => {
        if (err != null) {
            console.log(err);
        }
    });
    console.log("authority_keys_from_seed(\"" + mnemonic + "\"),")

}