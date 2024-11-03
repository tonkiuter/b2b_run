use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    
    let args : Vec<String> = env::args().collect();

    let output = Command::new("powershell")
        .arg("-Command")
        .arg("cd src; cp file.js file2.js;")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let new_dir = String::from_utf8_lossy(&output.stdout);
        println!("Nuevo directorio: {}", new_dir);
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    println!("Pointing to env number: {:?} cluster {:?}", args[1], args[2]);
    
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("C:/Users/PC/RustroverProjects/ModifyFile/src/file.js")
        .expect("Could not open file");

    let mut contents = String::new();

    let mut count : i32 = 0;

    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    let cluster = "411";
    
    let url = "amd-apigw-console.openshift.ilocpqcmmil".to_owned() + &*args[1] + ".ocp" + &*args[2] + ".amdocs.corp";
    
    println!("{}", url);
    
    let mut len_of_file : u64 = 0;
    
    let mut new_content = String::new();
    
    for line in contents.lines(){
        len_of_file += line.len() as u64;
        count += 1;
        if !line.starts_with("amd-apigw"){
            new_content.push_str(&line);
            new_content.push_str("\n");

        }else if line.starts_with("amd-apigw"){ 
            new_content.push_str(url.as_str());
            new_content.push_str("\n");
        }
        // if line.starts_with("amd-apigw"){
            // println!("Lo que se va a borrar: {}", line);
            // 
            // println!("la url: {}", url);


            // new_content.replace("amd-apigw-console.openshift.ilocpqcmm236.amdocs.corp", "nani");
            
            
            // len_of_file = len_of_file - line.len() as u64;
            // 
            // println!("Len of Escritura {}",len_of_file);
            // // println!("This line must be replaced with correct ocp cluster! ");
            // 
            // file.seek(std::io::SeekFrom::Start(len_of_file)).unwrap();
            // 
            // for i in 0..line.len() {
            //     file.write_all(b" ");
            // }
            // file.write_all(url.as_bytes()).expect("Unable to write to file");
        // }
    }

    // file.seek(std::io::SeekFrom::Start(0)).unwrap();
    // file.write_all(url.as_bytes()).expect("Unable to write to file");
    
    let mut new_file = File::create(Path::new("C:/Users/PC/RustroverProjects/ModifyFile/src/file.js")).unwrap();
    new_file.write_all((&new_content).as_ref()).unwrap();

}
