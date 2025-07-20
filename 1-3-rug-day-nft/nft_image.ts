// this code uploads an image from our sysynte to some decentraliedsystem like irys


// 1. Import the wallet

import wallet from "./Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));
umi.use(irysUploader({address: "https://devnet.irys.xyz/",}));


(async () => {
    try {
        //1. Load image
        const img = await readFile("./rug-day-nft.png");
        //2. Convert image to generic file.
        const generic = createGenericFile(img, "rug-day-nft.png", {
            contentType: "image/png",
        });
        //3. Upload image
        const [myUri] = await umi.uploader.upload([generic]);
        console.log("Your image URI: ", myUri);

            }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
        
    }
})();
