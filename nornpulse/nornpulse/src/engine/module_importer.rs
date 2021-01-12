use crate::engine::caos_description::CAOSDescription;
use crate::engine::caos_machine::CAOSMachine;
use crate::engine::handler_function::HandlerFunction;
use crate::engine::op_spec::OpSpec;
use crate::utils::cpp_adapter::CppString;
/// # ModuleImporter
///
/// tl;dr: `ModuleImporter` fakes the injection of networking functionality into
/// CAOS scripts so that they still work with networking removed.
///
/// `ModuleImporter` handles this and is executed at startup during App
/// initialization.
///
/// ## ModuleImporter role in c2e.
///
/// In c2e, `ModuleImporter` handled the dynamic loading of additional libraries
/// and then calling into them. Here, I will lay out how I believe it
/// worked, but please beware this is only estimated based on investigating a
/// compiled binary.
///
/// During `App` initialization, `ModuleImporter` scanned through the CWD
/// looking for any files that matched the following glob: `engine-*.dll`.
///
/// Upon finding the library, it was inspected for two functions:
///
/// * `ModuleInterface CreaturesEngineModuleInterface()`.
/// * `uint CreaturesEngineModuleInterfaceVersion()`.
///
/// The `ModuleInterface` that was extracted from the module was added to a list
/// of known module interfaces that could be manipulated or called upon at
/// run-time.
///
/// This is what the interface likely was:
///
/// ```
/// class ModuleInterface {
///     bool AddCAOSCommands();
///     bool Init();
///     bool IsNetworkInterface();
///     bool Persistent();
///     bool Read(CreatureArchive&);
///     void Twinned(AgentHandle, AgentHandle);
///     void Update(int);
///     void WorldEnding();
///     void WorldLoaded();
///     void Write(CreatureArchive&);
/// };
/// ```
/// As you can see, there are several points during game lifetime it can be
/// invoked.
///
/// ## netbabel
///
/// `engine-netbabel.dll` was the only library provided with Docking Station
/// which made use of this module loading system. It created a static
/// `NetworkImplementation` object which inherited from `ModuleInterface`.
///  
/// The purpose of `engine-netbabel` was to send PRAY data across the network,
/// enabling Docking Station's multiplayer functionality.
///
/// ## Role of `ModuleImporter` in nornpulse.
///
/// Hot-loading additional functionality is interesting future work, but
/// is beyond the functionality of nornpulse currently.
///
/// Further, it is irresponsible to put networking functionality
/// anywhere near nornpulse; it simply is not safe to do so until all the unsafe
/// code is ported and understood.
///
/// Thus, the purpose of `ModuleImporter` in nornpulse is to fake some of the
/// functionality provided by the old networking code.
/// Specifically, without `engine-netbabel all Docking
/// Station scrips (note: not Creatures 3 scripts) are going to error out
/// with a `CosInstaller` error like so:
///
/// > .\Bootstrap\010 Docking Station\DS creature history.cos
/// > 1 2 23 76 (Creature History)
/// > Expected a string at token 'net:'
/// > ... tu ov01 0 sets va06 {@}net: user doif va05 ne va06 an ...
/// >
/// > Windows NT/2000 6.2 build 9200 ()
/// > OriginalDisplay DirectX (no modules)
/// > *** *** ** **:**:** 2020 - *** - 2.286 b195
///
/// This is because `engine-netbabel.dll` injected the command `net: user` into CAOS
/// using the CAOS tables.
///
/// So `ModuleImporter` handles faking importing `engine-netbabel` which injects
/// dummy tokens into CAOS so as not stop upset old CAOS scripts expecting
/// networking (a.k.a all DockingStation scripts).
pub struct ModuleImporter {}
impl ModuleImporter {
    pub fn load_net_caos() -> Result<(), String> {
        let caos_description = unsafe { CAOSDescription::get_mut() };

        let net_str = "NET:";

        {
            let void_handler = HandlerFunction::from_call(no_op_void);
            let void_commands = make_net_void_commands();
            caos_description.push_table(void_handler, 0, net_str, &void_commands);
        }

        {
            let int_handler = HandlerFunction::from_get_int(no_op_int);
            let int_commands = make_net_int_commands();
            caos_description.push_table(int_handler, 1, net_str, &int_commands);
        }

        {
            let str_handler = HandlerFunction::from_get_string(no_op_string);
            let string_commands = make_net_string_commands();
            caos_description.push_table(str_handler, 4, &net_str, &string_commands);
        }

        Ok(())
    }
}

extern "C" fn no_op_void(machine: &mut CAOSMachine) {
    unsafe {
        consume_call(machine, &make_net_void_commands());
    }
}

extern "C" fn no_op_int(machine: &mut CAOSMachine) -> i32 {
    unsafe {
        consume_call(machine, &make_net_int_commands());
    }
    1
}

extern "C" fn no_op_string(machine: &mut CAOSMachine, s: *mut CppString) {
    unsafe {
        consume_call(machine, &make_net_string_commands());
        std::ptr::write(s, CppString::from("DEADBEEF"));
    }
}

pub unsafe fn consume_call(machine: &mut CAOSMachine, specs: &[OpSpec]) {
    let i = machine.ip();
    machine.set_ip(i + 2);
    let script = machine.script();
    let index_ptr: *const u16 = std::mem::transmute(script.op_spec_index + i);
    let index: usize = (*index_ptr) as usize;
    let spec = &specs[index as usize];
    let params = spec.parameters.to_string();
    skip_args(machine, &params);
}

fn skip_args(machine: &mut CAOSMachine, args: &str) {
    for c in args.chars() {
        match c {
            's' => {
                machine.fetch_string_rv();
            }
            'i' => {
                machine.fetch_integer_rv();
            }
            'f' => {
                machine.fetch_float_rv();
            }
            'v' => {
                machine.fetch_variable();
            }
            'm' => {
                machine.fetch_generic_rv();
            }
            _ => {
                log::error!("Unknown parameter type: {}. Skipping.", c);
            }
        }
    }
}

fn make_net_void_commands() -> Vec<OpSpec> {
    vec![
        OpSpec::new(
            "LINE",
            "i",
            "",
            "state",
            0x0000000F,
            "Goes on or offline, connecting or disconnecting from the Babel server.  \
            Set to 1 to connect, 0 to disconnect.  A @#NET: USER@ must be set first.  \
            @#NET: ERRA@ is set to any error code.  While the connection is being made, \
            this command can block for several ticks.  This command never runs in an @#INST@.",
        ),
        OpSpec::new(
            "WHON",
            "s",
            "",
            "user",
            0x0000000F,
            "Add a user to the whose wanted register for the target agent.  \
            Scripts @#User Online@ and @#User Offline@ are now called on this agent \
            when that user goes on or offline, or indeed when the local user goes offline.  \
            Use @#NET: WHOF@ to remove them from the register.  This command is blocking, \
            it can take several ticks to return.",
        ),
        OpSpec::new(
            "WHOF",
            "s",
            "",
            "user",
            0x0000000F,
            "Removes a user from the whose wanted list for the target agent.  See @#NET: WHON@.",
        ),
        OpSpec::new(
            "WHOD",
            "",
            "",
            "",
            0x0000000F,
            "Dump debugging information about the whose wanted register.",
        ),
        OpSpec::new(
            "WRIT",
            "ssimm",
            "",
            "user_id channel message_id param_1 param_2",
            0x0000000F,
            "Send a message to a remote machine, as specified by the user identifier.  \
            All agents which are @#NET: HEAR@ing on the given channel will receive the message, \
            and run the appropriate script.  If the specified user is offline, then the message \
            is discarded.  The @#FROM@ variable of the receiving script contains the user id of \
            the sender, as a string.  See also @#MESG WRIT@.",
        ),
        OpSpec::new(
            "HEAR",
            "s",
            "",
            "channel",
            0x0000000F,
            "The target agent will now accept CAOS messages over the network on the \
            specified channel, and execute their script as appropriate.  Use @#NET: WRIT@ \
            to send the message.",
        ),
        OpSpec::new(
            "HEAD",
            "",
            "",
            "",
            0x0000000F,
            "Dump debugging information about who is @#NET: HEAR@ing on what channels.",
        ),
        OpSpec::new(
            "PASS",
            "ss",
            "",
            "nick_name password",
            0x0000000F,
            "Set nickname and password - do this before connecting with @#NET: LINE@.  \
            If you set @#GAME@ \"@#engine_netbabel_save_passwords@\" to 1 then the password \
            for each nickname is saved in user.cfg, so you can specify an empty string for \
            the password after the first time.  The nickname is saved with the serialised world, \
            so is cleared when you start a new world.  You can use @#NET: PASS@ to retrieve the \
            user later.",
        ),
        OpSpec::new(
            "WHOZ",
            "",
            "",
            "",
            0x0000000F,
            "Zap the target agent's whose wanted register, removing all entries.",
        ),
        OpSpec::new(
            "RUSO",
            "v",
            "",
            "store_result",
            0x0000000F,
            "Returns (into store_result) a random user who is currently online.  \
            Returns an empty string if you're offline, or if you aren't using the Docking Station \
            Babel server module.  Since you're online, it can return yourself (especially if \
            you're the only person online!).  The user is also only likely to be online - they 
            could have gone offline since the server replied to you.<p>This is a command rather \
            than an integer r-value because it is blocking.  This means that it might take \
            several ticks before the server returns the result.  In this sense it is similar to a \
            command like @#OVER@, so it does not run in an @#INST@.  You should use @#LOCK@ if \
            you don't want your script interrupting.",
        ),
        OpSpec::new(
            "UNIK",
            "sv",
            "",
            "user_id store_result",
            0x0000000F,
            "Returns the specified user's screen or nick name.  Returns empty string if offline, \
             or no such user. This command can take many ticks to execute while the server is \
             quizzed, like @#NET: RUSO@.",
        ),
        OpSpec::new(
            "STAT",
            "vvvv",
            "",
            "time_online users_online bytes_received bytes_sent",
            0x0000000F,
            "Returns statistics for the current Babel connection, or -1 if offline.  \
            This command can block (doesn't execute in an @#INST@).  The statistics are: \
            <br>time_online - Time online in milliseconds\
            <br>users_online - Number of users currently connected to the server\
            <br>bytes_received - Bytes received by the client\
            <br>bytes_sent - Bytes sent from the client",
        ),
    ]
}

fn make_net_int_commands() -> Vec<OpSpec> {
    vec![
        OpSpec::new(
            "LINE",
            "",
            "",
            "",
            0x0000000F,
            "Returns 1 if you are connected to the Babel server, or 0 if you aren't.",
        ),
        OpSpec::new(
            "EXPO",
            "ss",
            "",
            "chunk_type dest_user_id",
            0x0000000F,
            "Transwarp the target creature to the given user.  The Creature is exported to the \
            warp out directory; this command is very similar to @#PRAY EXPO@. Return value is one \
            of the following:<br>0 for success<br>1 if the creature, or if pregnant any of its \
            offspring, are already on disk in some form.  This case won't happen much, if you use \
            a special chunk name like WARP.<br>2 if the user hasn't been online in this world yet \
            / since the user name changed, so we don't know who they are.<p>When receiving a \
            creature, use @#NET: FROM@ to find out who sent it.",
        ),
        OpSpec::new(
            "ULIN",
            "s",
            "",
            "user_id",
            0x0000000F,
            "Returns 1 if the specified user is online, or 0 if they are offline. This is slow \
            (i.e. has to call the server) unless the user is in the whose wanted register of any \
            agent. Use @#NET: WHON@ to add a user to the register.",
        ),
        OpSpec::new(
            "MAKE",
            "issv",
            "",
            "which_journal_spot journal_name user report_destination",
            0x00000011,
            "Like @#PRAY MAKE@, only sends the made pray file to the specified user. This will \
            arrive in their inbox, where it can be read with normal PRAY commands and deleted \
            with @#PRAY KILL@.",
        ),
        OpSpec::new(
            "ERRA",
            "",
            "",
            "",
            0x0000000F,
            "Returns an error code from the last command.  Currently @#NET: LINE@ is the only \
            command to set it.<p>Error codes are:<br>0 - Unknown<br>1 - Connection OK<br>2 - \
            Connection failed, you or the server are offline<br>3 - Connection failed, invalid \
            user name/password<br>4 - Connection failed, you are already logged in elsewhere<br>5 \
            - Connection failed, too many users for server<br>6 - Connection failed, internal \
            error<br>7 - Connection failed, new client version required.<p>Try @#NET: RAWE@ for \
            more detailed diagnostic codes.",
        ),
        OpSpec::new(
            "RAWE",
            "",
            "",
            "",
            0x0000000F,
            "Returns an internal error code from Babel.  Only use this for display and diagnostic \
            purpose, use @#NET: ERRA@ for documented error codes which you can rely on.",
        ),
    ]
}

fn make_net_string_commands() -> Vec<OpSpec> {
    vec![
        OpSpec::new(
            "USER",
            "",
            "",
            "",
            0x0000000F,
            "Returns the user's numeric Babel id, or an empty string if they have never logged in \
            with this world since they last changed user name.",
        ),
        OpSpec::new(
            "FROM",
            "s",
            "",
            "resource_name",
            0x0000000F,
            "The user who sent the PRAY file which contains the specified resource.  If the \
            resource did not arrive as a message over the network via @#NET: MAKE@ or \
            @#NET: EXPO@, then this returns an empty string.  The user returned by this command \
            is guaranteed in a way that looking at the content of the PRAY file would not be.  \
            For example, the \"Last Network User\" attribute in an exported Creature made with \
            @#NET: EXPO@ could be faked.",
        ),
        OpSpec::new(
            "HOST",
            "",
            "",
            "",
            0x0000000F,
            "Returns the hostname, port, id and friendly name on that host that we are currently \
            connected to, or empty string if offline.  The fields are space separated, although \
            the last field (friendly name) may contain spaces.",
        ),
        OpSpec::new(
            "PASS",
            "",
            "",
            "",
            0x0000000F,
            "Returns the currently set username, as selected with @#PASS@.",
        ),
        OpSpec::new(
            "WHAT",
            "",
            "",
            "",
            0x0000000F,
            "For debugging only.  Returns a string describing what the upload/query network \
            thread is currently doing.  For example, it may be fetching a random online user, or \
            uploading some creature history.  Returns an empty string if it is doing nothing.",
        ),
    ]
}
