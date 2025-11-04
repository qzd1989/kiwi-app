use crate::{
    app::{Log, api::Api, role::Role, store::Store},
    capture::{Engine as CaptureEngine, Frame},
    commands::server::Engine as ServerEngine,
    input::Engine as InputEngine,
    project::Project,
};
use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{collections::HashMap, path::PathBuf};
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;
use tokio::sync::Mutex;

static APP: OnceLock<Arc<App>> = OnceLock::new();
static APP_HANDLE: OnceLock<Arc<AppHandle>> = OnceLock::new();
static SERVER: OnceLock<Arc<Mutex<ServerEngine>>> = OnceLock::new(); //因为有异步方法serve_in_background，所以用tokio::sync::Mutex
static CAPTURER: OnceLock<Arc<CaptureEngine>> = OnceLock::new();
static INPUT: OnceLock<Arc<InputEngine>> = OnceLock::new();
static STORE: OnceLock<Arc<Store>> = OnceLock::new();

pub struct App {
    pub role: Arc<RwLock<Role>>,                    //是listener还是user?
    pub remote_server_address: Arc<RwLock<String>>, //脚本连接哪个server? 本地还是远程？
    project: Arc<RwLock<Option<Project>>>,
}

// Basic
impl App {
    pub fn get() -> Arc<Self> {
        APP.get_or_init(|| Arc::new(App::default())).clone()
    }

    pub fn version(&self) -> String {
        self.app_handle().config().version.clone().unwrap()
    }

    pub fn name(&self) -> String {
        self.app_handle().config().product_name.clone().unwrap()
    }

    pub fn log(&self) -> Log {
        Log::new(self.resource_dir().join("log"))
    }
}

// Self.role
impl App {
    pub fn to_listener(&self) {
        *self.role.write().unwrap() = Role::Listener;
    }

    pub fn to_user(&self) {
        *self.role.write().unwrap() = Role::User;
    }

    pub fn role(&self) -> Role {
        *self.role.read().unwrap()
    }
}

// Self.remote_server_address
impl App {
    pub fn remote_server_address(&self) -> String {
        self.remote_server_address.read().unwrap().clone()
    }

    pub fn set_remote_server_address(&self, address: impl Into<String>) {
        *self.remote_server_address.write().unwrap() = address.into();
    }
}

// dirs
impl App {
    pub fn resource_dir(&self) -> PathBuf {
        self.app_handle()
            .path()
            .resource_dir()
            .expect("Can't find resouce dir.")
    }
}

// get api via curl
impl App {
    pub async fn api_get_without_params<T: DeserializeOwned>(
        &self,
        path: impl Into<String>,
    ) -> Result<T> {
        self.api_get(path, std::iter::empty::<(&'static str, String)>())
            .await
    }

    pub async fn api_get<T: DeserializeOwned>(
        &self,
        path: impl Into<String>,
        params: impl IntoIterator<Item = (&'static str, String)>,
    ) -> Result<T> {
        let params = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<HashMap<String, String>>();

        Api::get(path, params).await
    }
}

// server
impl App {
    pub fn server(&self) -> Arc<Mutex<ServerEngine>> {
        SERVER
            .get_or_init(|| Arc::new(Mutex::new(ServerEngine::default())))
            .clone()
    }
}

// capturer
impl App {
    pub fn capturer(&self) -> Arc<CaptureEngine> {
        CAPTURER
            .get_or_init(|| Arc::new(CaptureEngine::default()))
            .clone()
    }

    pub fn with_capturer<R>(&self, f: impl FnOnce(&CaptureEngine) -> R) -> R {
        f(&*self.capturer())
    }

    pub fn capture(&self) -> Result<Frame> {
        self.with_capturer(|capturer| capturer.grab(2000))
    }

    pub fn get_frame(&self) -> Result<Frame> {
        self.with_capturer(|capturer| capturer.get_frame().ok_or(anyhow!("Frame not found.")))
    }

    pub fn get_frame_arc(&self) -> Result<Arc<Frame>> {
        self.with_capturer(|capturer| capturer.get_frame_arc().ok_or(anyhow!("Frame not found.")))
    }
}

// project
impl App {
    pub fn project(&'_ self) -> Result<RwLockReadGuard<'_, Option<Project>>> {
        self.project.read().map_err(|e| anyhow!(e.to_string()))
    }

    pub fn project_mut(&'_ self) -> Result<RwLockWriteGuard<'_, Option<Project>>> {
        self.project.write().map_err(|e| anyhow!(e.to_string()))
    }

    pub fn try_with_project<R>(&self, f: impl FnOnce(&Project) -> R) -> Result<R> {
        let app = App::get();
        let guard = app.project.read().unwrap();

        if let Some(project) = guard.as_ref() {
            Ok(f(project))
        } else {
            Err(anyhow!("Project not found."))
        }
    }
}

// input
impl App {
    pub fn input(&self) -> Arc<InputEngine> {
        INPUT
            .get_or_init(|| Arc::new(InputEngine::default()))
            .clone()
    }

    pub fn with_input<R>(&self, f: impl FnOnce(&InputEngine) -> R) -> R {
        f(&*self.input())
    }
}

//app handle
impl App {
    pub fn init_app_handle(&self, app_handle: Arc<AppHandle>) -> Result<()> {
        APP_HANDLE
            .set(app_handle)
            .map_err(|_| anyhow!("Failed to initialize app handle."))?;
        Ok(())
    }

    pub fn app_handle(&self) -> Arc<AppHandle> {
        APP_HANDLE.get().unwrap().clone()
    }
}

// locale
impl App {
    pub fn set_locale<S: AsRef<str>>(&self, locale: S) {
        rust_i18n::set_locale(locale.as_ref());
    }

    pub fn locale(&self) -> String {
        self.store()
            .get_string("locale")
            .unwrap_or("en-US".to_string())
    }
}

// store
impl App {
    pub fn store(&self) -> Arc<Store> {
        STORE
            .get_or_init(|| {
                let store = self
                    .app_handle()
                    .store("store.json")
                    .expect("Failed to initialize store");
                Arc::new(Store::new(store))
            })
            .clone()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            role: Arc::new(RwLock::new(Role::User)),
            remote_server_address: Arc::new(RwLock::new(ServerEngine::local_address())),
            project: Arc::new(RwLock::new(None)),
        }
    }
}
