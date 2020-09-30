use crate::engine::caos_description::CAOSDescription;
use crate::engine::caos_machine::CAOSMachine;
use crate::engine::caos_var::CAOSVar;
use crate::engine::configurator::Configurator;
use crate::engine::directory_manager::DirectoryManager;
use crate::engine::input_manager::InputManager;
use crate::engine::main_camera::MainCamera;
use crate::engine::module_importer::ModuleImporter;
use crate::engine::pray_manager::PrayManager;
use crate::engine::shared_gallery::SharedGallery;
use crate::engine::world::World;
use crate::utils::cpp_adapter::{CppString, CppVector};
use callengine::{call_engine, CheckStructAlign};
pub use injected_calls::inject_calls;
use std::convert::TryInto;
use std::time::Instant;

mod injected_calls;

static mut C_CALLED: bool = false;
static mut WORLD_TICK_INTERVAL: u32 = 0x32;
static mut APP: std::mem::MaybeUninit<App> = std::mem::MaybeUninit::uninit();

static mut APP_INSTANT: std::mem::MaybeUninit<Instant> = std::mem::MaybeUninit::uninit();

#[repr(C, packed)]
#[derive(CheckStructAlign)]
pub struct App {
    #[check_align(0)]
    save_required: bool,

    _padding8: [u8; 3],

    #[check_align(4)]
    pending_loading_scene_name: CppString,

    #[check_align(20)]
    pub terminate_triggered: bool,

    #[check_align(21)]
    _unused_field_0: bool,

    _padding1: [u8; 2],

    #[check_align(24)]
    pub unknown_flag_storage: i32,

    #[check_align(28)]
    pub aa: CppVector<u8>,

    #[check_align(44)]
    pub bb: CppVector<u8>,

    #[check_align(60)]
    pub preview_window_handle: usize,

    #[check_align(64)]
    pub is_screen_saver_preview: bool,

    #[check_align(65)]
    pub screen_saver_config_flag: bool,

    #[check_align(66)]
    pub is_app_screensaver: bool,

    _padding2: [u8; 1],

    #[check_align(68)]
    pub user_settings: Configurator,

    _padding6: [u8; 28],

    #[check_align(108)]
    pub machine_settings: Configurator,

    _padding7: [u8; 28],

    #[check_align(148)]
    pub internal_window_has_resized_flag: bool,

    #[check_align(149)]
    pub internal_window_has_moved_flag: bool,

    #[check_align(150)]
    pub display_settings_error_next_tick: bool,

    #[check_align(151)]
    pub window_has_resized_flag: bool,

    #[check_align(152)]
    pub window_has_moved_flag: bool,

    #[check_align(153)]
    pub should_skeletons_animate_double_speed: bool,

    #[check_align(154)]
    pub whether_we_should_highlight_agents_known_to_creature: bool,

    _padding3: [u8; 1],

    #[check_align(156)]
    pub which_creaure_permission_to_highlight: i32,

    #[check_align(160)]
    pub line_plane: i32,

    #[check_align(164)]
    pub creature_pickup_status: i32,

    #[check_align(168)]
    pub only_play_midi_music_flag: bool,
    _padding4: [u8; 3],

    #[check_align(172)]
    pub h_cursor: usize,

    #[check_align(176)]
    pub handle: usize,

    #[check_align(180)]
    pub world: *mut World,

    #[check_align(184)]
    pub input_manager: InputManager,

    _unused_field_1: bool,
    _padding10: [u8; 3],

    #[check_align(252)]
    game_name: CppString,

    #[check_align(268)]
    system_tick: u32,

    #[check_align(272)]
    unknown_progress_bar: i32,

    #[check_align(276)]
    play_all_sounds_at_maximum_level_flag: bool,

    #[check_align(277)]
    autokill_agent_on_error_flag: bool,

    _padding9: [u8; 2],

    #[check_align(280)]
    elapsed_time_history: CppVector<u32>,

    #[check_align(296)]
    elapsed_time_history_index: usize,

    #[check_align(300)]
    password: CppString,

    #[check_align(316)]
    do_i_need_to_get_password: bool,

    #[check_align(317)]
    display_rendering: bool,

    #[check_align(318)]
    refresh_display_at_end_of_tick: bool,

    #[check_align(319)]
    pub fastest_ticks: bool,

    #[check_align(320)]
    maximum_distance_before_port_line_warns: f32,

    #[check_align(324)]
    maximum_distance_before_port_line_snaps: f32,

    #[check_align(328)]
    last_timestamp: u32,

    #[check_align(332)]
    last_tick_gap: u32,

    #[check_align(336)]
    eame_map: [u8; 12], // std::map<std::string, CAOSVAR>
}

impl App {
    pub fn add_basic_pray_directories(&mut self) {
        unsafe { _add_basic_pray_directories(self) }
    }

    // pub fn add_initalisation_function() -> undefined4 {
    // }

    pub fn new() -> Self {
        Self {
            save_required: false,
            _padding8: Default::default(),
            pending_loading_scene_name: CppString::empty(),
            terminate_triggered: false,
            _unused_field_0: false,
            _padding1: Default::default(),
            unknown_flag_storage: 0xf,
            aa: {
                let mut aa = CppVector::<u8>::empty();
                aa.push(0x1);
                aa.push(0x2);
                aa.push(0x4);
                aa.push(0x8);
                aa.push(0x10);
                aa.push(0x20);
                aa.push(0x40);
                aa
            },
            bb: {
                let mut bb = CppVector::<u8>::empty();
                bb.push(0x0);
                bb.push(0x1);
                bb.push(0x2);
                bb.push(0x8);
                bb.push(0x10);
                bb.push(0x20);
                bb
            },
            preview_window_handle: 0,
            is_screen_saver_preview: false,
            screen_saver_config_flag: false,
            is_app_screensaver: false,
            _padding2: Default::default(),

            user_settings: Configurator::new(),
            _padding6: Default::default(),
            machine_settings: Configurator::new(),
            _padding7: Default::default(),

            internal_window_has_resized_flag: false,
            internal_window_has_moved_flag: false,
            display_settings_error_next_tick: false,
            window_has_resized_flag: false,
            window_has_moved_flag: false,
            should_skeletons_animate_double_speed: false,
            whether_we_should_highlight_agents_known_to_creature: false,
            _padding3: Default::default(),
            which_creaure_permission_to_highlight: 0,
            line_plane: 0x270e,
            creature_pickup_status: 0,
            only_play_midi_music_flag: false,
            _padding4: Default::default(),
            h_cursor: 0,
            handle: 0,
            world: std::ptr::null_mut::<World>(),
            input_manager: InputManager::new(),
            _unused_field_1: false,
            _padding10: Default::default(),
            game_name: CppString::empty(),
            system_tick: 0,
            unknown_progress_bar: 0,
            play_all_sounds_at_maximum_level_flag: false,
            autokill_agent_on_error_flag: false,
            _padding9: Default::default(),
            elapsed_time_history: CppVector::empty(),
            elapsed_time_history_index: 0,
            password: CppString::empty(),
            do_i_need_to_get_password: false,
            display_rendering: true,
            refresh_display_at_end_of_tick: false,
            fastest_ticks: false,
            maximum_distance_before_port_line_warns: 600.0,
            maximum_distance_before_port_line_snaps: 800.0,
            last_timestamp: 0,
            last_tick_gap: 0xFFFFFFFF,
            eame_map: Default::default(),
        }
    }

    // pub fn auto_kill_agents_on_error(&self) -> bool {
    // }

    // pub fn begin_wait_cursor(&self) -> bool {
    // }

    #[call_engine(0x005575d0)]
    #[rustfmt::skip]
    unsafe fn call_initialization_functions(&mut self);

    // pub fn change_resolution(&mut self) {
    // }

    // pub fn check_all_free_disk_space(&mut self, i1: i32, i2: i32) -> bool {
    // }

    // pub fn check_for_cd(&self) -> bool {
    // }

    // pub fn check_for_mutex(&self) -> bool {
    // }

    // pub fn check_free_disk_space(&mut self, path: String, i1: i32) -> bool {
    // }

    // pub fn create_new_world(&mut self, name: String) {
    // }

    #[call_engine(0x0054ff50)]
    #[rustfmt::skip]
    unsafe fn create_progress_bar(&mut self) -> bool;

    // pub fn debug_key_now(&mut self) -> bool {
    //     false
    // }

    // pub fn debug_key_now_no_shift(&mut self) -> bool {
    //     false
    // }

    // pub fn delete_eame_var(&mut self, var_name: String) {
    // }

    // pub fn disable_main_view(&mut self) {
    // }

    // pub fn disable_map_image(&mut self) {
    // }

    // pub fn display_settings_error_next_tick(&mut self) {
    // }

    // pub fn do_i_need_to_get_password(&mut self) -> bool {
    // }

    fn do_load_world(&mut self, world: &str) {
        let s = CppString::from(world);
        unsafe {
            _do_load_world(self, &s);
        }
    }

    // pub fn do_refresh_from_game_variables(&mut self) {
    // }

    // pub fn do_you_only_play_midi_music(&self) -> bool {
    // }

    // pub fn enable_main_view(&mut self) {
    // }

    // pub fn enable_map_image(&mut self) {
    // }

    // pub fn end_progress_bar(&mut self) {
    // }

    // pub fn end_wait_cursor(&mut self) {
    // }

    // pub fn eor_wolf_values(&mut self, i1: i32, i2: i32) -> int {
    // }

    // pub fn generate_window_title(&mut self) -> String {
    // }

    // pub fn get_app_details(&mut self, d1: &String, d2: &String, d3: &String) -> bool {
    // }

    // pub fn get_creature_pickup_status(&mut self) -> int {
    // }

    // pub fn get_default_mng(&mut self) -> String {
    // }

    pub fn eame_var(&mut self, var_name: &str) -> &mut CAOSVar {
        let s = CppString::from(var_name);
        unsafe { &mut *_eame_var(self, &s) }
    }

    // pub fn get_fastest_ticks(&mut self) -> bool {
    // }

    // pub fn get_game_name(&mut self) -> String {
    // }

    // pub fn get_game_var(&mut self, var_name: String) -> CAOSVar {
    // }

    // pub fn get_initialisation_functions(var_name: String) -> vector<fn (&mut App)> {
    // }

    // pub fn get_input_manager(&mut self) -> &mut InputManager {
    // }

    // pub fn get_is_screen_saver_preview(&self) -> bool {
    // }

    fn _key_from_lang_cfg(&self, key: &str, default: &str) -> String {
        let language_config = Configurator::from("language.cfg");
        language_config
            .get(key)
            .or(self.user_settings.get(key))
            .map(|s| s.clone())
            .unwrap_or(String::from(default))
    }

    pub fn lang_catalogue(&self) -> String {
        self._key_from_lang_cfg("Language", "en")
    }

    pub fn lang_c_lib(&self) -> String {
        self._key_from_lang_cfg("LanguageCLibrary", "english")
    }

    // pub fn get_last_tick_gap(&self) -> int {
    // }

    // pub fn get_line_plane(&self) -> int {
    // }

    // pub fn get_maximum_distance_before_port_line_snaps(&self) -> int {
    // }

    // pub fn get_maximum_distance_before_port_line_warns(&self) -> float {
    // }

    // pub fn get_maximum_distance_before_port_line_warns(&self) -> float {
    // }

    // pub fn get_network_nickname(&self) -> String {
    // }

    // pub fn get_network_user_id(&self) -> String {
    // }

    // pub fn get_next_eame_var(&self, d: String) -> String {
    // }

    // pub fn get_password(&self) -> String {
    // }

    // pub fn get_preview_window_handle(&self) -> HWND__ {
    // }

    // pub fn get_screen_saver_config(&self) -> bool {
    // }

    // pub fn get_system_tick(&self) -> bool {
    // }

    pub fn get() -> &'static mut App {
        unsafe {
            if !C_CALLED {
                log::debug!("App Construction called");
                //APP = std::mem::MaybeUninit::new(App::new());
                APP_INSTANT = std::mem::MaybeUninit::new(Instant::now());

                _app_constructor(APP.as_mut_ptr());

                std::ptr::write(&mut (*APP.as_mut_ptr()).user_settings, Configurator::new());
                std::ptr::write(
                    &mut (*APP.as_mut_ptr()).machine_settings,
                    Configurator::new(),
                );
                C_CALLED = true;
            }

            &mut *APP.as_mut_ptr()
        }
    }

    // pub fn get_tick_rate_factor(&self) -> float {
    // }

    // pub fn get_warp_incoming_path(&self) -> String {
    // }

    // pub fn get_warp_outgoing_path(&self) -> String {
    // }

    // pub fn get_warp_outgoing_path(&self) -> String {
    // }

    // pub fn which_creature_permission_to_highlight(&self) -> int {
    // }

    // pub fn get_world(&self) -> Box<World> {
    // }

    // pub fn get_world_name(&self) -> String {
    // }

    pub fn world_tick_interval() -> u32 {
        unsafe { WORLD_TICK_INTERVAL }
    }

    #[call_engine(0x0054f970)]
    #[rustfmt::skip]
    pub unsafe fn handle_input(&mut self);

    pub fn init(&mut self) -> Result<(), String> {
        log::debug!("In App init");

        let dm = unsafe { DirectoryManager::get() };
        let shared_gallery_dir = unsafe { dm.directory(0xd) };

        let shared_gallery = SharedGallery::get();
        shared_gallery.set_creature_gallery_folder(shared_gallery_dir.as_str());
        unsafe { shared_gallery.clean_creature_gallery_folder() };

        log::debug!("Pretending to loading modules (no-op).");
        // A function like: ModuleImporter::load_modules()
        // would go here.

        log::debug!("Loading syntax tables");
        let caos_description = unsafe { CAOSDescription::get() };
        unsafe { caos_description.load_default_tables() };

        log::debug!("Pretending to execute netbabel module.");
        // An iterator through all modules, loading up their syntax would go
        // here.

        ModuleImporter::load_net_caos()?;

        log::debug!("Making syntax file for CAOS tool");

        let mut syntax_dir = unsafe { dm.directory(0x0) };
        syntax_dir += "caos.syntax";
        caos_description.save_syntax(syntax_dir.as_str())?;

        unsafe { CAOSMachine::initialize_handler_tables() };

        self.lang_catalogue();
        log::debug!("Flight recorder self reference ;-)");

        unsafe {
            self.eame_var("engine_nudge_border_t").set_integer(2);
            self.eame_var("engine_nudge_border_b").set_integer(2);
            self.eame_var("engine_nudge_border_l").set_integer(2);
            self.eame_var("engine_nudge_border_r").set_integer(2);
        }

        log::debug!("Setting up PRAY system.");
        {
            let lang = self.lang_catalogue();
            unsafe { PrayManager::get().set_language(&lang) };
        }
        self.add_basic_pray_directories();

        log::debug!("No need to seed random number generator, using rust rng.");

        log::debug!("Calling generic init functions.");

        unsafe { self.call_initialization_functions() };

        log::debug!("Setting up view");

        unsafe { self.set_up_main_view() };

        unsafe {
            let main_camera = MainCamera::get();
            main_camera.enable();
        }

        unsafe { self.create_progress_bar() };

        log::debug!("Loading startup world");

        self.do_load_world("Startup");

        unsafe {
            self.refresh_from_game_variables();
        }

        log::debug!("Setting up sound");

        unsafe { self.set_up_sound() };

        log::debug!("Reinitializing catalogue files");

        unsafe { self.init_localization() };

        Ok(())
    }

    pub fn init_config_files(&mut self) -> std::io::Result<()> {
        self.machine_settings.bind_to_file("machine.cfg")?;
        self.user_settings.bind_to_file("user.cfg")?;

        if let Some(game_name) = self.user_settings.get("Game Name") {
            let game_name_str = game_name.clone();
            self.set_game_name(game_name_str.as_str());
        }
        Ok(())
    }

    // pub fn init_local_catalogue_files_from_the_worlds_directory(&mut self) -> bool {
    // }

    // pub fn init_localisation(&mut self) -> bool {
    // }

    #[call_engine(0x0054e920)]
    #[rustfmt::skip]
    unsafe fn internal_window_has_moved(&mut self);

    #[call_engine(0x0054e8f0)]
    #[rustfmt::skip]
    unsafe fn internal_window_has_resized(&mut self);

    // pub fn is_app_a_screensaver(&self) -> bool {
    // }

    // pub fn is_app_full_screen(&self) -> bool {
    // }

    // pub fn machine_settings(&self) -> &Configurator {
    // }

    // pub fn notify_new_nickname(&self, nickname: String) {
    // }

    // pub fn play_all_sounds_at_maximum_level(&self, nickname: String) {
    // }

    #[call_engine(0x00550df0)]
    #[rustfmt::skip]
    pub unsafe fn refresh_from_game_variables(&mut self);

    pub fn set_game_name(&mut self, name: &str) {
        self.game_name = CppString::from(name);
    }

    // pub fn set_password(&mut self, name: String) {
    // }

    #[call_engine(0x0054e4d0)]
    #[rustfmt::skip]
    pub unsafe fn set_up_main_view(&mut self);

    #[call_engine(0x0054e930)]
    #[rustfmt::skip]
    pub unsafe fn set_up_sound(&mut self);

    // pub fn set_whether_we_should_highlight_agents_known_to_creature(&mut self, flag: bool) {
    // }

    // pub fn set_which_creature_permission_to_highlight(&mut self, permission: i32) {
    // }

    pub fn set_world_tick_interval(tick: u32) {
        unsafe {
            WORLD_TICK_INTERVAL = tick;
        }
    }

    // pub fn should_highlight_agents_known_to_creature(&self) -> bool {
    // }

    // pub fn should_skeletons_animate_double_speed(&self) -> bool {
    // }

    // pub fn set_should_skeletons_animate_double_speed(&self, flag: bool) {
    // }

    #[call_engine(0x0054e3d0)]
    #[rustfmt::skip]
    pub unsafe fn shut_down(&mut self);

    // pub fn specify_progress_intervals(&mut self, i1: i32) {
    // }

    // pub fn start_progress_bar(&mut self, i1: i32) {
    // }

    // pub fn toggle_full_screen_mode(&mut self) {
    // }

    // pub fn toggle_midi(&mut self) {
    // }

    pub fn update(&mut self) {
        {
            let duration = ticks().unwrap();
            self.last_tick_gap = duration.saturating_sub(self.last_timestamp);
            self.last_timestamp = duration;
        }

        unsafe {
            MainCamera::get().make_the_entity_handler_reset_bounds_properly();
        }

        if self.display_settings_error_next_tick {
            // On windows this does something quite complex that involves message boxes, on Linux,
            // the bool is reset and nothing else happening. If not worthwhile enough to port to
            // Linux we're not going to bother figuring it out what it is.
            self.display_settings_error_next_tick = false;
        }

        if self.internal_window_has_moved_flag {
            unsafe {
                self.internal_window_has_moved();
            }
            self.internal_window_has_moved_flag = false;
        }

        if self.internal_window_has_resized_flag {
            unsafe {
                self.internal_window_has_resized();
            }
            self.internal_window_has_resized_flag = false;
        }

        if self.window_has_moved_flag {
            self.window_has_moved_flag = false;
            self.internal_window_has_moved_flag = true;
            unsafe {
                self.internal_window_has_moved();
            }
        }

        if self.window_has_resized_flag {
            self.window_has_resized_flag = false;
            self.internal_window_has_resized_flag = true;
            unsafe {
                self.internal_window_has_resized();
            }
        }

        if self.save_required {
            unsafe {
                (*self.world).save();
            }
            self.save_required = false;
        }

        if self.terminate_triggered {
            log::debug!("Signalling termination.");
            unsafe {
                _quit_signalled();
            }
            self.terminate_triggered = false;
        }

        {
            let game_name = self.pending_loading_scene_name.to_string();
            if !game_name.is_empty() {
                self.do_load_world(&game_name);
                self.pending_loading_scene_name = CppString::empty();
            }
        }

        unsafe {
            self.handle_input();
        }
        self.system_tick += 1;

        // Module Interface would go here for calling world ticks.

        unsafe { (*self.world).task_switcher() };

        if self.display_rendering || self.refresh_display_at_end_of_tick {
            unsafe {
                MainCamera::get().render();
            }
            self.refresh_display_at_end_of_tick = false;
        }

        unsafe {
            let duration = ticks().unwrap();

            self.elapsed_time_history_index += 1;
            if 9 < self.elapsed_time_history_index {
                self.elapsed_time_history_index = 0;
            }

            let elapsed = duration.saturating_sub(self.last_timestamp);
            self.elapsed_time_history[self.elapsed_time_history_index] = elapsed;
        }
    }

    // pub fn update_progress_bar(&mut self, progress: i32) {
    // }

    // pub fn update_progress_bar(&mut self, progress: i32) {
    // }

    // pub fn user_settings(&self) -> &Configurator {
    // }

    // pub fn window_has_moved(&self) -> bool {
    // }

    pub fn process_command_line(&mut self, args: &str) {
        if args == "--autokill" {
            self.autokill_agent_on_error_flag = true;
        }
    }

    #[call_engine(0x0054f210)]
    #[rustfmt::skip]
    pub unsafe fn init_localization(&mut self) -> bool;

    #[call_engine(0x0054e8d0)]
    #[rustfmt::skip]
    pub unsafe fn window_has_moved(&mut self);

    #[call_engine(0x0054e8e0)]
    #[rustfmt::skip]
    pub unsafe fn window_has_resized(&mut self);

    #[call_engine(0x00557fa0)]
    #[rustfmt::skip]
    pub unsafe fn is_full_screen(&self) -> bool;

    #[call_engine(0x0054ec50)]
    #[rustfmt::skip]
    pub unsafe fn toggle_full_screen_mode(&mut self) -> bool;
}

/// Will fail after ~49 days of play due to integer overflow.
/// This is terrible for wolfing runs and we need out of the u32 sized
/// types as soon as humanly possible.
fn ticks() -> Option<u32> {
    unsafe {
        // TODO: Will be real swell to store the Instant objects directly in `App`.
        let duration = Instant::now().duration_since(*APP_INSTANT.as_ptr());
        duration.as_millis().try_into().ok()
    }
}

#[call_engine(0x00557280, "thiscall")]
#[rustfmt::skip]
unsafe fn _add_basic_pray_directories(app: *mut App);

#[call_engine(0x0054cc60, "thiscall")]
#[rustfmt::skip]
unsafe fn _app_constructor(app: *mut App);

#[call_engine(0x00550e10, "thiscall")]
#[rustfmt::skip]
unsafe fn _do_load_world(app: &mut App, world: *const CppString);

#[call_engine(0x00557c60, "thiscall")]
#[rustfmt::skip]
unsafe fn _eame_var(app: &mut App, world: &CppString) -> *mut CAOSVar;

#[call_engine(0x00478e80)]
#[rustfmt::skip]
unsafe fn _quit_signalled();
