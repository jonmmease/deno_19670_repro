use crate::module_loader::import_map::{vega_url, VlVersion};
use crate::module_loader::VlConvertModuleLoader;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use deno_runtime::deno_core::anyhow::bail;
use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::{serde_v8, v8, Extension};

use deno_core::{op, ModuleCode};
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::{Permissions, PermissionsContainer};
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use futures::channel::{mpsc, mpsc::Sender, oneshot};
use futures_util::{SinkExt, StreamExt};
use std::thread;
use std::thread::JoinHandle;

lazy_static! {
    pub static ref TOKIO_RUNTIME: tokio::runtime::Runtime =
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
    static ref JSON_ARGS: Arc<Mutex<HashMap<i32, String>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref NEXT_ARG_ID: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

fn set_json_arg(arg: serde_json::Value) -> Result<i32, AnyError> {
    // Increment arg id
    let id = match NEXT_ARG_ID.lock() {
        Ok(mut guard) => {
            let id = *guard;
            *guard = (*guard + 1) % i32::MAX;
            id
        }
        Err(err) => {
            bail!("Failed to acquire lock: {}", err.to_string())
        }
    };

    // Add Arg at id to args
    match JSON_ARGS.lock() {
        Ok(mut guard) => {
            guard.insert(id, serde_json::to_string(&arg).unwrap());
        }
        Err(err) => {
            bail!("Failed to acquire lock: {}", err.to_string())
        }
    }

    Ok(id)
}

#[op]
fn op_get_json_arg(arg_id: i32) -> Result<String, AnyError> {
    match JSON_ARGS.lock() {
        Ok(mut guard) => {
            if let Some(arg) = guard.remove(&arg_id) {
                Ok(arg)
            } else {
                bail!("Arg id not found")
            }
        }
        Err(err) => {
            bail!("Failed to acquire lock: {}", err.to_string())
        }
    }
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

/// Struct that interacts directly with the Deno JavaScript runtime.
/// Not Sendable
pub struct InnerVlConverter {
    worker: MainWorker,
    initialized_vl_versions: HashSet<VlVersion>,
    vega_initialized: bool,
}

impl InnerVlConverter {
    async fn init_vega(&mut self) -> Result<(), AnyError> {
        if !self.vega_initialized {
            let import_code = ModuleCode::from(format!(
                r#"
var vega;
import('{vega_url}').then((imported) => {{
    vega = imported;
}})

"#,
                vega_url = vega_url(),
            ));

            self.worker.execute_script("<anon>", import_code)?;
            self.worker.run_event_loop(false).await?;

            // Create and initialize svg function string
            let function_str = r#"
function vegaToSvg(vgSpec) {
    let runtime = vega.parse(vgSpec);
    let view = new vega.View(runtime, {renderer: 'none'});
    let svgPromise = view.toSVG();
    return svgPromise
}
"#;

            self.worker
                .execute_script("<anon>", deno_core::FastString::Static(function_str))?;
            self.worker.run_event_loop(false).await?;

            self.vega_initialized = true;
        }

        Ok(())
    }

    async fn init_vl_version(&mut self, vl_version: &VlVersion) -> Result<(), AnyError> {
        if !self.initialized_vl_versions.contains(vl_version) {
            // Create and evaluate import string
            let import_code = ModuleCode::from(format!(
                r#"
var {ver_name};
import('{vl_url}').then((imported) => {{
    {ver_name} = imported;
}})
"#,
                ver_name = format!("{:?}", vl_version),
                vl_url = vl_version.to_url()
            ));

            self.worker.execute_script("<anon>", import_code)?;
            self.worker.run_event_loop(false).await?;

            // Create and initialize function string
            let function_code = ModuleCode::from(format!(
                r#"
function vegaLiteToSvg_{ver_name}(vlSpec) {{
    let vgSpec = {ver_name}.compile(vlSpec).spec;
    return vegaToSvg(vgSpec)
}}
"#,
                ver_name = format!("{:?}", vl_version),
            ));

            self.worker.execute_script("<anon>", function_code)?;
            self.worker.run_event_loop(false).await?;

            // Register that this Vega-Lite version has been initialized
            self.initialized_vl_versions.insert(*vl_version);
        }
        Ok(())
    }

    pub async fn try_new() -> Result<Self, AnyError> {
        let module_loader = Rc::new(VlConvertModuleLoader::new());

        let ext = Extension::builder("vl_convert_extensions")
            .ops(vec![op_get_json_arg::decl()])
            .build();

        let create_web_worker_cb = Arc::new(|_| {
            todo!("Web workers are not supported");
        });
        let web_worker_event_cb = Arc::new(|_| {
            todo!("Web workers are not supported");
        });

        let options = WorkerOptions {
            bootstrap: Default::default(),
            extensions: vec![ext],
            startup_snapshot: None,
            create_params: None,
            unsafely_ignore_certificate_errors: None,
            root_cert_store_provider: None,
            seed: None,
            source_map_getter: None,
            format_js_error_fn: None,
            web_worker_preload_module_cb: web_worker_event_cb.clone(),
            web_worker_pre_execute_module_cb: web_worker_event_cb,
            create_web_worker_cb,
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            module_loader: module_loader.clone(),
            npm_resolver: None,
            get_error_class_fn: Some(&get_error_class_name),
            cache_storage_dir: None,
            origin_storage_dir: None,
            blob_store: BlobStore::default(),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
            stdio: Default::default(),
            should_wait_for_inspector_session: false,
            fs: Arc::new(RealFs),
        };

        let main_module =
            deno_core::resolve_path("vl-convert-rs.js", Path::new(env!("CARGO_MANIFEST_DIR")))?;
        let permissions = PermissionsContainer::new(Permissions::allow_all());

        let mut worker =
            MainWorker::bootstrap_from_options(main_module.clone(), permissions, options);
        worker.execute_main_module(&main_module).await?;
        worker.run_event_loop(false).await?;

        let this = Self {
            worker,
            initialized_vl_versions: Default::default(),
            vega_initialized: false,
        };

        Ok(this)
    }

    async fn execute_script_to_string(&mut self, script: &str) -> Result<String, AnyError> {
        let code = ModuleCode::from(script.to_string());
        let res = self.worker.js_runtime.execute_script("<anon>", code)?;

        self.worker.run_event_loop(false).await?;

        let scope = &mut self.worker.js_runtime.handle_scope();
        let local = v8::Local::new(scope, res);

        // Deserialize a `v8` object into a Rust type using `serde_v8`,
        // in this case deserialize to a JSON `Value`.
        let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local);

        let value = match deserialized_value {
            Ok(value) => {
                let value = value.as_str();
                value.unwrap().to_string()
            }
            Err(err) => bail!("{}", err.to_string()),
        };

        Ok(value)
    }

    pub async fn vegalite_to_svg(
        &mut self,
        vl_spec: serde_json::Value,
    ) -> Result<String, AnyError> {
        self.init_vega().await?;
        self.init_vl_version(&VlVersion::v5_11).await?;

        let spec_arg_id = set_json_arg(vl_spec)?;
        let code = ModuleCode::from(format!(
            r#"
var svg;
vegaLiteToSvg_{ver_name:?}(
    JSON.parse(Deno[Deno.internal].core.ops.op_get_json_arg({spec_arg_id})),
).then((result) => {{
    svg = result;
}});
"#,
            ver_name = VlVersion::v5_11,
            spec_arg_id = spec_arg_id,
        ));
        self.worker.execute_script("<anon>", code)?;
        self.worker.run_event_loop(false).await?;

        let value = self.execute_script_to_string("svg").await?;
        Ok(value)
    }
}

pub enum VlConvertCommand {
    VlToSvg {
        vl_spec: serde_json::Value,
        responder: oneshot::Sender<Result<String, AnyError>>,
    },
}

#[derive(Clone)]
pub struct VlConverter {
    sender: Sender<VlConvertCommand>,
    _handle: Arc<JoinHandle<Result<(), AnyError>>>,
}

/// Wrapper that communicates with InnerVlConverter over an mpsc channel
/// Is Sendable
impl VlConverter {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::channel::<VlConvertCommand>(32);
        let handle = Arc::new(thread::spawn(move || {
            TOKIO_RUNTIME.block_on(async {
                let mut inner = InnerVlConverter::try_new().await.unwrap();

                while let Some(cmd) = receiver.next().await {
                    match cmd {
                        VlConvertCommand::VlToSvg { vl_spec, responder } => {
                            let svg_result = inner.vegalite_to_svg(vl_spec.clone()).await;
                            responder.send(svg_result).ok();
                        }
                    }
                }
            });
            Ok(())
        }));

        Self {
            sender,
            _handle: handle,
        }
    }

    pub async fn vegalite_to_svg(
        &mut self,
        vl_spec: serde_json::Value,
    ) -> Result<String, AnyError> {
        let (resp_tx, resp_rx) = oneshot::channel::<Result<String, AnyError>>();
        let cmd = VlConvertCommand::VlToSvg {
            vl_spec,
            responder: resp_tx,
        };

        // Send request
        match self.sender.send(cmd).await {
            Ok(_) => {
                // All good
            }
            Err(err) => {
                bail!("Failed to send SVG conversion request: {}", err.to_string())
            }
        }

        // Wait for result
        match resp_rx.await {
            Ok(svg_result) => svg_result,
            Err(err) => bail!("Failed to retrieve conversion result: {}", err.to_string()),
        }
    }
}
