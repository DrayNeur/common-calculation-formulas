
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let v1 = Vector3{x:20.5,y:3.2, z: 78.5};
    let v2 = Vector3{x:25.5,y:3.2, z: 78.5};
    println!("distance: {}", distance(v1,v2));
}

/*

Distance calculation:

√(x2-x1)²+(y2-y1)²+(z2-z1)²

Why pow 2 ??
Because:
x2-x1 IF 15-20
then = -5

And we don't negative values for distance
Also:

x2-x1 + y2-y1
IF
15-20 + 25-20
-5 + 5 = 0

but no !! the distance is not 0.

*/
fn distance(v1: Vector3, v2: Vector3)-> f32 {
    return (
        (v2.x-v1.x).powf(2.0)+
        (v2.y-v1.y).powf(2.0)+
        (v2.z-v1.z).powf(2.0)
    ).sqrt();
}

/*

Speed without friction calculation:

v speed in m/s
t time in s
d distance in m

v = d/t

^ Some physician guys find out that it was right :/

*/
fn speed_wf(d: f32, t: f32)->f32 {
    return d/t;
}
fn time_wf(v:f32, d:f32)->f32 {
    return v*d;
}
fn distance_wf(v:f32, t:f32)->f32 {
    return v/t;
}

/*

Convertion from m/s to km/h and km/h to m/s

v(km/h) = v(m/s)*3.6
v(m/s) = v(km/h)/3.6

*/
fn ms_to_kmh(v: f32)->f32 {
    return v*3.6;
}

fn kmh_to_ms(v: f32)->f32 {
    return v/3.6;
}

/*

Energy without friction calculation:

E energy in Joule
m mass in kg
v speed in m/s

E = 1/2*mv²

*/
fn energy_wf(m: f32, v: f32)->f32 {
    return 1/2*m*v.powf(2.0);
}

/*

Percentage calculation:

5 in 50
is 10%
(5*100)/50

10% of 50
is 5
(10*50)/100

*/
fn percent_fnum(num:u32, total:u32)->u32 {
    return (num*100)/total;
}
fn num_fpercent(percent:u32, total:u32)->u32 {
    return (percent*total)/100;
}

/*

Sound disipation calculation:

I = vol-20*log(d/1)

I decibel in dB
vol volume in INT
d distance in m

*/
fn soundDis(vol: u32, d: f32) -> f32 {
    return vol-20*log(d); // TODO: implement log function
}