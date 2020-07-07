use regex::Regex;
const TO_SEARCH:&'static str= "On 2020-12-31, happy. On 2018-01-01, New year.";
fn main(){
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for caps in re.captures_iter(TO_SEARCH){
        println!("year: {}, month: {}, day: {}", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str())
    }
}