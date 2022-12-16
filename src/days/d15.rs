use std::{fs, collections::{HashMap, btree_map::Range}, ops::RangeInclusive};

pub fn d15() -> (String, String) {
    let read = fs::read_to_string("inputs/d15").unwrap();
    let mut lines = read.split("\n").collect::<Vec<&str>>();
    lines.remove(lines.len()-1);
    let mut sensors: Vec<(i32,i32,usize)> = vec![];

    for line in lines {
        let split1 = line.split_once(": closest beacon is at x=").unwrap();
        let (a,b) = split1.0[12..].split_once(", y=").unwrap();
        let (c,d) = split1.1.split_once(", y=").unwrap();
        sensors.push((
            a.parse().unwrap(),
            b.parse().unwrap(),
            distance(
                (a.parse().unwrap(),
                b.parse().unwrap()),
                (c.parse().unwrap(),
                d.parse().unwrap())
            )
        ));
    }
    
    let p1 = scan_y_count(&sensors, 2000000);
    
    let mut p2 = 0;
    for y in 0..=4000000 {
        let r = scan_y(&sensors, y);
        if let Some(r) = r {
            p2 = r;
            break;
        }
    }

    ((p1-1).to_string(), p2.to_string())
}

fn distance(a: (i32,i32), b: (i32,i32)) -> usize {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    (dx+dy) as usize
}

fn tuning(x: i32, y: i32) -> Option<u64> {
    println!("{x},{y}");
    if x < 0 || x > 4000000 || y < 0 || y > 4000000 {
        return None;
    } else {
        return Some(((x as u64)*4000000 + y as u64) as u64);
    }
}

fn scan_y(sensors: &Vec<(i32,i32,usize)>, y: i32) -> Option<u64> {
    let mut found = None;
    let ranges: Vec<RangeInclusive<i32>> = sensors.iter()
        .map(|x| sensor_range(x,y))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let mut x = 0;
    while x <= 4000000 {
        let mut ok = true;
        for range in ranges.iter() {
            if range.contains(&x) {
                ok = false;
                x = *range.end()+1;
                break;
            }
        }
        if ok {
            found = Some(tuning(x,y).unwrap());
            break;
        }
    }
    found
}

fn scan_y_count(sensors: &Vec<(i32,i32,usize)>, y: i32) -> usize {
    let ranges: Vec<RangeInclusive<i32>> = sensors.iter()
        .map(|x| sensor_range(x,y))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for range in ranges.iter() {
        min = min.min(*range.start());
        max = max.max(*range.end());
    }

    let mut count = 0;
    let mut x = min;
    while x <= max {
        for range in ranges.iter() {
            if range.contains(&x) {
                count += *range.end()-x+1;
                x = *range.end()+1;
                break;
            }
        }
    }
    count as usize
}

fn sensor_range(sensor: &(i32,i32,usize), y: i32) -> Option<RangeInclusive<i32>> {
    let dy = sensor.1.abs_diff(y) as usize;
    let range = sensor.2 as i32 - dy as i32;
    if range < 0 {
        return None;
    }
    Some((sensor.0 - range)..=(sensor.0 + range))
}