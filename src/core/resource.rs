trait Resource {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn dot_file_config_path(&self) -> String;
    fn check_installed(&self) -> Result<bool, &'static str>;
    fn run_install(&self) -> Result<bool, &'static str>;
}
