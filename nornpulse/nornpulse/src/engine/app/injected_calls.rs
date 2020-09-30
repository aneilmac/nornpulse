use super::*;
use crate::utils::cpp_adapter::CppString;

pub unsafe fn inject_calls() {
    // replace_call!(0x00557280, add_basic_pray_directories);
    replace_call!(0x005573a0, add_initalisation_function);
    replace_call!(0x0041d5a0, auto_kill_agents_on_error);
    // replace_call!(0x0054f930, begin_wait_cursor);
    replace_call!(0x005575d0, call_initialization_functions);
    // replace_call!(0x0054ec00, change_resolution);
    replace_call!(0x00556430, check_all_free_disk_space);
    replace_call!(0x00556030, check_for_cd);
    replace_call!(0x00556300, check_for_mutex);
    replace_call!(0x005564c0, check_free_disk_space);
    // replace_call!(0x0054f850, create_new_world);
    // replace_call!(0x0054ff50, create_progress_bar);
    // replace_call!(0x00556000, debug_key_now);
    replace_call!(0x00555ef0, debug_key_now_no_shift);
    // replace_call!(0x00557dc0, delete_eame_var);
    replace_call!(0x0054e8a0, disable_main_view);
    replace_call!(0x0054e8c0, disable_map_image);
    replace_call!(0x0041d1b0, display_settings_error_next_tick);
    replace_call!(0x00550de0, do_i_need_to_get_password);
    // replace_call!(0x00550e10, do_load_world);
    // replace_call!(0x0054fb20, do_refresh_from_game_variables);
    replace_call!(0x0041d530, do_you_only_play_midi_music);
    replace_call!(0x0054e890, enable_main_view);
    replace_call!(0x0054e8b0, enable_map_image);
    // replace_call!(0x00550ae0, end_progress_bar);
    // replace_call!(0x0054f960, end_wait_cursor);
    // replace_call!(0x00555e50, eor_wolf_values);
    // replace_call!(0x005566a0, generate_window_title);
    // replace_call!(0x00551990, get_app_details);
    replace_call!(0x0041d1d0, get_creature_pickup_status);
    // replace_call!(0x00557e50, get_default_mng);
    // replace_call!(0x00557c60, get_eame_var);
    replace_call!(0x0041d580, get_fastest_ticks);
    // replace_call!(0x0041d280, get_game_name);
    // replace_call!(0x00557e30, get_game_var);
    replace_call!(0x00557560, get_initialisation_functions);
    replace_call!(0x0041d270, get_input_manager);
    replace_call!(0x00557c40, get_is_screen_saver_preview);
    replace_call!(0x0054ef50, get_lang_catalogue);
    replace_call!(0x0054ec90, get_lang_c_lib);
    replace_call!(0x0041d590, get_last_tick_gap);
    replace_call!(0x0041d1c0, get_line_plane);
    replace_call!(0x0041d250, get_maximum_distance_before_port_line_snaps);
    replace_call!(0x0041d240, get_maximum_distance_before_port_line_warns);
    // replace_call!(0x005576f0, get_network_nickname);
    // replace_call!(0x00557600, get_network_user_id);
    // replace_call!(0x00557ca0, get_next_eame_var);
    replace_call!(0x00550c90, get_password);
    // replace_call!(0x00557c50, get_preview_window_handle);
    // replace_call!(0x00557c30, get_screen_saver_config);
    replace_call!(0x0041d570, get_system_tick);
    replace_call!(0x0054cc50, get_the_app);
    replace_call!(0x0054ff00, get_tick_rate_factor);
    // replace_call!(0x00556ee0, get_warp_incoming_path);
    // replace_call!(0x00556c50, get_warp_outgoing_path);
    replace_call!(0x0041d230, which_creature_permission_to_highlight);
    replace_call!(0x0041d260, get_world);
    // replace_call!(0x005577e0, get_world_name);
    replace_call!(0x00557c00, get_world_tick_interval);
    // replace_call!(0x0054f970, handle_input);
    replace_call!(0x0054d210, init);
    replace_call!(0x005578b0, init_config_files);
    // replace_call!(0x0054f670, init_local_catalogue_files_from_the_worlds_directory);
    // replace_call!(0x0054f210, init_localisation);
    // replace_call!(0x0054e920, _internal_window_has_moved);
    // replace_call!(0x0054e8f0, _internal_window_has_resized);
    replace_call!(0x00557c20, is_app_a_screensaver);
    // replace_call!(0x00557fa0, is_app_full_screen);
    replace_call!(0x0041d5c0, machine_settings);
    // replace_call!(0x00557170, notify_new_nickname);
    replace_call!(0x0041d540, play_all_sounds_at_maximum_level);
    replace_call!(0x005511f0, process_command_line);
    // replace_call!(0x00550df0, refresh_from_game_variables);
    replace_call!(0x0041d3d0, set_game_name);
    replace_call!(0x00550b00, set_password);
    // replace_call!(0x0054e4d0, set_up_main_view);
    // replace_call!(0x0054e930, set_up_sound);
    replace_call!(0x0041d210, set_whether_we_should_highlight_agents_known_to_creature);
    replace_call!(0x0041d220, set_which_creature_permission_to_highlight);
    replace_call!(0x00557c10, set_world_tick_interval);
    // replace_call!(0x0041d200, should_highlight_agents_known_to_creature);
    replace_call!(0x0041d1e0, should_skeletons_animate_double_speed);
    replace_call!(0x0041d1f0, set_should_skeletons_animate_double_speed);
    // replace_call!(0x0054e3d0, shut_down);
    // replace_call!(0x00550a60, specify_progress_intervals);
    // replace_call!(0x005502c0, start_progress_bar);
    // replace_call!(0x0054ec50, toggle_full_screen_mode);
    replace_call!(0x0041d550, toggle_midi);
    // replace_call!(0x0054e000, update_app);
    // replace_call!(0x00550a90, update_progress_bar);
    replace_call!(0x0041d5b0, user_settings);
    replace_call!(0x0054e8d0, window_has_moved);
    replace_call!(0x0054e8e0, window_has_resized);
}

// extern "thiscall" fn add_basic_pray_directories(app: &mut App) {
//     app.add_basic_pray_directories()
// }

extern "cdecl" fn add_initalisation_function(_f: extern fn (&mut App)) {
    // No-op
}

extern "thiscall" fn auto_kill_agents_on_error(app: &App) -> bool {
    app.autokill_agent_on_error_flag
}

// extern "thiscall" fn begin_wait_cursor(app: &mut App) {
//     app.begin_wait_cursor()
// }

extern "thiscall" fn call_initialization_functions(app: &mut App) {
    // no-op
}

// extern "thiscall" fn change_resolution(app: &mut App) {
//   app.change_resolution()
// }

extern "thiscall" fn check_all_free_disk_space(app: &mut App, _i1: i32, _i2: i32) -> bool {
    true
}

extern "thiscall" fn check_for_cd(_app: &App) -> bool {
    true // Always assume the CD exists.
}

extern "thiscall" fn check_for_mutex(_app: &App) -> bool {
    true
}

extern "thiscall" fn check_free_disk_space(_app: &mut App, _path: CppString, _i1: i32) -> bool {
    true
}

// extern "thiscall" fn create_new_world(app: &mut App, name: CppString) {
// }

// extern "thiscall" fn create_progress_bar(app: &mut App) -> bool {
// }

// extern "thiscall" fn debug_key_now(app: &mut App) -> bool {

// }

extern "thiscall" fn debug_key_now_no_shift(app: &App) -> bool {
    app.debug_key_now_no_shift()
}

// extern "thiscall" fn delete_eame_var(app: &mut App, var_name: CppString) {
// }

extern "thiscall" fn disable_main_view(app: &mut App) {
    app.disable_main_view();
}

extern "thiscall" fn disable_map_image(app: &mut App) {
    app.disable_map_image();
}

extern "thiscall" fn display_settings_error_next_tick(app: &mut App) {
    app.display_settings_error_next_tick = true;
}

extern "thiscall" fn do_i_need_to_get_password(_app: &mut App) -> bool {
    false //no-op. This strange function would set flg to false then return the previous value.
}

// extern "thiscall" fn do_load_world(app: &mut App, world: CppString) {
// }

// extern "thiscall" fn do_refresh_from_game_variables(app: &mut App) {
// }

extern "thiscall" fn do_you_only_play_midi_music(app: &App) -> bool {
    app.only_play_midi_music_flag
}

extern "thiscall" fn enable_main_view(app: &mut App) {
    app.enable_main_view()
}

extern "thiscall" fn enable_map_image(app: &mut App) {
    app.enable_map_image()
}

// extern "thiscall" fn end_progress_bar(app: &mut App) {
// }

// extern "thiscall" fn end_wait_cursor(app: &mut App) {
// }

// extern "thiscall" fn eor_wolf_values(app: &mut App, i1: i32, i2: i32) -> int {
// }

// extern "thiscall" fn generate_window_title(app: &mut App) -> String {
// }

// extern "thiscall" fn get_app_details(app: &mut App, d1: &CppString, d2: &CppString, d3: &CppString) -> bool {
// }

extern "thiscall" fn get_creature_pickup_status(app: &mut App) -> i32 {
    app.creature_pickup_status
}

// extern "thiscall" fn get_default_mng(app: &mut App) -> String {
// }

// extern "thiscall" fn get_eame_var(app: &mut App, var_name: CppString) -> CAOSVar {
// }

extern "thiscall" fn get_fastest_ticks(app: &mut App) -> bool {
    app.fastest_ticks
}

// extern "thiscall" fn get_game_name(app: &mut App) -> String {
// }

// extern "thiscall" fn get_game_var(app: &mut App, var_name: CppString) -> CAOSVar {
// }

extern "cdecl" fn get_initialisation_functions() -> *mut CppVector<extern fn (&mut App)> {
    std::ptr::null_mut()
}

extern "thiscall" fn get_input_manager(app: &mut App) -> &mut InputManager {
    &mut app.input_manager
}

extern "thiscall" fn get_is_screen_saver_preview(_app: &App) -> bool {
    false
}

extern "thiscall" fn get_lang_catalogue(app: &mut App, out: *mut CppString) -> *mut CppString {
    let s = app.lang_catalogue();
    let s = CppString::from(s);
    unsafe {
        std::ptr::write(out, s);
    }
    out
}

extern "thiscall" fn get_lang_c_lib(app: &mut App, out: *mut CppString) -> *mut CppString {
    let s = app.lang_c_lib();
    let s = CppString::from(s);
    unsafe {
        std::ptr::write(out, s);
    }
    out
}

extern "thiscall" fn get_last_tick_gap(app: &App) -> u32 {
    app.last_tick_gap
}

extern "thiscall" fn get_line_plane(app: &App) -> i32 {
    app.line_plane
}

extern "thiscall" fn get_maximum_distance_before_port_line_snaps(app: &App) -> f32 {
    app.maximum_distance_before_port_line_snaps
}

extern "thiscall" fn get_maximum_distance_before_port_line_warns(app: &App) -> f32 {
    app.maximum_distance_before_port_line_warns
}

// extern "thiscall" fn get_network_nickname(app: &App) -> String {
// }

// extern "thiscall" fn get_network_user_id(app: &App) -> String {
// }

// extern "thiscall" fn get_next_eame_var(app: &App, d: String) -> String {
// }

extern "thiscall" fn get_password(app: &App, storage_ptr: *mut CppString) -> *mut CppString {
    unsafe { std::ptr::write(storage_ptr, app.password.clone()); }
    storage_ptr
}

// extern "thiscall" fn get_preview_window_handle(app: &App) -> HWND__ {
// }

// extern "thiscall" fn get_screen_saver_config(app: &App) -> bool {
// }

extern "thiscall" fn get_system_tick(app: &App) -> u32 {
    app.system_tick
}

extern "cdecl" fn get_the_app() -> &'static mut App {
    App::get()
}

extern "thiscall" fn get_tick_rate_factor(app: &App) -> f32 {
    app.tick_rate_factor()
}

// extern "thiscall" fn get_warp_incoming_path(app: &App) -> String {
// }

// extern "thiscall" fn get_warp_outgoing_path(app: &App) -> String {
// }

extern "thiscall" fn which_creature_permission_to_highlight(app: &App) -> i32 {
    app.which_creature_permission_to_highlight
}

extern "thiscall" fn get_world(app: &App) -> *mut World {
    app.world
}

// extern "thiscall" fn get_world_name(app: &App) -> String {
// }

extern "cdecl" fn get_world_tick_interval() -> u32 {
    App::world_tick_interval()
}

// extern "thiscall" fn handle_input(app: &mut App) {
// }

extern "thiscall" fn init(_app: &mut App) -> bool {
    true // no-op
}

extern "thiscall" fn init_config_files(app: &mut App) -> bool {
    true // no-op
}

// extern "thiscall" fn init_local_catalogue_files_from_the_worlds_directory(app: &mut App) -> bool {
// }

// extern "thiscall" fn init_localisation(app: &mut App) -> bool {
// }

// extern "thiscall" fn _internal_window_has_moved(app: &mut App) {
// }

// extern "thiscall" fn _internal_window_has_resized(app: &mut App) {
// }

extern "thiscall" fn is_app_a_screensaver(app: &App) -> bool {
    app.is_app_screensaver
}

// extern "thiscall" fn is_app_full_screen(app: &App) -> bool {
// }

extern "thiscall" fn machine_settings(app: &mut App) -> &mut Configurator {
    &mut app.machine_settings
}

// extern "thiscall" fn notify_new_nickname(app: &App, nickname: CppString) {
// }

extern "thiscall" fn play_all_sounds_at_maximum_level(app: &App) -> bool {
    app.play_all_sounds_at_maximum_level_flag
}

extern "thiscall" fn process_command_line(_app: &mut App, _args: CppString) {
    // no-op
}

// extern "thiscall" fn refresh_from_game_variables(app: &mut App) {
// }

extern "thiscall" fn set_game_name(app: &mut App, name: &CppString) {
    app.game_name = name.clone();
}

extern "thiscall" fn set_password(app: &mut App, password: &CppString) {
    app.password = password.clone();
}

// extern "thiscall" fn set_up_main_view(app: &mut App) {
// }

// extern "thiscall" fn set_up_sound(app: &mut App) {
// }

extern "thiscall" fn set_whether_we_should_highlight_agents_known_to_creature(
    app: &mut App, 
    flag: bool
) {
    app.whether_we_should_highlight_agents_known_to_creature = flag;
}

extern "thiscall" fn set_which_creature_permission_to_highlight(app: &mut App, permission: i32) {
    app.which_creature_permission_to_highlight = permission;
}

extern "cdecl" fn set_world_tick_interval(tick: u32) {
    App::set_world_tick_interval(tick)
}

// extern "thiscall" fn should_highlight_agents_known_to_creature(app: &App) -> bool {
// }

extern "thiscall" fn should_skeletons_animate_double_speed(app: &App) -> bool {
    app.should_skeletons_animate_double_speed
}

extern "thiscall" fn set_should_skeletons_animate_double_speed(app: &mut App, flag: bool) {
    app.should_skeletons_animate_double_speed = flag;
}

// extern "thiscall" fn shut_down(app: &mut App) {
// }

// extern "thiscall" fn specify_progress_intervals(app: &mut App, i1: i32) {
// }

// extern "thiscall" fn start_progress_bar(app: &mut App, i1: i32) {
// }

// extern "thiscall" fn toggle_full_screen_mode(app: &mut App) {
// }

extern "thiscall" fn toggle_midi(app: &mut App) {
    app.toggle_midi()
}

// extern "thiscall" fn update_app(app: &mut App) {
// }

// extern "thiscall" fn update_progress_bar(app: &mut App, progress: i32) {
// }

extern "thiscall" fn user_settings(app: &mut App) -> &mut Configurator {
    &mut app.user_settings
}

extern "thiscall" fn window_has_moved(app: &mut App) {
    app.window_has_moved_flag = true;
}

extern "thiscall" fn window_has_resized(app: &mut App) {
    app.window_has_resized_flag = true;
}
