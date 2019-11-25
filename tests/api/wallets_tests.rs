use crate::utils::asserts::meta::assert_meta;
use crate::utils::asserts::transaction::{test_lock_array, test_transaction_array};
use crate::utils::asserts::wallet::{assert_wallet_data, test_wallet_array};
use crate::utils::mockito_helpers::{mock_client, mock_http_request, mock_post_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;
use std::collections::HashMap;

#[test]
fn test_wallets_all() {
    let (_mock, body) = mock_http_request("wallets");
    {
        let mut client = mock_client();
        let response = client.wallets.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_wallet_array(response.data, expected);
    }
}

#[test]
fn test_wallet_show() {
    let (_mock, body) = mock_http_request("wallets/dummy");
    {
        let mut client = mock_client();
        let response = client.wallets.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_wallet_data(response.data, &expected["data"]);
    }
}

#[test]
fn test_wallet_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions");
    {
        let mut client = mock_client();
        let response = client.wallets.transactions("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(response.data, expected);
    }
}

#[test]
fn test_wallet_sent_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/sent");
    {
        let mut client = mock_client();
        let response = client.wallets.sent_transactions("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(response.data, expected);
    }
}

#[test]
fn test_wallet_received_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/received");
    {
        let mut client = mock_client();
        let response = client.wallets.received_transactions("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(response.data, expected);
    }
}

#[test]
fn test_votes() {
    let (_mock, body) = mock_http_request("wallets/dummy/votes");
    {
        let mut client = mock_client();
        let response = client.wallets.votes("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(response.data, expected);
    }
}

#[test]
fn test_wallet_search() {
    let (_mock, body) = mock_post_request("wallets/search");
    {
        let mut client = mock_client();
        let mut query = HashMap::new();
        query.insert("address", "D77tg5cPsDScdATRHRyWJ7CaeJJpN6XgZT");

        let response = client
            .wallets
            .search(query, Vec::<(String, String)>::new())
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_wallet_array(response.data, expected);
    }
}

#[test]
fn test_wallet_top() {
    let (_mock, body) = mock_http_request("wallets/top");
    {
        let mut client = mock_client();
        let response = client.wallets.top().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_wallet_array(response.data, expected);
    }
}

#[test]
fn test_wallet_locks() {
    let (_mock, body) = mock_http_request("wallets/dummy/locks");
    {
        let mut client = mock_client();
        let response = client.wallets.locks("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_lock_array(response.data, expected);
    }
}
