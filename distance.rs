
//returns distance between two coordinates (deg) in miles
fn distance(lat1deg:f32, lon1deg:f32, lat2deg:f32, lon2deg:f32) -> f32 {
    let earth_r_km = 6371.0 ;
    let lat1 = lat1deg*3.1415926/180.0;
    let lon1 = lon1deg*3.1415926/180.0;
    let lat2 = lat2deg*3.1415926/180.0;
    let lon2 = lon2deg*3.1415926/180.0;
    
    let l = (((lat2-lat1)/2.0).sin())*(((lat2-lat1)/2.0).sin());
    println!("l: {}", l);
    let r = (lat1.cos()*lat2.cos())*(((lon2-lon1)/2.0).sin())*(((lon2-lon1)/2.0).sin());
    println!("r: {}", r);
    let b = (l+r).sqrt();
    println!("b: {}", b);
    let d_km = 2.0*earth_r_km*((b).asin());
    println!("d_km: {}", d_km);
    let d_mile = 0.62137119*d_km;
    return d_mile;
}