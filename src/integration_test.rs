#![cfg(test)]

use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{
    coins, from_binary, to_binary, Addr, BalanceResponse, BankQuery, Coin, Empty, Uint128,
};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg};
use cw_multi_test::{App, Contract, ContractWrapper, SimpleBank};

use crate::msg::{ExecuteMsg, InstantiateMsg, ReceiveMsg};

fn mock_app() -> App {
    let env = mock_env();
    let api = Box::new(MockApi::default());
    let bank = SimpleBank {};

    App::new(api, env.block, bank, || Box::new(MockStorage::new()))
}

pub fn contract_sale() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

#[test]
// receive cw20 tokens and release upon approval
fn sale_happy_path() {
    let mut router = mock_app();

    const NATIVE_TOKEN_DENOM: &str = "token";

    let owner = Addr::unchecked("owner");

    // set up cw20 contract with some tokens
    let cw20_id = router.store_code(contract_cw20());
    let msg = cw20_base::msg::InstantiateMsg {
        name: "Cash Money".to_string(),
        symbol: "CASH".to_string(),
        decimals: 2,
        initial_balances: vec![Cw20Coin {
            address: owner.to_string(),
            amount: Uint128(5000),
        }],
        mint: None,
    };
    let cash_addr = router
        .instantiate_contract(cw20_id, owner.clone(), &msg, &[], "CASH")
        .unwrap();

    // set up sale contract
    let sale_id = router.store_code(contract_sale());
    let price = Uint128::from(1u128);
    let msg = InstantiateMsg {
        cw20_address: cash_addr.clone(),
        price,
        denom: NATIVE_TOKEN_DENOM.to_string(),
    };
    let sale_addr = router
        .instantiate_contract(sale_id, owner.clone(), &msg, &[], "Sale")
        .unwrap();

    assert_ne!(cash_addr, sale_addr);

    // set up cw20 helpers
    let cash = Cw20Contract(cash_addr.clone());

    // check initial balances
    let owner_balance = cash.balance(&router, owner.clone()).unwrap();
    assert_eq!(owner_balance, Uint128(5000));
    let sale_balance = cash.balance(&router, sale_addr.clone()).unwrap();
    assert_eq!(sale_balance, Uint128(0));

    // send tokens to contract address
    let send_msg = Cw20ExecuteMsg::Send {
        contract: sale_addr.to_string(),
        amount: Uint128::from(100u128),
        msg: Some(to_binary(&ReceiveMsg::Receive {}).unwrap()),
    };
    let res = router
        .execute_contract(owner.clone(), cash_addr.clone(), &send_msg, &[])
        .unwrap();
    println!("{:?}", res.attributes);
    assert_eq!(4, res.attributes.len());

    // ensure balances updated
    let owner_balance = cash.balance(&router, owner.clone()).unwrap();
    assert_eq!(owner_balance, Uint128(4900));
    let sale_balance = cash.balance(&router, sale_addr.clone()).unwrap();
    assert_eq!(sale_balance, Uint128(100));

    // Create buyer address with some native tokens
    let buyer = Addr::unchecked("buyer");
    let funds = coins(2000, NATIVE_TOKEN_DENOM);
    router.set_bank_balance(&buyer, funds).unwrap();

    // Check cw20 token balance is zero
    let buyer_balance = cash.balance(&router, buyer.clone()).unwrap();
    assert_eq!(buyer_balance, Uint128(0));

    // Buy cw20tokens through sale contract
    let buy_msg = ExecuteMsg::Buy {
        denom: NATIVE_TOKEN_DENOM.to_string(),
        price,
    };
    let res = router
        .execute_contract(
            buyer.clone(),
            sale_addr.clone(),
            &buy_msg,
            &[Coin {
                amount: Uint128(10u128),
                denom: NATIVE_TOKEN_DENOM.to_string(),
            }],
        )
        .unwrap();
    println!("{:?}", res.attributes);
    assert_eq!(5, res.attributes.len());

    let buyer_balance = cash.balance(&router, buyer.clone()).unwrap();
    assert_eq!(buyer_balance, Uint128(10));

    // Check balances of owner and buyer reflect the sale transaction
    let query_res = router
        .query(
            cosmwasm_std::QueryRequest::Bank(BankQuery::Balance {
                address: buyer.to_string(),
                denom: NATIVE_TOKEN_DENOM.to_string(),
            })
            .into(),
        )
        .unwrap();
    let balance: BalanceResponse = from_binary(&query_res).unwrap();
    assert_eq!(balance.amount.amount, Uint128(1990));

    let query_res = router
        .query(
            cosmwasm_std::QueryRequest::Bank(BankQuery::Balance {
                address: owner.to_string(),
                denom: NATIVE_TOKEN_DENOM.to_string(),
            })
            .into(),
        )
        .unwrap();
    let balance: BalanceResponse = from_binary(&query_res).unwrap();
    assert_eq!(balance.amount.amount, Uint128(10));

    // Check cw20 token balance is 10
    let buyer_balance = cash.balance(&router, buyer.clone()).unwrap();
    assert_eq!(buyer_balance, Uint128(10));

    let withdraw_msg = ExecuteMsg::WithdrawAll {};
    let res = router
        .execute_contract(owner.clone(), sale_addr.clone(), &withdraw_msg, &[])
        .unwrap();
    println!("{:?}", res.attributes);
    assert_eq!(5, res.attributes.len());

    // check cash has been returned to owner
    let owner_balance = cash.balance(&router, owner.clone()).unwrap();
    assert_eq!(owner_balance, Uint128(4990))
}
