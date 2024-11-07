use std::{env};
use std::fs::{File, OpenOptions};
use std::io::{stdout, IsTerminal, Read, Write};
use std::path::Path;
use std::process::Command;
use webbrowser;

fn main2() {
    
    let args : Vec<String> = env::args().collect();

    println!("Pointing to env number: {:?} cluster {:?}", args[1], args[2]);

    let cluster_number = &args[1];
    let env_number = &args[2];

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("server/config.js")
        .expect("Could not open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read the file");

    let url = "\'amd-apigw-stack-service-cmm-il".to_owned() + cluster_number + "-env" + env_number +
        "-runtime.apps.ildelocpcmm4" + cluster_number + ".ocpd.corp.amdocs.com\',";
    // hostname: 'amd-apigw-stack-service-cmm-il18-env236-runtime.apps.ildelocpcmm418.ocpd.corp.amdocs.com',

    println!("{}", url);

    let mut new_content = String::new();
    
    for line in contents.lines(){
        if !line.contains("amd-apigw"){
            new_content.push_str(&line);
            new_content.push_str("\n");

        }else if line.contains("amd-apigw"){
            let hostname = "        hostname: ";
            new_content.push_str(&*(hostname.to_owned() + url.as_str()));
            new_content.push_str("\n");
        }
    }
    
    let mut new_file = File::create(Path::new("server/config.js")).unwrap();
    new_file.write_all((&new_content).as_ref()).unwrap();


    run_npm();

    // run_node_config();

    // change_package_omni();

    // npm_start();
}
fn main(){
    check_node_version();
}


fn run_npm(){
    println!("Running npm installing node dependencies...");

    let output = Command::new("powershell")
        .arg("-Command")
        .arg("npm i; npm run sso:restore ; npm run sso:replace ")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let new_dir = String::from_utf8_lossy(&output.stdout);
        println!("{}", new_dir);
        run_node_config();
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn run_node_config(){
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Copy-Item -Path ./replace/NormalModuleFactory.js -Destination ./node_modules/webpack/lib/NormalModuleFactory.js; \
            Copy-Item -Path ./replace/index.js -Destination ./node_modules/omni-sdk/src/service/index.js;\
            New-Item -ItemType Directory -Path ./node_modules/omni-sdk/src/service/search-customer -Force;\
            Copy-Item -Path ./replace/searchCustomer.js -Destination ./node_modules/omni-sdk/src/service/search-customer/searchCustomer.js;\
            Copy-Item -Path ./replace/GeneralActions.js -Destination ./node_modules/digital-search-customer/src/actions/GeneralActions.js;\
            Copy-Item -Path ./replace/index_root.js -Destination ./node_modules/omni-sdk/src/index.js")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let new_dir = String::from_utf8_lossy(&output.stdout);
        println!("{}", new_dir);
        change_package_omni();
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

}

fn change_package_omni(){
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("node_modules/omni-sdk/package.json")
        .expect("Could not open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read the file");

    let mut new_content = String::new();

    for line in contents.lines(){
        if !line.contains("dist/index.js"){
            new_content.push_str(&line);
            new_content.push_str("\n");

        }else if line.contains("dist/index.js"){
            let index_string = "  \"main\": \"src/index.js\",";
            new_content.push_str(index_string);
            new_content.push_str("\n");
        }
    }

    let mut new_file = File::create(Path::new("node_modules/omni-sdk/package.json")).unwrap();
    new_file.write_all((&new_content).as_ref()).unwrap();

    // Start npm when all process has been completed
    npm_start();
}

fn npm_start(){
    let _output = Command::new("powershell")
        .arg("-Command")
        .arg("npm run start")
        .spawn()
        .expect("failed to execute process");

}


fn check_node_version(){
    let command = Command::new("powershell")
        .arg("-Command")
        .arg("node --version")
        .output()
        .expect("failed to execute process");


    let node_version = String::from_utf8_lossy(&command.stdout);

    print!("{}", node_version);

    if !node_version.contains("v12.22.12") {
        println!("Error, B2B is supported with v12.22.12, using: {}", node_version);
    }else {
        println!("You have the right version of node :3")
    }

    let user = env::var("USERPROFILE");
    println!("User Profile: {}", user.unwrap());
    install_node();
}

fn install_node(){

    println!("Node version will be downloaded...");

    if webbrowser::open("https://nodejs.org/dist/v12.22.12/node-v12.22.12-x64.msi").is_ok() {
        println!("Redirecting to download Node JS");
    }
    let user = env::var("USERPROFILE").unwrap();
    
    let node_path = user + "\\Downloads\\node-v12.22.12-x64.msi";
    
    println!("Node Path: {}", node_path);
    
    loop {
        
        let mut file_exist = Path::exists(node_path.as_ref());

        // println!("Trying to find and execute node installer...");
        match file_exist {
            true => {
                let output = Command::new("powershell")
                    .arg("-Command")
                    .arg(&node_path)
                    .output()
                    .expect("Failed to execute process");
                
                println!("Se encontro el archivo :33");
                if output.status.success() {
                    println!("Fin de execution");
                    //salimos del loop, y continuamos la ejecucion de node
                    //verificar que la version de npm sea 6.14.16
                    //despues de que se instale ir a otro paso que seria ahora si instalar las dependencias
                    //y pues todo lo demas normal as usual en el proceso de instalacion
                    // also hacer el checking de si ya tienes la version correcta de node, hace el bypass de todo el proceso como si nada xdddd
                    // estoyk deprimido, kiero llorar jajajajajaja pero pues la verdad no puedo hacer nada al respecto/
                    break;
                }
                ;
            }
            _ => {}
        }
    }


    // println!("{}", String::from_utf8_lossy(&output.stdout));


}