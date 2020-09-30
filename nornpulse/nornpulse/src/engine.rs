pub mod agents;
pub mod app;
pub mod camera;
pub mod caos_description;
pub mod caos_machine;
pub mod caos_var;
pub mod classifier;
pub mod configurator;
pub mod creature_history;
pub mod directory_manager;
pub mod flight_recorder;
pub mod handler_function;
pub mod input_manager;
pub mod macro_script;
pub mod main_camera;
pub mod module_importer;
pub mod op_spec;
pub mod pray_manager;
pub mod shared_gallery;
pub mod world;

pub unsafe fn inject_calls() {
    flight_recorder::inject_calls();
    agents::inject_calls();
    app::inject_calls();
    creature_history::inject_calls();
    configurator::inject_calls();
}
