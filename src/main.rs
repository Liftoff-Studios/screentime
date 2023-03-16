use notify_rust::*;
use std::fs;
use chrono::*;
use std::thread;

fn main()->std::io::Result<()>{
    //Get config file contents	
    let fc = fs::read_to_string("./config.txt")?;
    
    //Get the current time using chrono
    let time = Local::now();

    //Variables
    let mut daily_limit: i32 = 0; // Your daily time limit
    let mut interval: i32 = 0; // The interval after which it reminds you how much time you have left
    let mut time_done: i32 = 0; // Time you've completed while this program has been on

    //Get Variables and times from config.txt contents
    for (i, line) in fc.lines().enumerate(){
        if i==0{
            daily_limit = line.split("=").collect::<Vec<&str>>()[1].trim().parse::<i32>().unwrap();
        }else if i==1{
            interval = line.split("=").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }else{
            let whole = line.split(":").collect::<Vec<&str>>();
            let date = whole[0].split("/").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

            if time.year()==(date[2] as i32) && time.month()==date[1] && time.day()==date[0]{
                time_done = whole[1].trim().parse::<i32>().unwrap();
            }
        }
    }

    // Checks before running a program
    if time_done>=daily_limit {
        Notification::new()
            .summary("Screen Time Over")
            .body("Your screen time is over, please stop looking at the screen")
            .timeout(Timeout::Never) //milliseconds
            .show().unwrap();
    }

    //Creates the eternal loop LOL
    loop{
	  // Sleeps the thread and blocks it
        thread::sleep(
            std::time::Duration::from_millis((interval*60*1000)as u64)
        );

	  //Updates time done
        time_done+=interval;

	  // Checks if daily limit is surpassed
        if time_done>=daily_limit {
            Notification::new()
                .summary("Screen Time Over")
                .body("Your screen time is over, please stop looking at the screen")
                .timeout(Timeout::Never) //milliseconds
                .show().unwrap();
        }
        else{
            Notification::new()
                .summary("Screen Time Update")
                .body(format!("You have elapsed another {} minutes\nYou have {} minutes remaining for today", interval, daily_limit-time_done).as_str())
                .timeout(Timeout::Never) //milliseconds
                .show().unwrap();
        }

	  //Update config file
	  update_config(&interval, &daily_limit, &time_done);
    }

    // Need it for compiling even though value is never read
    Ok(())
}


//Update function
fn update_config(interval: &i32, daily_limit:&i32, time_done: &i32){
    //Get the current time
    let time = Local::now();

    //Create the string to write to the file
    let data = format!("daily={}\r\ninterval={}\r\n{}/{}/{}: {}",
                        daily_limit.clone(),
                        interval.clone(),
                        time.day(),
                        time.month(),
                        time.year(),
                        time_done.clone()
                    );

    //Writes the data to file
    let data = data.as_str();
    fs::write("./config.txt", data).expect("Unable to write file");
}