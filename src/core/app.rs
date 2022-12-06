
use crate::app::yaml::parse_yaml_config;

#[path = "../utils/measureable.rs"]
mod measure;
#[path = "yaml.rs"]
mod yaml;
#[path = "../utils/spinner.rs"]
mod spinner;
#[path = "../utils/sanitize.rs"]
mod sanitize;
#[path = "../core/ssh.rs"]
mod ssh;
#[path = "installation.rs"]
mod install;
#[path = "../config/config.rs"]
mod installation;
#[path = "yaml.rs"]
mod config;

pub(crate) fn app( path: String) {
    let spinner_handle = spinner::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = parse_yaml_config(path);
    spinner_handle.done();

    for (i,masters) in parsed_yaml.masters.iter().enumerate() {
        let spinner_handle = spinner::spinner(format!("{}{}{}{}", "Connecting to server: ", masters.ip, " | Name: ", masters.name).parse().expect("spinner working"));
        let ssh_session = ssh::connect_server_via_ssh(&masters);
        spinner_handle.done();
        let ip = &masters.ip;

        for (i, instructions) in installation::get_installation().linux_amd64.iter().enumerate() {
            install::install(instructions, &ssh_session);
        }
        if i ==0 {
            install::get_kube_config_into_local(ip, &ssh_session);
            install::get_k3s_token_and_save( &ssh_session);
        }

    }
}

