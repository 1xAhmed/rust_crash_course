// specifying the absolute path of something at every call site could be really painful especially if the path is really long, which is where the use statement comes in use. Brings an item from some path into some scope

// project_name::function_name      ==> project name is not a directory name, get the project name in cargo.toml as `name = "module_system"`
use module_system::greet;

fn main() {
    greet();
}
