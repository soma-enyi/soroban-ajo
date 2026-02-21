#![cfg(test)]

use soroban_sdk::{testutils::{Address as _, Events}, symbol_short, Address, Env, IntoVal, Vec};
use soroban_ajo::{AjoContract, AjoContractClient};

fn setup_test_env() -> (Env, AjoContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let member2 = Address::generate(&env);
    let member3 = Address::generate(&env);
    
    (env, client, creator, member2, member3)
}

// In soroban-sdk v21.x, env.events().all() returns Vec<(Address, Vec<Val>, Val)>
// where tuple is (contract_id, topics, data)

#[test]
fn test_group_created_event() {
    let (env, client, creator, _, _) = setup_test_env();
    
    let contribution = 100_000_000i128;
    let cycle_duration = 604_800u64;
    let max_members = 10u32;
    
    let group_id = client.create_group(&creator, &contribution, &cycle_duration, &max_members);
    
    let events = env.events().all();
    let last = events.last().unwrap();
    // last is (contract_id, topics_vec, data_val)
    let (_contract_id, topics, data) = last;
    
    let expected_topics: Vec<soroban_sdk::Val> = (symbol_short!("created"), group_id).into_val(&env);
    assert_eq!(topics, expected_topics);
    
    let payload: (Address, i128, u32) = data.into_val(&env);
    assert_eq!(payload.0, creator);
    assert_eq!(payload.1, contribution);
    assert_eq!(payload.2, max_members);
}

#[test]
fn test_member_joined_event() {
    let (env, client, creator, member2, _) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &10u32);
    
    client.join_group(&member2, &group_id);
    
    let events = env.events().all();
    let (_contract_id, topics, data) = events.last().unwrap();
    
    let expected_topics: Vec<soroban_sdk::Val> = (symbol_short!("joined"), group_id).into_val(&env);
    assert_eq!(topics, expected_topics);
    
    let payload: Address = data.into_val(&env);
    assert_eq!(payload, member2);
}

#[test]
fn test_contribution_made_event() {
    let (env, client, creator, member2, _) = setup_test_env();
    
    let contribution = 100_000_000i128;
    let group_id = client.create_group(&creator, &contribution, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    
    client.contribute(&creator, &group_id);
    
    let events = env.events().all();
    let (_contract_id, topics, data) = events.last().unwrap();
    
    let expected_topics: Vec<soroban_sdk::Val> = (symbol_short!("contrib"), group_id, 1u32).into_val(&env);
    assert_eq!(topics, expected_topics);
    
    let payload: (Address, i128) = data.into_val(&env);
    assert_eq!(payload.0, creator);
    assert_eq!(payload.1, contribution);
}

#[test]
fn test_payout_executed_event() {
    let (env, client, creator, member2, member3) = setup_test_env();
    
    let contribution = 100_000_000i128;
    let group_id = client.create_group(&creator, &contribution, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    client.join_group(&member3, &group_id);
    
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.contribute(&member3, &group_id);
    
    client.execute_payout(&group_id);
    
    let events = env.events().all();
    // Second to last event (last is cycle_advanced)
    let event = events.iter().rev().nth(1).unwrap();
    let (_contract_id, topics, data) = event;
    
    let expected_topics: Vec<soroban_sdk::Val> = (symbol_short!("payout"), group_id, 1u32).into_val(&env);
    assert_eq!(topics, expected_topics);
    
    let payload: (Address, i128) = data.into_val(&env);
    assert_eq!(payload.0, creator);
    assert_eq!(payload.1, contribution * 3);
}

#[test]
fn test_cycle_advanced_event() {
    let (env, client, creator, member2, member3) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    client.join_group(&member3, &group_id);
    
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.contribute(&member3, &group_id);
    
    client.execute_payout(&group_id);
    
    let events = env.events().all();
    let (_contract_id, topics, data) = events.last().unwrap();
    
    let expected_topics: Vec<soroban_sdk::Val> = (symbol_short!("cycle"), group_id).into_val(&env);
    assert_eq!(topics, expected_topics);
    
    let payload: (u32, u64) = data.into_val(&env);
    assert_eq!(payload.0, 2u32); // New cycle number
    // payload.1 is the cycle_start_time (can be 0 in test env)
}

#[test]
fn test_group_completed_event() {
    let (env, client, creator, member2, member3) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    client.join_group(&member3, &group_id);
    
    // Complete all cycles
    for _ in 0..3 {
        client.contribute(&creator, &group_id);
        client.contribute(&member2, &group_id);
        client.contribute(&member3, &group_id);
        client.execute_payout(&group_id);
    }
    
    let events = env.events().all();
    let (_contract_id, topics, _data) = events.last().unwrap();
    
    let expected_topics: Vec<soroban_sdk::Val> = (symbol_short!("complete"), group_id).into_val(&env);
    assert_eq!(topics, expected_topics);
}

#[test]
fn test_all_events_include_group_id() {
    let (env, client, creator, member2, member3) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    client.join_group(&member3, &group_id);
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.contribute(&member3, &group_id);
    client.execute_payout(&group_id);
    
    let events = env.events().all();
    
    // Verify all events include group_id in topics (topic index 1)
    for event in events.iter() {
        let (_contract_id, topics, _data) = event;
        if topics.len() >= 2 {
            let gid: u64 = topics.get(1).unwrap().into_val(&env);
            assert_eq!(gid, group_id);
        }
    }
}

#[test]
fn test_contribution_events_include_amount() {
    let (env, client, creator, member2, _) = setup_test_env();
    
    let contribution = 100_000_000i128;
    let group_id = client.create_group(&creator, &contribution, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    
    let events = env.events().all();
    
    // Find contribution events
    let mut contrib_count = 0u32;
    for event in events.iter() {
        let (_contract_id, topics, data) = event;
        if topics.len() > 0 {
            let symbol: soroban_sdk::Symbol = topics.get(0).unwrap().into_val(&env);
            if symbol == symbol_short!("contrib") {
                let payload: (Address, i128) = data.into_val(&env);
                assert_eq!(payload.1, contribution);
                contrib_count += 1;
            }
        }
    }
    
    assert_eq!(contrib_count, 2);
}

#[test]
fn test_payout_events_include_recipient_and_amount() {
    let (env, client, creator, member2, member3) = setup_test_env();
    
    let contribution = 100_000_000i128;
    let group_id = client.create_group(&creator, &contribution, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    client.join_group(&member3, &group_id);
    
    // First cycle
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.contribute(&member3, &group_id);
    client.execute_payout(&group_id);
    
    let events = env.events().all();
    
    // Find payout event
    let mut found = false;
    for event in events.iter() {
        let (_contract_id, topics, data) = event;
        if topics.len() > 0 {
            let symbol: soroban_sdk::Symbol = topics.get(0).unwrap().into_val(&env);
            if symbol == symbol_short!("payout") {
                let payload: (Address, i128) = data.into_val(&env);
                assert_eq!(payload.0, creator); // First recipient
                assert_eq!(payload.1, contribution * 3); // Total pool
                found = true;
            }
        }
    }
    
    assert!(found, "Payout event not found");
}

#[test]
fn test_event_order_in_lifecycle() {
    let (env, client, creator, member2, _) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &2u32);
    client.join_group(&member2, &group_id);
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.execute_payout(&group_id);
    
    let events = env.events().all();
    let mut symbols = std::vec::Vec::new();
    
    for event in events.iter() {
        let (_contract_id, topics, _data) = event;
        if topics.len() > 0 {
            let symbol: soroban_sdk::Symbol = topics.get(0).unwrap().into_val(&env);
            symbols.push(symbol);
        }
    }
    
    assert_eq!(symbols[0], symbol_short!("created"));
    assert_eq!(symbols[1], symbol_short!("joined"));
    assert_eq!(symbols[2], symbol_short!("contrib"));
    assert_eq!(symbols[3], symbol_short!("contrib"));
    assert_eq!(symbols[4], symbol_short!("payout"));
    assert_eq!(symbols[5], symbol_short!("cycle"));
}
