module iota_identity::controller {
  use iota::transfer::Receiving;
  use iota::borrow::{Self, Referent, Borrow};
  use iota_identity::permissions;  

  public use fun delete_controller_cap as ControllerCap.delete;
  public use fun delete_delegation_token as DelegationToken.delete;

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

  /// Returns the controller's ID of this `DelegationToken`.
  public fun controller(self: &DelegationToken): ID {
    self.controller
  }

  /// Returns the permissions of this `DelegationToken`.
  public fun permissions(self: &DelegationToken): u32 {
    self.permissions
  }

  /// Returns the ID of this `DelegationToken`.
  public fun delegation_token_id(self: &DelegationToken): &UID {
    &self.id
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