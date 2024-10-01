#[cfg(test)]
mod integration_tests {

    use alloy::network::eip2718::Encodable2718;
    use alloy::providers::WalletProvider;
    use alloy::{
        eips::{BlockId, BlockNumberOrTag},
        network::TransactionBuilder,
        primitives::{Bytes, U256},
        providers::Provider,
        rpc::types::{BlockTransactionsKind, Filter, TransactionInput, TransactionRequest},
        sol_types::{SolEvent, SolValue},
    };
    use core::time;
    use eyre::{Result, WrapErr};
    use futures::stream::{self, TryStreamExt};
    use futures_util::StreamExt;
    use log::info;
    use std::error::Error;
    use std::time::{Instant, UNIX_EPOCH};
    use std::{collections::HashMap, collections::HashSet, sync::Arc};
    use tokio::time::Duration;
    use tokio::{sync::Mutex, time::sleep};

    use bitcoind::bitcoincore_rpc::{RawTx, RpcApi};
    use futures::future::{try_join_all, TryFutureExt};
    use std::str::FromStr;

    use alloy::{primitives::Address, providers::ext::AnvilApi};
    use bitcoin::{hashes::Hash, hex::DisplayHex};
    use eyre::eyre;
    use log::debug;
    use rift_core::{btc_light_client::AsLittleEndianBytes, lp::LiquidityReservation};
    use test_utils::core::{get_new_core_aware_address, RiftDevnet};
    use tokio;

    async fn setup() -> Result<RiftDevnet> {
        let _ = env_logger::init();
        let mock_proofs = true;
        RiftDevnet::setup(mock_proofs).await
    }

    async fn teardown(devnet: RiftDevnet) {
        drop(devnet);
    }

    #[tokio::test]
    async fn test_basic_swap() -> Result<()> {
        let devnet = setup().await?;
        devnet.spawn_hypernode().await?;
        let signer = devnet
            .rift_exchange_contract
            .provider()
            .wallet()
            .default_signer();

        // mint 100 usdt
        let deposit_amount = U256::from(100_000_000);
        let depositor_btc_address =
            get_new_core_aware_address(&devnet.bitcoin_regtest_instance, bitcoin::Network::Regtest);
        let depositer_btc_locking_script = depositor_btc_address.script_pubkey().into_bytes();

        let exchange_rate = 652173900000000_u64;
        info!("Minting 100 USDT to depositer");
        info!("Deposit btc address: {}", depositor_btc_address);
        info!(
            "Deposit btc locking script: {}",
            depositer_btc_locking_script.as_hex()
        );

        devnet
            .usdt_contract
            .mint(signer.address(), deposit_amount)
            .send()
            .await?
            .watch()
            .await?;

        devnet
            .usdt_contract
            .approve(*devnet.rift_exchange_contract.address(), deposit_amount)
            .send()
            .await?
            .watch()
            .await?;

        // output all the parameters of depositLiquidity
        debug!(
            "deposit_amount: {}",
            deposit_amount.to_string().as_str().parse::<f64>().unwrap()
        );
        debug!("exchange_rate: {}", exchange_rate);
        debug!(
            "depositor_btc_locking_script: {}",
            depositer_btc_locking_script.as_hex()
        );

        // deposit 100 usdt
        devnet
            .rift_exchange_contract
            .depositLiquidity(
                deposit_amount,
                exchange_rate,
                alloy::primitives::FixedBytes::<22>(
                    depositer_btc_locking_script.as_slice().try_into().unwrap(),
                ),
            )
            .send()
            .await?
            .watch()
            .await?;

        // reserve it
        let vault_indexes_to_reserve = [U256::from(0)].to_vec();
        let amounts_to_reserve = [deposit_amount].to_vec();
        let eth_payout_address =
            Address::from_str("0x6778FE3006aE09A933AaCE119e51466d9B4925EC").unwrap();
        let total_sats_input_including_proxy_fee = U256::from(0);
        let expired_swap_reservation_indexes: Vec<U256> = Vec::new();

        let current_unix_timestamp = chrono::Utc::now().timestamp() as u64;
        devnet
            .rift_exchange_contract
            .provider()
            .anvil_set_next_block_timestamp(current_unix_timestamp)
            .await?;

        devnet
            .rift_exchange_contract
            .reserveLiquidity(
                eth_payout_address,
                vault_indexes_to_reserve,
                amounts_to_reserve,
                eth_payout_address,
                total_sats_input_including_proxy_fee,
                expired_swap_reservation_indexes,
            )
            .send()
            .await?
            .watch()
            .await?;

        let swap_reservation = devnet
            .rift_exchange_contract
            .getReservation(U256::from(0))
            .call()
            .await?
            ._0;

        let liquidity_reservations: Vec<LiquidityReservation> = try_join_all(
            swap_reservation
                .expectedSatsOutput
                .into_iter()
                .zip(swap_reservation.vaultIndexes.into_iter())
                .map(|(expected_sats, vault_index)| {
                    let rift_exchange_contract = devnet.rift_exchange_contract.clone();
                    async move {
                        let vault = rift_exchange_contract
                            .getDepositVault(vault_index)
                            .call()
                            .await
                            .map_err(|e| eyre!("Failed to get deposit vault: {}", e))?
                            ._0;
                        Ok(LiquidityReservation {
                            script_pub_key: vault.btcPayoutLockingScript.0,
                            expected_sats,
                        })
                    }
                    .map_err(|e: eyre::Report| e)
                }),
        )
        .await?;

        let total_sats_output = liquidity_reservations
            .iter()
            .map(|reservation| reservation.expected_sats)
            .sum::<u64>();
        info!("Total sats output: {}", total_sats_output);

        let allocated_btc_fees = 2000;
        // Now simulate the proxy wallet getting send funds from a random wallet add 2k sats for
        // fees
        let in_tx = devnet.create_btc_utxo(total_sats_output + (allocated_btc_fees * 4))?;

        info!(
            "Proxy wallet locking script: {}",
            devnet
                .funded_btc_wallet
                .get_p2wpkh_script()
                .into_bytes()
                .as_hex()
        );
        info!(
            "Proxy wallet public key: {}",
            devnet.funded_btc_wallet.public_key
        );

        let tx_vout = in_tx
            .output
            .iter()
            .position(|output| {
                output.script_pubkey.clone().into_bytes()
                    == devnet.funded_btc_wallet.get_p2wpkh_script().into_bytes()
            })
            .unwrap();

        // now that we've reserved, send the lp bitcoin
        let rift_bitcoin_transaction = rift_lib::transaction::build_rift_payment_transaction(
            swap_reservation.nonce.into(),
            &liquidity_reservations,
            in_tx
                .compute_txid()
                .as_raw_hash()
                .to_byte_array()
                .to_little_endian(),
            &in_tx,
            tx_vout as u32,
            &devnet.funded_btc_wallet,
            allocated_btc_fees,
        );

        devnet.rift_exchange_contract.provider().anvil_set_interval_mining(5).await?;

        devnet
            .bitcoin_regtest_instance
            .client
            .send_raw_transaction(rift_bitcoin_transaction.raw_hex())?;
        info!("Sent rift bitcoin payment transaction");

        // now mine the tx
        devnet
            .bitcoin_regtest_instance
            .client
            .generate_to_address(2, &devnet.miner)?;


        // Now setup an event listener that waits for ProofSubmitted event => mine a block 10
        // minutes into the future
        let swap_complete_filter = devnet
            .rift_exchange_contract
            .SwapComplete_filter()
            .from_block(0)
            .watch()
            .await?;
        let proof_proposed_filter = devnet
            .rift_exchange_contract
            .ProofSubmitted_filter()
            .from_block(0)
            .watch()
            .await?;

        // Convert the filters into streams.
        let mut swap_complete_stream = swap_complete_filter.into_stream();
        let mut proof_proposed_stream = proof_proposed_filter.into_stream();

        // Use a HashSet to keep track of already-processed logs
        let processed_logs = Arc::new(Mutex::new(HashSet::new()));

        loop {
            let contract = Arc::clone(&devnet.rift_exchange_contract);
            tokio::select! {
                Some(log) = swap_complete_stream.next() => {
                    let log_data = log.clone()?;
                    let log_identifier = (log_data.1.block_number, log_data.1.transaction_index, log_data.1.log_index);

                    let mut processed_logs_guard = processed_logs.lock().await;
                    if !processed_logs_guard.contains(&log_identifier) {
                        processed_logs_guard.insert(log_identifier);
                        drop(processed_logs_guard);

                        let swap_reservation_index = log_data.0.swapReservationIndex;
                        info!("SwapComplete with reservation index: {:?}", &swap_reservation_index);
                    }
                }

                Some(log) = proof_proposed_stream.next() => {
                    let log_data = log.clone()?;
                    let log_identifier = (log_data.1.block_number, log_data.1.transaction_index, log_data.1.log_index);

                    let mut processed_logs_guard = processed_logs.lock().await;
                    if !processed_logs_guard.contains(&log_identifier) {
                        processed_logs_guard.insert(log_identifier);
                        drop(processed_logs_guard);
                        info!("ProofSubmitted w/ reservation index: {:?}", &log_data.0.swapReservationIndex);
                        let swap_reservation_index = log_data.0.swapReservationIndex;
                        break;
                    }
                }
            };
        }

        teardown(devnet).await;
        Ok(())
    }
}
