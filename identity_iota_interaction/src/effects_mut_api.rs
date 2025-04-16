use crate::rpc_types::IotaObjectRef;
use crate::rpc_types::IotaTransactionBlockEffects;
use crate::rpc_types::IotaTransactionBlockEffectsAPI;
use crate::rpc_types::IotaTransactionBlockEffectsV1;
use crate::rpc_types::OwnedObjectRef;

/// A mutable version of [IotaTransactionBlockEffectsAPI] that allows the
/// in-place mutation of [IotaTransactionBlockEffects]
pub trait IotaTransactionBlockEffectsMutAPI: IotaTransactionBlockEffectsAPI {
  fn shared_objects_mut(&mut self) -> &mut Vec<IotaObjectRef>;
  fn created_mut(&mut self) -> &mut Vec<OwnedObjectRef>;
  fn mutated_mut(&mut self) -> &mut Vec<OwnedObjectRef>;
  fn unwrapped_mut(&mut self) -> &mut Vec<OwnedObjectRef>;
  fn deleted_mut(&mut self) -> &mut Vec<IotaObjectRef>;
  fn unwrapped_then_deleted_mut(&mut self) -> &mut Vec<IotaObjectRef>;
  fn wrapped_mut(&mut self) -> &mut Vec<IotaObjectRef>;
}

impl IotaTransactionBlockEffectsMutAPI for IotaTransactionBlockEffectsV1 {
  fn shared_objects_mut(&mut self) -> &mut Vec<IotaObjectRef> {
    &mut self.shared_objects
  }

  fn created_mut(&mut self) -> &mut Vec<OwnedObjectRef> {
    &mut self.created
  }

  fn mutated_mut(&mut self) -> &mut Vec<OwnedObjectRef> {
    &mut self.mutated
  }

  fn unwrapped_mut(&mut self) -> &mut Vec<OwnedObjectRef> {
    &mut self.unwrapped
  }

  fn deleted_mut(&mut self) -> &mut Vec<IotaObjectRef> {
    &mut self.deleted
  }

  fn unwrapped_then_deleted_mut(&mut self) -> &mut Vec<IotaObjectRef> {
    &mut self.unwrapped_then_deleted
  }

  fn wrapped_mut(&mut self) -> &mut Vec<IotaObjectRef> {
    &mut self.wrapped
  }
}

impl IotaTransactionBlockEffectsMutAPI for IotaTransactionBlockEffects {
  fn shared_objects_mut(&mut self) -> &mut Vec<IotaObjectRef> {
    match self {
      Self::V1(effects) => &mut effects.shared_objects,
    }
  }

  fn created_mut(&mut self) -> &mut Vec<OwnedObjectRef> {
    match self {
      Self::V1(effects) => &mut effects.created,
    }
  }

  fn mutated_mut(&mut self) -> &mut Vec<OwnedObjectRef> {
    match self {
      Self::V1(effects) => &mut effects.mutated,
    }
  }

  fn unwrapped_mut(&mut self) -> &mut Vec<OwnedObjectRef> {
    match self {
      Self::V1(effects) => &mut effects.unwrapped,
    }
  }

  fn deleted_mut(&mut self) -> &mut Vec<IotaObjectRef> {
    match self {
      Self::V1(effects) => &mut effects.deleted,
    }
  }

  fn unwrapped_then_deleted_mut(&mut self) -> &mut Vec<IotaObjectRef> {
    match self {
      Self::V1(effects) => &mut effects.unwrapped_then_deleted,
    }
  }

  fn wrapped_mut(&mut self) -> &mut Vec<IotaObjectRef> {
    match self {
      Self::V1(effects) => &mut effects.wrapped,
    }
  }
}
