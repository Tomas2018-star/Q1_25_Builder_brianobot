import wallet from "../wba-wallet.json"
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

        const image = "https://devnet.irys.xyz/FwmSbw8qk2pNzEzYt62mgjKBuLAt8cAVcACHSnoUqUtQ"
        const metadata = {
            name: "Brian Obot",
            symbol: "BATMAN",
            description: "Legendary BATMAN NFT From Brian Obot",
            image: image,
            attributes: [
                {trait_type: 'Collection', value: 'Genesis'},
                {trait_type: 'Style', value: 'Modern'},
                {trait_type: 'Color', value: 'Brown'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        // upload metadata to the blockchain
        const metadataUri = await umi.uploader.uploadJson(metadata);
        const cleanedUrl = metadataUri.replace("https://arweave.net", "https://devnet.irys.xyz");
        console.log("Your metadata URI: ", cleanedUrl);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
