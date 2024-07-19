import { LCDClient } from '@terra-money/feather.js';

const lcd = new LCDClient({
  // key must be the chainID
  'pisco-1': {
    lcd: 'https://pisco-lcd.terra.dev',
    chainID: 'pisco-1',
    gasAdjustment: 1.75,
    gasPrices: { uluna: 0.015 },
    prefix: 'terra', // bech32 prefix, used by the LCD to understand which is the right chain to query
  },
});

const mk = new MnemonicKey({ mnemonic: process.env.WARP_WALLET });
console.log("My key is ", mk);

// const wallet = lcd.wallet(mk);

// console.log("My wallet is ", wallet);
