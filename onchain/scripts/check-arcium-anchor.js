const anchor = require("@coral-xyz/anchor");
const { 
  getArciumProgAddress, 
  getArciumAccountBaseSeed, 
  getCompDefAccOffset,
  getMXEAccAddress
} = require("@arcium-hq/client");
const arciumAnchor = require("@arcium-hq/anchor");

console.log("Arcium Anchor functions:", Object.keys(arciumAnchor));