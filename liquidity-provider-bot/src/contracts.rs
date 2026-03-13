use crate::gas::GasStation;
use crate::metrics::CHAIN_REQUESTS_COUNTER;
use ethers::prelude::*;
use ethers::types::Bytes;

const DECIMALS: u64 = 1_000_000;

#[derive(Clone)]
pub struct Contracts {
    provider: Provider<Http>,
    gas_station: GasStation,
    address: Address,
}

impl Contracts {
    pub fn new(provider: Provider<Http>, gas_station: GasStation, address: Address) -> Self {
        Self {
            provider,
            gas_station,
            address,
        }
    }

    pub async fn token_balance_of(
        &self,
        token: Address,
        address: Address,
        token_id: Option<u64>,
    ) -> f64 {
        if token_id.is_none() {
            self.balance_of_erc20(token, address).await
        } else {
            self.balance_of_erc1155(token, address, token_id.unwrap()).await
        }
    }

    async fn balance_of_erc20(&self, token: Address, address: Address) -> f64 {
        // ERC20 balanceOf(address) function selector: 0x70a08231
        let mut data = vec![0x70, 0xa0, 0x82, 0x31];
        let address_bytes = address.as_fixed_bytes();
        data.extend_from_slice(&[0u8; 12]); // Padding to 32 bytes
        data.extend_from_slice(address_bytes);

        let call_request = TransactionRequest::new()
            .to(token)
            .data(Bytes::from(data));

        match self.provider.call(&call_request, None).await {
            Ok(result) => {
                CHAIN_REQUESTS_COUNTER.inc();
                if let Some(bytes) = result {
                    if bytes.len() >= 32 {
                        let balance = U256::from_big_endian(&bytes[..32]);
                        // Assume 6 decimals for USDC (most common collateral)
                        balance.as_u128() as f64 / 1_000_000.0
                    } else {
                        log::warn!("Invalid balance response length");
                        0.0
                    }
                } else {
                    0.0
                }
            }
            Err(e) => {
                CHAIN_REQUESTS_COUNTER.inc();
                log::error!("Error fetching ERC20 balance: {}", e);
                0.0
            }
        }
    }

    async fn balance_of_erc1155(
        &self,
        token: Address,
        address: Address,
        token_id: u64,
    ) -> f64 {
        // ERC1155 balanceOf(address,uint256) function selector: 0x00fdd58e
        let mut data = vec![0x00, 0xfd, 0xd5, 0x8e];
        let address_bytes = address.as_fixed_bytes();
        data.extend_from_slice(&[0u8; 12]); // Padding to 32 bytes
        data.extend_from_slice(address_bytes);
        // Token ID as u256 (32 bytes, big-endian)
        let mut token_id_bytes = [0u8; 32];
        let token_id_be = token_id.to_be_bytes();
        token_id_bytes[24..].copy_from_slice(&token_id_be);
        data.extend_from_slice(&token_id_bytes);

        let call_request = TransactionRequest::new()
            .to(token)
            .data(Bytes::from(data));

        match self.provider.call(&call_request, None).await {
            Ok(result) => {
                CHAIN_REQUESTS_COUNTER.inc();
                if let Some(bytes) = result {
                    if bytes.len() >= 32 {
                        let balance = U256::from_big_endian(&bytes[..32]);
                        // ERC1155 tokens typically have 18 decimals or match the collateral
                        balance.as_u128() as f64 / 1e18
                    } else {
                        log::warn!("Invalid balance response length");
                        0.0
                    }
                } else {
                    0.0
                }
            }
            Err(e) => {
                CHAIN_REQUESTS_COUNTER.inc();
                log::error!("Error fetching ERC1155 balance: {}", e);
                0.0
            }
        }
    }

    pub async fn gas_balance(&self, address: Address) -> f64 {
        match self.provider.get_balance(address, None).await {
            Ok(balance) => {
                CHAIN_REQUESTS_COUNTER.inc();
                balance.as_u128() as f64 / 1e18
            }
            Err(e) => {
                CHAIN_REQUESTS_COUNTER.inc();
                log::error!("Error get_balance: {}", e);
                0.0
            }
        }
    }

    pub async fn max_approve_erc20(
        &self,
        _token: Address,
        _owner: Address,
        _spender: Address,
    ) -> Option<H256> {
        // ERC20 approve implementation would go here
        None
    }

    pub async fn max_approve_erc1155(
        &self,
        _token: Address,
        _owner: Address,
        _spender: Address,
    ) -> Option<H256> {
        // ERC1155 setApprovalForAll implementation would go here
        None
    }
}

