use crate::configs::configs::{ClientKey, CommonKey, ServerKey};
use derive_deftly::Deftly;
use pt_tracing::PtTracing;
use serde::{Deserialize, Serialize};

/// Main Config
/// But we are not going to serializing it
#[derive(Deftly, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigKey {
    /// Config which is needed in Server Side
    Server {
        /// Server config
        #[serde(flatten)]
        server_key: ServerKey,
        /// Common config
        #[serde(flatten)]
        common_key: CommonKey,
    },
    /// Config which is needed in Client Side
    Client {
        /// Client config
        #[serde(flatten)]
        client_key: ClientKey,
        /// Common config
        #[serde(flatten)]
        common_key: CommonKey,
    },
}

impl ConfigKey {
    /// initialize Config from env
    /// We panic fastly here
    pub fn init() -> Self {
        let common_key = match envy::from_env::<CommonKey>() {
            Err(x) => {
                PtTracing::env_error(&format!("Invalid or unset CommonKey {}", x.to_string()));
                panic!("Invalid or unset CommonKey")
            },
            Ok(common_key) => common_key,
        };
        let key = match (envy::from_env::<ServerKey>(), envy::from_env::<ClientKey>()) {
            (Err(_), Ok(client_key)) => ConfigKey::Client {
                client_key,
                common_key,
            },
            (Ok(server_key), Err(_)) => ConfigKey::Server {
                server_key,
                common_key,
            },
            (Err(x), Err(y)) => {
                PtTracing::env_error(&format!(
                    "Invalid or unset ServerKey and ClientKey 
                You must set one of them:  {} {}",
                    x.to_string(),
                    y.to_string()
                ));
                panic!("env error")
            },
            (Ok(_), Ok(_)) => {
                PtTracing::env_error("ServerKey and Client are both set. You may only set one");
                panic!("ServerKey and Client are both set You may only set one")
            },
        };
        key
    }
}

#[cfg(test)]
mod test {
    // @@ begin test lint list maintained by maint/add_warning @@
    #![allow(clippy::bool_assert_comparison)]
    #![allow(clippy::clone_on_copy)]
    #![allow(clippy::dbg_macro)]
    #![allow(clippy::mixed_attributes_style)]
    #![allow(clippy::print_stderr)]
    #![allow(clippy::print_stdout)]
    #![allow(clippy::single_char_pattern)]
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::unchecked_time_subtraction)]
    #![allow(clippy::useless_vec)]
    #![allow(clippy::needless_pass_by_value)]
    #![allow(clippy::string_slice)] // See arti#2571
    //! <!-- @@ end test lint list maintained by maint/add_warning @@ -->
    #![allow(unused)]
    use crate::configs::{
        configs::{ClientKey, CommonKey, ServerKey},
        core::ConfigKey,
    };
    use std::env;
    fn set_env() {
        unsafe {
            env::set_var("TOR_PT_MANAGED_TRANSPORT_VER", "1");
            env::set_var("TOR_PT_STATE_LOCATION", "/var/lib/tor/pt_state/");
            env::set_var("TOR_PT_EXIT_ON_STDIN_CLOSE", "1");
            env::set_var("TOR_PT_OUTBOUND_BIND_ADDRESS_V4", "203.0.113.4");
            env::set_var("TOR_PT_OUTBOUND_BIND_ADDRESS_V6", "2001:db8::4");

            env::set_var("TOR_PT_SERVER_TRANSPORTS", "obfs3,scramblesuit,automata");
            env::set_var(
                "TOR_PT_SERVER_BINDADDR",
                "obfs3-198.51.100.1:1984,scramblesuit-127.0.0.1:4891,automata-0.0.0.0:5555",
            );
            env::set_var(
                "TOR_PT_SERVER_TRANSPORT_OPTIONS",
                "scramblesuit:key=banana;automata:rule=110;automata:depth=3",
            );
            env::set_var("TOR_PT_ORPORT", "127.0.0.1:4200");
            env::set_var("TOR_PT_EXTENDED_SERVER_PORT", "127.0.0.1:4201");
            env::set_var(
                "TOR_PT_AUTH_COOKIE_FILE",
                "/var/lib/tor/extended_orport_auth_cookie",
            );
        }
    }
    /// Safety: We are not in a Async function so we have only 1 thread
    #[test]
    fn server_parse_test() -> anyhow::Result<()> {
        set_env();
        let config_key = ConfigKey::init();
        Ok(())
    }
}
