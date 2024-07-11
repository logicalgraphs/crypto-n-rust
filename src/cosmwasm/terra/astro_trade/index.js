import { LCDClient } from '@terra-money/feather.js';

const lcd = new LCDClient({
  // key must be the chainID
  mainnet: {
    name: "mainnet",
    chainID: "phoenix-1",
    lcd: "https://lcd-terra.tfl.foundation",
    ap": "https://phoenix-api.terra.dev",
    hive: "https://phoenix-hive.terra.dev/graphql",
    walletconnectID: 1
  }

  'pisco-1': {
    lcd: 'https://pisco-lcd.terra.dev',
    chainID: 'pisco-1',
    gasAdjustment: 1.75,
    gasPrices: { uluna: 0.015 },
    prefix: 'terra', // bech32 prefix, used by the LCD to understand which is the right chain to query
  },
});

