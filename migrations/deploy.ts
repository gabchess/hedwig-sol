import * as anchor from "@anchor-lang/core";

module.exports = async function (provider: anchor.AnchorProvider) {
  // Anchor injects the provider configured by the selected cluster and wallet.
  anchor.setProvider(provider);
};
