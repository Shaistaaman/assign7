mod weather_2020
{
    pub mod summer
    {
              
        pub fn value () 
        {
            println!("Summer goes upto 40C :: from sub module");
        }
    }
    pub fn value_out () 
        {
            println!("From main module");
        }
}

use crate::weather_2020::summer;

fn main()
{
        
    summer::value();  
    weather_2020::value_out();
  
}
