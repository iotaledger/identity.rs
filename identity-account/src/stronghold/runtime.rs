// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use futures::Future;
use hashbrown::HashMap;
use iota_stronghold::Stronghold;
use iota_stronghold::StrongholdFlags;
use once_cell::sync::OnceCell;
use riker::actors::ActorSystem;
use riker::actors::SystemBuilder;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex as SyncMutex;
use std::sync::Once;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::runtime::Runtime as AsyncRuntime;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use crate::error::Error;
use crate::error::PleaseDontMakeYourOwnResult;
use crate::error::Result;
use crate::stronghold::SnapshotStatus;

#[derive(Debug)]
pub(crate) struct Runtime {
  state: Arc<Mutex<ActorState>>,
}

impl Runtime {
  fn get() -> Result<&'static Self> {
    static __THIS: OnceCell<Runtime> = OnceCell::new();

    __THIS.get_or_try_init(|| {
      let system: ActorSystem = SystemBuilder::new()
        // // Disable the default actor system logger
        // .log(slog::Logger::root(slog::Discard, slog::o!()))
        .create()?;

      let stronghold: Stronghold = Stronghold::init_stronghold_system(system, Vec::new(), Vec::new());
      let state: Arc<Mutex<ActorState>> = Arc::new(Mutex::new(ActorState::new(stronghold)));

      Ok(Self { state })
    })
  }

  pub(crate) async fn lock() -> Result<MutexGuard<'static, ActorState>> {
    match Self::get() {
      Ok(this) => Ok(this.state.lock().await),
      Err(error) => Err(error),
    }
  }

  pub(crate) async fn set_password(path: &Path, password: Password) {
    Passwords::get().store(path, password).await;
  }

  pub(crate) async fn snapshot_status(path: &Path) -> SnapshotStatus {
    let passwords: &'static Passwords = Passwords::get();

    if let Some(elapsed) = passwords.elapsed(path).await {
      let interval: Duration = passwords.interval().await;
      let locked: bool = interval.as_millis() > 0 && elapsed >= interval;

      if locked {
        SnapshotStatus::locked()
      } else if interval.as_millis() == 0 {
        SnapshotStatus::unlocked(interval)
      } else {
        SnapshotStatus::unlocked(interval - elapsed)
      }
    } else {
      SnapshotStatus::locked()
    }
  }

  #[allow(dead_code)]
  pub(crate) async fn bind_listener<L>(listener: L)
  where
    L: Into<Listener>,
  {
    Listeners::get().data.lock().await.push(listener.into());
  }

  pub(crate) async fn emit_change(path: &Path, status: SnapshotStatus) {
    for listener in Listeners::get().data.lock().await.iter_mut() {
      (listener.0)(path, &status);
    }
  }
}

// =============================================================================
// ActorState
// =============================================================================

pub(crate) struct ActorState {
  stronghold: Stronghold,
  clients_active: HashSet<Vec<u8>>,
  clients_loaded: HashSet<Vec<u8>>,
}

impl ActorState {
  fn new(stronghold: Stronghold) -> Self {
    Self {
      stronghold,
      clients_active: HashSet::new(),
      clients_loaded: HashSet::new(),
    }
  }

  pub(crate) async fn set_snapshot(&mut self, path: &Path) -> Result<()> {
    let mut current: _ = CurrentSnapshot::lock().await;

    if current.is_neq(path) {
      self.flush(path, true).await?;
    }

    current.set(path);

    Ok(())
  }

  pub(crate) async fn write(&mut self, path: &Path, persist: bool) -> Result<()> {
    let mut current: _ = CurrentSnapshot::lock().await;

    if current.is_eq(path) {
      self.flush(path, persist).await?;
    }

    current.clear();

    Ok(())
  }

  pub(crate) async fn flush(&mut self, path: &Path, persist: bool) -> Result<()> {
    if persist && !self.clients_loaded.is_empty() {
      self.write_snapshot(path).await?;
    }

    for client in self.clients_loaded.iter() {
      self
        .stronghold
        .kill_stronghold(client.clone(), false)
        .await
        .to_result()?;

      self
        .stronghold
        .kill_stronghold(client.clone(), true)
        .await
        .to_result()?;
    }

    // delay to wait for the actors to be killed
    thread::sleep(Duration::from_millis(300));

    self.clients_active.clear();
    self.clients_loaded.clear();

    Ok(())
  }

  pub(crate) async fn write_snapshot(&mut self, path: &Path) -> Result<()> {
    let password: Password = Passwords::get().load(path).await?;

    self
      .stronghold
      .write_all_to_snapshot(&password.to_vec(), None, Some(path.to_path_buf()))
      .await
      .to_result()
  }

  pub(crate) async fn load_actor(&mut self, snapshot: &Path, client: &[u8], flags: &[StrongholdFlags]) -> Result<()> {
    Passwords::get().touch(snapshot).await?;

    if self.clients_active.contains(client) {
      self.stronghold.switch_actor_target(client.into()).to_result()?;
    } else {
      self
        .stronghold
        .spawn_stronghold_actor(client.into(), flags.to_vec())
        .to_result()?;

      self.clients_active.insert(client.into());
    }

    if !self.clients_loaded.contains(client) {
      if snapshot.exists() {
        self
          .stronghold
          .read_snapshot(
            client.into(),
            None,
            &Passwords::get().load(snapshot).await?.to_vec(),
            None,
            Some(snapshot.to_path_buf()),
          )
          .await
          .to_result()?;
      }

      self.clients_loaded.insert(client.into());
    }

    Ok(())
  }
}

impl Deref for ActorState {
  type Target = Stronghold;

  fn deref(&self) -> &Self::Target {
    &self.stronghold
  }
}

impl DerefMut for ActorState {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.stronghold
  }
}

impl Debug for ActorState {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.debug_struct("ActorState")
      .field("clients_active", &self.clients_active)
      .field("clients_loaded", &self.clients_loaded)
      .finish()
  }
}

// =============================================================================
// CurrentSnapshot
// =============================================================================

#[derive(Debug, Default)]
struct CurrentSnapshot {
  state: Arc<Mutex<CurrentSnapshotState>>,
}

impl CurrentSnapshot {
  async fn lock() -> MutexGuard<'static, CurrentSnapshotState> {
    Self::get().state.lock().await
  }

  fn get() -> &'static Self {
    static __THIS: OnceCell<CurrentSnapshot> = OnceCell::new();
    __THIS.get_or_init(Default::default)
  }
}

#[derive(Debug, Default)]
struct CurrentSnapshotState(Option<PathBuf>);

impl CurrentSnapshotState {
  fn is_eq(&self, other: &Path) -> bool {
    match self.0.as_ref() {
      Some(data) => data == other,
      None => false,
    }
  }

  fn is_neq(&self, other: &Path) -> bool {
    match self.0.as_ref() {
      Some(data) => data != other,
      None => false,
    }
  }

  fn set(&mut self, path: &Path) {
    self.0.replace(path.to_path_buf());
  }

  fn clear(&mut self) {
    self.0 = None;
  }
}

// =============================================================================
// Task
// =============================================================================

pub(crate) struct Task {
  data: SyncMutex<AsyncRuntime>,
}

impl Task {
  fn get() -> Result<&'static Self> {
    static __THIS: OnceCell<Task> = OnceCell::new();

    __THIS.get_or_try_init(|| {
      Ok(Task {
        data: SyncMutex::new(AsyncRuntime::new()?),
      })
    })
  }

  pub(crate) async fn spawn<F, T>(future: F) -> Result<()>
  where
    F: Future<Output = Result<T>> + Send + 'static,
    T: Send + 'static,
  {
    let tpool: _ = Self::get()?;
    let guard: _ = tpool.data.lock().unwrap();

    guard.spawn(future);

    Ok(())
  }
}

// =============================================================================
// Passwords
// =============================================================================

pub type Password = [u8; 32];

struct Passwords {
  store: Mutex<HashMap<PathBuf, (Password, Instant)>>,
  sweep: Mutex<Duration>,
}

impl Passwords {
  const DEFAULT_SWEEP: Duration = Duration::from_millis(0);

  fn get() -> &'static Self {
    static __THIS: OnceCell<Passwords> = OnceCell::new();
    static __SWEEP: Once = Once::new();

    let this: &Self = __THIS.get_or_init(|| Self {
      store: Mutex::new(HashMap::new()),
      sweep: Mutex::new(Self::DEFAULT_SWEEP),
    });

    __SWEEP.call_once(|| {
      thread::spawn(|| async {
        Task::spawn::<_, ()>(async {
          loop {
            Passwords::prune().await?;
          }
        })
        .await
      });
    });

    this
  }

  async fn interval(&self) -> Duration {
    *self.sweep.lock().await
  }

  async fn elapsed(&self, path: &Path) -> Option<Duration> {
    self
      .store
      .lock()
      .await
      .get(path)
      .map(|(_, interval)| interval.elapsed())
  }

  async fn store(&self, path: &Path, password: Password) {
    self
      .store
      .lock()
      .await
      .insert(path.to_path_buf(), (password, Instant::now()));
  }

  async fn load(&self, path: &Path) -> Result<Password> {
    self
      .store
      .lock()
      .await
      .get(path)
      .map(|(password, _)| *password)
      .ok_or(Error::StrongholdPasswordNotSet)
  }

  async fn drain(&self, interval: Duration) -> Vec<PathBuf> {
    self
      .store
      .lock()
      .await
      .drain_filter(|_, (_, instant)| instant.elapsed() > interval)
      .map(|(path, _)| path)
      .collect()
  }

  async fn touch(&self, path: &Path) -> Result<()> {
    if let Some((_, ref mut time)) = self.store.lock().await.get_mut(path) {
      *time = Instant::now();
    } else {
      return Err(Error::StrongholdPasswordNotSet);
    }

    Ok(())
  }

  async fn prune() -> Result<()> {
    let passwords: &Self = Self::get();
    let interval: Duration = passwords.interval().await;

    thread::sleep(interval);

    if interval.as_nanos() == 0 {
      return Ok(());
    }

    let cleared: Vec<PathBuf> = passwords.drain(interval).await;
    let current: _ = CurrentSnapshot::lock().await;

    for path in cleared {
      if current.is_eq(&path) {
        Runtime::lock().await?.flush(&path, true).await?;
      }

      Runtime::emit_change(&path, SnapshotStatus::locked()).await;
    }

    Ok(())
  }
}

// =============================================================================
// Event Listeners
// =============================================================================

pub struct Listener(Box<dyn FnMut(&Path, &SnapshotStatus) + Send>);

impl<T> From<T> for Listener
where
  T: FnMut(&Path, &SnapshotStatus) + Send + 'static,
{
  fn from(other: T) -> Self {
    Self(Box::new(other))
  }
}

#[derive(Default)]
struct Listeners {
  data: Arc<Mutex<Vec<Listener>>>,
}

impl Listeners {
  fn get() -> &'static Self {
    static __THIS: OnceCell<Listeners> = OnceCell::new();
    __THIS.get_or_init(Default::default)
  }
}
