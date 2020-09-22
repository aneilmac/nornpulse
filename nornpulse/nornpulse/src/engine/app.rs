use crate::engine::configurator::Configurator;
use crate::engine::input_manager::InputManager;
use crate::utils::cpp_adapter::{CppString, CppVector};
use callengine::{call_engine, CheckStructAlign};
use std::ffi::CStr;

mod injected_calls;

static mut C_CALLED: bool = false;
static mut WORLD_TICK_INTERVAL: i32 = 0x32;
static mut APP: std::mem::MaybeUninit<App> = std::mem::MaybeUninit::uninit();

#[repr(C, packed)]
#[derive(CheckStructAlign, Debug)]
pub struct App {
    _unknown1: [u8; 4],

    #[check_align(4)]
    current_loading_scene_name: CppString,

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

    // #[check_align(68)] pub user_settings_a: [u8;20],
    // pub user_settings_b: [u8;20],
    // #[check_align(108)] pub machine_settings_a: [u8;20],
    // pub machine_settings_b: [u8;20],
    
    #[check_align(148)]
    pub internal_window_has_resized: bool,

    #[check_align(149)]
    pub internal_window_has_moved: bool,

    #[check_align(150)]
    pub display_settings_error_next_tick: bool,

    #[check_align(151)]
    pub window_has_resized: bool,

    #[check_align(152)]
    pub window_has_moved: bool,

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
    pub world: usize,

    #[check_align(184)]
    pub input_manager: InputManager,

    _unknown3: [u8; 4],

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

    _unknown6: [u8; 22],

    #[check_align(300)]
    password: CppString,

    #[check_align(316)]
    do_i_need_to_get_password: bool,

    #[check_align(317)]
    display_rendering: bool,

    #[check_align(318)]
    refresh_display_at_end_of_tick: bool,

    #[check_align(319)]
    fastest_ticks: bool,

    #[check_align(320)]
    maximum_distance_before_port_line_warns: f32,

    #[check_align(324)]
    maximum_distance_before_port_line_snaps: f32,

    _unknown8: [u8; 4],

    #[check_align(332)]
    last_tick_gap: i32,

    _padding5: [u8; 12],
}

impl App {
    // pub fn add_basic_pray_directories() {
    // }

    // pub fn add_initalisation_function() -> undefined4 {
    // }

    pub fn new() -> Self {
        Self {
            _unknown1: Default::default(),
            current_loading_scene_name: CppString::empty(),
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

            // user_settings_a: Default::default(),
            // user_settings_b: Default::default(),
            // machine_settings_a: Default::default(),
            // machine_settings_b: Default::default(),


            internal_window_has_resized: false,
            internal_window_has_moved: false,
            display_settings_error_next_tick: false,
            window_has_resized: false,
            window_has_moved: false,
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
            world: 0,
            input_manager: InputManager::new(),
            _unknown3: Default::default(),
            game_name: CppString::empty(),
            system_tick: 0,
            unknown_progress_bar: 0,
            play_all_sounds_at_maximum_level_flag: false,
            autokill_agent_on_error_flag: false,
            _unknown6: Default::default(),
            password: CppString::empty(),
            do_i_need_to_get_password: false,
            display_rendering: true,
            refresh_display_at_end_of_tick: false,
            fastest_ticks: false,
            maximum_distance_before_port_line_warns: 600.0,
            maximum_distance_before_port_line_snaps: 800.0,
            _unknown8: Default::default(),
            last_tick_gap: -1,
            _padding5: Default::default(),
        }
    }

    // pub fn auto_kill_agents_on_error(&self) -> bool {
    // }

    // pub fn begin_wait_cursor(&self) -> bool {
    // }

    // pub fn call_initialization_functions(&mut self) {
    // }

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

    // pub fn create_progress_bar(&mut self) -> bool {
    // }

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

    // pub fn do_load_world(&mut self, world: String) {
    // }

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

    // pub fn get_eame_var(&mut self, var_name: String) -> CAOSVar {
    // }

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
        let  language_config = Configurator::from("language.cfg");
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
                
                app_constructor(APP.as_mut_ptr());

                std::ptr::write(&mut (*APP.as_mut_ptr()).user_settings, Configurator::new());
                std::ptr::write(&mut (*APP.as_mut_ptr()).machine_settings, Configurator::new());

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

    pub fn world_tick_interval() -> i32 {
        unsafe { WORLD_TICK_INTERVAL }
    }

    // pub fn handle_input(&mut self) {
    // }

    // pub fn init(&mut self) -> bool {
    // }

    // pub fn init_config_files(&mut self) -> bool {
    // }

    // pub fn init_local_catalogue_files_from_the_worlds_directory(&mut self) -> bool {
    // }

    // pub fn init_localisation(&mut self) -> bool {
    // }

    // pub fn _internal_window_has_moved(&mut self) {
    // }

    // pub fn _internal_window_has_resized(&mut self) {
    // }

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

    // pub fn refresh_from_game_variables(&mut self) {
    // }

    // pub fn set_game_name(&mut self, name: String) {
    // }

    // pub fn set_password(&mut self, name: String) {
    // }

    // pub fn set_up_main_view(&mut self) {
    // }

    // pub fn set_up_sound(&mut self) {
    // }

    // pub fn set_whether_we_should_highlight_agents_known_to_creature(&mut self, flag: bool) {
    // }

    // pub fn set_which_creature_permission_to_highlight(&mut self, permission: i32) {
    // }

    pub fn set_world_tick_interval(tick: i32) {
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

    // pub fn shut_down(&mut self) {
    // }

    // pub fn specify_progress_intervals(&mut self, i1: i32) {
    // }

    // pub fn start_progress_bar(&mut self, i1: i32) {
    // }

    // pub fn toggle_full_screen_mode(&mut self) {
    // }

    // pub fn toggle_midi(&mut self) {
    // }

    // pub fn update_app(&mut self) {
    // }

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

    #[call_engine(0x0054e000)]
    #[rustfmt::skip]
    pub unsafe fn update(&mut self);

    #[call_engine(0x05578b0)]
    #[rustfmt::skip]
    pub unsafe fn init_config_files(&mut self) -> bool;

    #[call_engine(0x0054f210)]
    #[rustfmt::skip]
    pub unsafe fn init_localization(&mut self) -> bool;

    #[call_engine(0x0041d270)]
    #[rustfmt::skip]
    pub unsafe fn get_input_manager(&self) -> *mut InputManager;

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

pub unsafe fn inject_calls() {
    injected_calls::inject_calls()
}

#[call_engine(0x0054cc60, "thiscall")]
unsafe fn app_constructor(app: *mut App);
