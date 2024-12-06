module iota_identity::controller {
  use iota::transfer::Receiving;
  use iota::borrow::{Self, Referent, Borrow};
  use iota_identity::permissions;  

  public use fun delete_controller_cap as ControllerCap.delete;
  public use fun delete_delegation_token as DelegationToken.delete;
  public use fun delegation_token_id as DelegationToken.id;

  /// This `ControllerCap` cannot delegate access.
  const ECannotDelegate: u64 = 0;
  // The permission of the provided `DelegationToken` are not
  // valid to perform this operation.
  const EInvalidPermissions: u64 = 1;

  /// Event that is created when a new `DelegationToken` is minted.
  public struct NewDelegationTokenEvent has copy, drop {
    controller: ID,
    token: ID,
    permissions: u32,
  }

  /// Capability that allows to access mutative APIs of a `Multicontroller`.
  public struct ControllerCap has key {
    id: UID,
    can_delegate: bool,
    access_token: Referent<DelegationToken>,
  }

  public fun id(self: &ControllerCap): &UID {
    &self.id
  }

  /// Borrows this `ControllerCap`'s access token.
  public fun borrow(self: &mut ControllerCap): (DelegationToken, Borrow) {
    self.access_token.borrow()
  }

  /// Returns the borrowed access token together with the hot potato.
  public fun put_back(self: &mut ControllerCap, token: DelegationToken, borrow: Borrow) {
    self.access_token.put_back(token, borrow);
  }

  /// Creates a delegation token for this controller. The created `DelegationToken`
  /// will have full permissions. Use `delegate_with_permissions` to set or unset
  /// specific permissions.
  public fun delegate(self: &ControllerCap, ctx: &mut TxContext): DelegationToken {
    assert!(self.can_delegate, ECannotDelegate);
    new_delegation_token(self.id.to_inner(), permissions::all(), ctx)
  }

  /// Creates a delegation token for this controller, specifying the delegate's permissions.
  public fun delegate_with_permissions(self: &ControllerCap, permissions: u32, ctx: &mut TxContext): DelegationToken {
    assert!(self.can_delegate, ECannotDelegate);
    new_delegation_token(self.id.to_inner(), permissions, ctx)
  }

  /// A token that allows an entity to act in a Controller's stead.
  public struct DelegationToken has key, store {
    id: UID,
    permissions: u32,
    controller: ID,
  }

  /// Returns the ID of this `DelegationToken`.
  public fun delegation_token_id(self: &DelegationToken): ID {
    self.id.to_inner()
  }

  /// Returns the controller's ID of this `DelegationToken`.
  public fun controller(self: &DelegationToken): ID {
    self.controller
  }

  /// Returns the permissions of this `DelegationToken`.
  public fun permissions(self: &DelegationToken): u32 {
    self.permissions
  }

  /// Returns true if this `DelegationToken` has permission `permission`.
  public fun has_permission(self: &DelegationToken, permission: u32): bool {
    self.permissions & permission != 0
  }

  /// Aborts if this `DelegationToken` doesn't have permission `permission`.
  public fun assert_has_permission(self: &DelegationToken, permission: u32) {
    assert!(self.has_permission(permission), EInvalidPermissions)
  }

  /// Creates a new `ControllerCap`.
  public(package) fun new(can_delegate: bool, ctx: &mut TxContext): ControllerCap {
    let id = object::new(ctx);
    let access_token = borrow::new(new_delegation_token(id.to_inner(), permissions::all(), ctx), ctx);

    ControllerCap {
      id,
      access_token,
      can_delegate,
    }
  }

  /// Transfer a `ControllerCap`.
  public(package) fun transfer(cap: ControllerCap, recipient: address) {
    transfer::transfer(cap, recipient)
  }

  /// Receives a `ControllerCap`.
  public(package) fun receive(owner: &mut UID, cap: Receiving<ControllerCap>): ControllerCap {
    transfer::receive(owner, cap)
  }

  public(package) fun new_delegation_token(
    controller: ID,
    permissions: u32,
    ctx: &mut TxContext
  ): DelegationToken {
    let id = object::new(ctx);

    iota::event::emit(NewDelegationTokenEvent {
      controller,
      token: id.to_inner(),
      permissions,
    });

    DelegationToken {
      id,
      controller,
      permissions,
    }
  }

  public(package) fun delete_controller_cap(cap: ControllerCap) {
    let ControllerCap {
      access_token,
      id,
      ..
    } = cap;

    delete_delegation_token(access_token.destroy());
    object::delete(id);
  }

  public(package) fun delete_delegation_token(token: DelegationToken) {
    let DelegationToken {
      id,
      ..
    } = token;
    object::delete(id);
  }
}

#[test_only]
module iota_identity::controller_tests {
  use iota::test_scenario;
  use iota_identity::controller::{Self, ControllerCap, ECannotDelegate, EInvalidPermissions};
  use iota_identity::permissions;
  use iota_identity::multicontroller::{Self, Multicontroller};

  #[test, expected_failure(abort_code = ECannotDelegate)]
  fun test_only_delegatable_controllers_can_create_delegation_tokens() {
    let owner = @0x1;
    let mut scenario = test_scenario::begin(owner);

    let non_delegatable = controller::new(false, scenario.ctx());
    let delegation_token = non_delegatable.delegate(scenario.ctx());

    delegation_token.delete();
    non_delegatable.delete();
    scenario.end();
  }
  
  #[test, expected_failure(abort_code = EInvalidPermissions)]
  fun delegate_cannot_create_proposal_when_missing_permission() {
    let controller = @0x1;
    let mut scenario = test_scenario::begin(controller);

    let mut multicontroller: Multicontroller<u64> = multicontroller::new(0, true, scenario.ctx());
    scenario.next_tx(controller);

    let controller_cap = scenario.take_from_address<ControllerCap>(controller);
    let delegation_token = controller_cap.delegate_with_permissions(
      permissions::all() & permissions::not(permissions::can_create_proposal()),
      scenario.ctx(),
    );

    scenario.next_tx(controller);

    multicontroller.create_proposal<_, u64>(
      &delegation_token,
      0,
      option::none(),
      scenario.ctx(),
    );

    abort(0)
  }

  #[test, expected_failure(abort_code = EInvalidPermissions)]
  fun delegate_cannot_execute_proposal_when_missing_permission() {
    let controller = @0x1;
    let mut scenario = test_scenario::begin(controller);

    let mut multicontroller: Multicontroller<u64> = multicontroller::new(0, true, scenario.ctx());
    scenario.next_tx(controller);

    let controller_cap = scenario.take_from_address<ControllerCap>(controller);
    let delegation_token = controller_cap.delegate_with_permissions(
      permissions::all() & permissions::not(permissions::can_execute_proposal()),
      scenario.ctx(),
    );

    scenario.next_tx(controller);

    let proposal_id = multicontroller.create_proposal<_, u64>(
      &delegation_token,
      0,
      option::none(),
      scenario.ctx(),
    );

    multicontroller.execute_proposal<_, u64>(
      &delegation_token,
      proposal_id,
      scenario.ctx(),
    ).unwrap();

    abort(0)
  }

  #[test, expected_failure(abort_code = EInvalidPermissions)]
  fun delegate_cannot_approve_proposal_when_missing_permission() {
    let controller = @0x1;
    let mut scenario = test_scenario::begin(controller);

    let mut multicontroller: Multicontroller<u64> = multicontroller::new(0, true, scenario.ctx());
    scenario.next_tx(controller);

    let controller_cap = scenario.take_from_address<ControllerCap>(controller);
    let delegation_token = controller_cap.delegate_with_permissions(
      permissions::all() & permissions::not(permissions::can_approve_proposal()),
      scenario.ctx(),
    );

    scenario.next_tx(controller);

    let proposal_id = multicontroller.create_proposal<_, u64>(
      &delegation_token,
      0,
      option::none(),
      scenario.ctx(),
    );

    multicontroller.approve_proposal<_, u64>(
      &delegation_token,
      proposal_id,
    );

    abort(0)
  }

  #[test, expected_failure(abort_code = EInvalidPermissions)]
  fun delegate_cannot_remove_approval_when_missing_permission() {
    let controller = @0x1;
    let mut scenario = test_scenario::begin(controller);

    let mut multicontroller: Multicontroller<u64> = multicontroller::new(0, true, scenario.ctx());
    scenario.next_tx(controller);

    let controller_cap = scenario.take_from_address<ControllerCap>(controller);
    let delegation_token = controller_cap.delegate_with_permissions(
      permissions::all() & permissions::not(permissions::can_remove_approval()),
      scenario.ctx(),
    );

    scenario.next_tx(controller);

    let proposal_id = multicontroller.create_proposal<_, u64>(
      &delegation_token,
      0,
      option::none(),
      scenario.ctx(),
    );

    multicontroller.remove_approval<_, u64>(
      &delegation_token,
      proposal_id,
    );

    abort(0)
  }

  #[test, expected_failure(abort_code = EInvalidPermissions)]
  fun delegate_cannot_delete_proposal_when_missing_permission() {
    let controller = @0x1;
    let mut scenario = test_scenario::begin(controller);

    let mut multicontroller: Multicontroller<u64> = multicontroller::new(0, true, scenario.ctx());
    scenario.next_tx(controller);

    let controller_cap = scenario.take_from_address<ControllerCap>(controller);
    let delegation_token = controller_cap.delegate_with_permissions(
      permissions::all() & permissions::not(permissions::can_remove_approval()),
      scenario.ctx(),
    );

    scenario.next_tx(controller);

    let proposal_id = multicontroller.create_proposal<_, u64>(
      &delegation_token,
      0,
      option::none(),
      scenario.ctx(),
    );

    multicontroller.remove_approval<_, u64>(
      &delegation_token,
      proposal_id,
    );

    multicontroller.delete_proposal<_, u64>(
      &delegation_token,
      proposal_id,
    );

    abort(0)
  }
}