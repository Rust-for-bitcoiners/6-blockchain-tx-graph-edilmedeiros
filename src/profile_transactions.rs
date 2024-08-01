use std::env;
use bitcoincore_rpc::{Auth, Client};
use bitcoin::hash_types::Txid;

use super::graph::Graph;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

fn build_transaction_graph(start_height: u64, end_height: u64) -> Graph<Txid> {
    // Every Transaction has a set of Inputs and outputs
    // Each Input refers to an output of some earlier transaction
    // We say a Transaction A funds Transaction B if an ouput of A is an input of B
    // Build a graph where nodes represents Txid and an edge (t1, t2) is in the graph
    // if the transaction t1 funds transaction t2
    Graph::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::Amount;

    #[test]
    fn test_bitcoind() {
        use bitcoincore_rpc::RpcApi;
        use bitcoincore_rpc::*;


        // use bitcoind from the system's path
        if let Ok(exe_path) = bitcoind::exe_path() {
            let bitcoind = bitcoind::BitcoinD::new(exe_path).unwrap();

            // create an address to receive new funds
            let coinbase_address = bitcoind.client.get_new_address(None, None).unwrap()
                .require_network(bitcoin::Network::Regtest).unwrap();

            // generate 101 blocks, otherwise can't spend newly created funds
            bitcoind.client.generate_to_address(101, &coinbase_address);
            assert_eq!(101, bitcoind.client.get_blockchain_info().unwrap().blocks);

            // generate a receiving addresses
            let address = bitcoind.client.get_new_address(None, Some(json::AddressType::Legacy)).unwrap()
                .require_network(bitcoin::Network::Regtest).unwrap();

            // send 10 btc
            bitcoind.client.send_to_address(
                &address,
                Amount::ONE_BTC * 10,
                None,
                None,
                None,
                None,
                None,
                None
            );

            // Mine a block
            bitcoind.client.generate_to_address(1, &coinbase_address);
            println!("{:?}", bitcoind.client.get_balances());
            println!("{:?}", bitcoind.client.get_blockchain_info());
            println!("{:?}", bitcoind.client.get_received_by_address(&address, None));
            println!("{:?}", address);
            println!("{:?}", address.address_type());
            println!("{:?}", address.pubkey_hash());

            let block_hash = bitcoind.client.get_block_hash(102).unwrap();
            let block = bitcoind.client.get_block(&block_hash).unwrap();
            println!("{:?}", block);
        }

    }
}
