import React, { FC, useMemo } from 'react';
import {
    WalletModalProvider,
    WalletDisconnectButton,
    WalletMultiButton
} from '@solana/wallet-adapter-react-ui';

export const Wallet = () => {
    return (
        <WalletMultiButton />
    );
};