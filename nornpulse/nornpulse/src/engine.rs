pub mod agents;
pub mod app;
pub mod creature_history;
pub mod directory_manager;
pub mod flight_recorder;
pub mod input_manager;


pub unsafe fn inject_calls() {
  flight_recorder::inject_calls();
  agents::inject_calls();
  app::inject_calls();
  creature_history::inject_calls();
}