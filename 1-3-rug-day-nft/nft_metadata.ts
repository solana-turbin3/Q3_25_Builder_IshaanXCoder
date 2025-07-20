import wallet from "./Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://gateway.irys.xyz/5rAUJo6467jEX9vdwJ8SRfyGcrQXb4xDD4j2iAaVwng2   "
        const metadata = {
            name: "RugMe",
            symbol: "RGM",
            description: "hehe this is a jeff rugme nft, yes ofc rug day means photo of jeff + some ai + some cool stuff heh",
            image: "https://gateway.irys.xyz/BdLuytrBuKfcFNufwNvETVEPhoUJ3poxWpkAmgHkksfF",
            attributes: [
                {trait_type: '?', value: '?'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://gateway.irys.xyz/BdLuytrBuKfcFNufwNvETVEPhoUJ3poxWpkAmgHkksfF"
                    },
                ]
            },
            creators: []
        };
        const metadataBuffer = Buffer.from(JSON.stringify(metadata));
        const genericMetadata = createGenericFile(metadataBuffer, "metadata.json", {
            contentType: "application/json",
        });
        const [myUri] = await umi.uploader.upload([genericMetadata]);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
