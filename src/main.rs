use std::env;

fn main(){
	println!("{}",get_xdg_config());
}

fn get_xdg_config() -> String {
	match env::var("XDG_CACHE_HOME".to_string()){
		Ok(val) => val.to_string(),
		Err(e) => get_default_config(),
	}
}

fn get_default_config() -> String {
	match env::var("HOME".to_string()){
		Ok(val) => val.to_string()+"/.cache/clm",
		Err(e) => "Could not be retrieved".to_string(),
	}
}
