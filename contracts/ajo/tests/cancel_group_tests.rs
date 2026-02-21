#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_ajo::{AjoContract, AjoContractClient, AjoError};

/// Helper function to create a test environment and contract
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

// ──────────────────────────────────────────
// Happy path
// ──────────────────────────────────────────

#[test]
fn test_cancel_group_by_creator() {
    let (_env, client, creator, member2, _) = setup_test_env();
    
    // Create group and add a member
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &5u32);
    client.join_group(&member2, &group_id);
    
    // Creator cancels
    client.cancel_group(&creator, &group_id);
    
    // Group should now be complete and cancelled
    let group = client.get_group(&group_id);
    assert_eq!(group.is_complete, true);
    assert_eq!(group.is_cancelled, true);
}

#[test]
fn test_cancel_group_with_contributions() {
    let (_env, client, creator, member2, member3) = setup_test_env();
    
    // Create group with 3 members
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    client.join_group(&member3, &group_id);
    
    // All members contribute in cycle 1
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.contribute(&member3, &group_id);
    
    // Execute payout for cycle 1, advancing to cycle 2
    client.execute_payout(&group_id);
    
    // Some members contribute in cycle 2
    client.contribute(&creator, &group_id);
    
    // Creator cancels mid-cycle
    client.cancel_group(&creator, &group_id);
    
    // Verify state
    let group = client.get_group(&group_id);
    assert_eq!(group.is_complete, true);
    assert_eq!(group.is_cancelled, true);
    assert_eq!(group.current_cycle, 2); // Was on cycle 2 when cancelled
}

#[test]
fn test_cancel_group_only_creator_as_member() {
    let (_env, client, creator, _, _) = setup_test_env();
    
    // Create group with just the creator
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &5u32);
    
    // Cancel immediately
    client.cancel_group(&creator, &group_id);
    
    let group = client.get_group(&group_id);
    assert_eq!(group.is_complete, true);
    assert_eq!(group.is_cancelled, true);
}

// ──────────────────────────────────────────
// Authorization / guard checks
// ──────────────────────────────────────────

#[test]
fn test_cancel_group_not_creator_fails() {
    let (_env, client, creator, member2, _) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &5u32);
    client.join_group(&member2, &group_id);
    
    // Non-creator tries to cancel
    let result = client.try_cancel_group(&member2, &group_id);
    assert_eq!(result, Err(Ok(AjoError::Unauthorized)));
}

#[test]
fn test_cancel_completed_group_fails() {
    let (_env, client, creator, member2, _) = setup_test_env();
    
    // Create a 2-member group and complete it
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &2u32);
    client.join_group(&member2, &group_id);
    
    // Complete both cycles
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.execute_payout(&group_id);
    
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.execute_payout(&group_id);
    
    assert_eq!(client.is_complete(&group_id), true);
    
    // Try to cancel completed group
    let result = client.try_cancel_group(&creator, &group_id);
    assert_eq!(result, Err(Ok(AjoError::GroupComplete)));
}

#[test]
fn test_cancel_already_cancelled_fails() {
    let (_env, client, creator, _, _) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &5u32);
    
    // Cancel once
    client.cancel_group(&creator, &group_id);
    
    // Try to cancel again — is_complete is true so GroupComplete is returned
    let result = client.try_cancel_group(&creator, &group_id);
    assert_eq!(result, Err(Ok(AjoError::GroupComplete)));
}

// ──────────────────────────────────────────
// Post-cancellation behavior
// ──────────────────────────────────────────

#[test]
fn test_no_join_after_cancel() {
    let (env, client, creator, _, _) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &5u32);
    client.cancel_group(&creator, &group_id);
    
    // Try to join cancelled group
    let new_member = Address::generate(&env);
    let result = client.try_join_group(&new_member, &group_id);
    assert_eq!(result, Err(Ok(AjoError::GroupComplete)));
}

#[test]
fn test_no_contribute_after_cancel() {
    let (_env, client, creator, member2, _) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &5u32);
    client.join_group(&member2, &group_id);
    client.cancel_group(&creator, &group_id);
    
    // Try to contribute to cancelled group
    let result = client.try_contribute(&creator, &group_id);
    assert_eq!(result, Err(Ok(AjoError::GroupComplete)));
}

#[test]
fn test_no_payout_after_cancel() {
    let (_env, client, creator, member2, member3) = setup_test_env();
    
    let group_id = client.create_group(&creator, &100_000_000i128, &604_800u64, &3u32);
    client.join_group(&member2, &group_id);
    client.join_group(&member3, &group_id);
    
    // All contribute then cancel before payout
    client.contribute(&creator, &group_id);
    client.contribute(&member2, &group_id);
    client.contribute(&member3, &group_id);
    client.cancel_group(&creator, &group_id);
    
    // Try to execute payout on cancelled group
    let result = client.try_execute_payout(&group_id);
    assert_eq!(result, Err(Ok(AjoError::GroupComplete)));
}
