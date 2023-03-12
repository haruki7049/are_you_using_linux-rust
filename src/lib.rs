/// Check your OS
pub fn are_you_using_linux(){
    if cfg!(target_os = "linux"){
        println!("You are using Linux!! You are My friends!! Foo!!");
    }else if cfg!(target_os = "macos"){
        println!("You are using Mac OS. Surely Mac OS is cool. I understood why you are using Mac OS.");
    }else if cfg!(target_os = "windows"){
        println!("You are using Windows. Why don't you use Linux? Linux give you very nice experience!!");
    }else{
        println!("Wow!! You are using the OS which is not known by us!! You should be using the OS!!");
    }
}
