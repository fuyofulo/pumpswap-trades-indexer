use std::collections::HashMap;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterAccounts,
    SubscribeRequestFilterTransactions,
};

pub fn create_subscription_request() -> SubscribeRequest {
    let mut accounts = HashMap::new();

    accounts.insert(
        "pumpswap_accounts".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![
                "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string(),
            ],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let mut transactions = HashMap::new();
    transactions.insert(
        "pumpswap_transactions".to_string(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            failed: Some(false),
            signature: None,
            account_include: vec![
                "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string(),
            ],
            account_exclude: vec![],
            account_required: vec![],
        },
    );

    SubscribeRequest {
        accounts,
        slots: HashMap::new(),
        transactions,
        transactions_status: HashMap::new(),
        blocks: HashMap::new(),
        blocks_meta: HashMap::new(),
        entry: HashMap::new(),
        commitment: Some(CommitmentLevel::Confirmed as i32),
        accounts_data_slice: vec![],
        ping: None,
        from_slot: None,
    }
}
