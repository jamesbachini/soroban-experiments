const StellarSdk = require("@stellar/stellar-sdk");
require("dotenv").config();

/*
const newKeypair = StellarSdk.Keypair.random();
const newPublicKey = newKeypair.publicKey();
const newSecretKey = newKeypair.secret();
console.log("New Public Key:", newPublicKey);
console.log("New Secret Key:", newSecretKey);
const restoredKeypair = StellarSdk.Keypair.fromSecret(newSecretKey);
const restoredPublicKey = restoredKeypair.publicKey();
console.log("Restored Public Key:", restoredPublicKey);
console.log("Restored Secret Key:", newSecretKey);
*/

const keypair = StellarSdk.Keypair.fromSecret(process.env.SECRET);
console.log("Public Key:", keypair.publicKey());
