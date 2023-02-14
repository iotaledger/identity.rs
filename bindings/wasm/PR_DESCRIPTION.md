# Description of change
This PR generalizes the `CredentialValidator`, `PresentationValidator` and `Resolver` in the bindings to support the use of custom DID document types (which is already possible in Rust). 

This PR also introduces locks used internally in `WasmCoreDocument` and `WasmIotaDocument` which we will need when we later introduce the new KMS API. Since the generalization and use of locks affect each other I decided to do both in the same PR in order to get an idea of how that combination might work. 

## The new `IAsCoreDocument` interface
This is the typescript analogue of `AsRef<CoreDocument>` that we use in Rust. It just requires implementers to add a method `asCoreDocument` on their document type which gives a representation of the underlying DID document as the `CoreDocument` type we ship.

 If one builds a type by inheriting from `CoreDocument` this is interface is automatically implemented (and will actually not even be called by the Identity framework in that case).  Implementers may however in some cases prefer to restrict access to their document type in which case extending `CoreDocument` is less desirable. In that case one may choose to instead implement an  `asCoreDocument` that clones the underlying `CoreDocument` (or otherwise produces it somehow). This is essentially the case for `IotaDocument`.  See the mocks in `tests/credential.ts` to get a more concrete idea of how implementers can implement this interface. 

## Custom TS functions to avoid expensive clones 
Some custom TS functions for internal use are added in the `lib` folder which are called internally by Rust. This allows us to both get a cheap copy of a document type and avoid nulling out pointers passed from TS. 

## Locks 
For now `WasmCoreDocument` and `WasmIotaDocument` wrap `Rc<tokio::sync::RwLock>`. I used `tokio::sync::RwLock` because it was the only lock I  could find with both an `async` and blocking API. The locks are currently used in such a way that it should be convenient to swap out the lock with another one in the future if desired.  

Care needs to be taken however once we introduce async methods on `WasmCoreDocument` and `WasmIotaDocument` in order to provide the thread from being blocked forever. This can happen with `Promise.all` if a user introduces an async function that blockingly reads (resp. writes) to a document while a write guard (resp. read guard) is being held over an await point.  For this reason it might be a good idea to use `try_write` instead of `blocking_write` and make the non-asynchronous functions that need `&mut` fallible. 

If it turns out we need to hold write guards across await points as well, then we might consider making every method fallible and use `RefCell` instead of `tokio::sync::RwLock` for better performance.  

## ImportedDocumentLock 
This is a new type used internally that can be thought of as an imported document from TS that "implements" `AsRef<CoreDocument>` , but with a few caveats. All calls to the custom TS functions mentioned above are made by this type.    

The `ImportedDocumentLock` type is used by the `CredentialValidator` and `PresentationValidator` and allows us to work with any document type passed in from TS as long as it implements `IAsCoreDocument`. It uses reference counting for cheap clones, but that also means we need to remember to drop it in order to prevent memory leaks. When working with arrays it might require extra allocation for the read guards, but that should not be expensive in comparison to credential/presentation validation. In theory those allocations could maybe be avoided by switching to `Arc<tokio::sync::RwLock>` (by using [this method](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html#method.try_read_owned)), or using something like the [self_cell crate](https://crates.io/crates/self_cell), but I doubt it is worth it. 

Finally the usage of `ImportedDocumentLock` could be made much nicer (and perhaps even more secure from memory leaks) if we wrote the Resolver entirely in typescript. That would also provide a nicer API for TS users and it avoids the necessity of removing some of the `AsRef<CoreDocument>` bounds one might expect in the Rust resolver that had to be done in this PR in order to accommodate for the bindings. Furthermore we could then also remove one of the generics on the Rust Resolver by requiring all handlers to provide `Send` futures thus simplifying that API. For these reasons I would recommend porting the Resolver to TS (while keeping the Rust Resolver in the Rust library of course). 


## Open questions: 
- What should we do about the possibility of TS users accidentally blocking the thread once we introduce async methods on the document(s)? (This can also be answered when that time comes). 
- Should we proceed with writing the resolver for the bindings entirely in TS? (this can also be done in a later PR and we can then also clean up other things introduced here).   