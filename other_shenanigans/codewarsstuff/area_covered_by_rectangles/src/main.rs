//calc area covered by union of rectangles
// non-empty intersection
// [x0, y0, x1, y1]
// bottom left - (x0, y0)
// top right - (x1, y1)
// xi, yi -positive integers or zeros - use unsigned (u32?)
// input data is an array of rectangles - so 2d array, need to loop through 2d array
// calc difference between x0 and x2 (point on second rectangle)
// area calc onboth rectangles comboned, minus that difference

//finding the area
//given rectangle [3, 3, 8, 5]
//8 -3 gives 5 for the length side    x1-x0
//5 - 3 gives 2 for the height        y1-y0
// 5 * 2 = area 10
//
// so we cycle through 2d array, get area for one rectangle by multiplying (x1-x0) by (y1-y0)
// to handle overlap though...
// compare xs and ys
// if first rectangle x0 or x1 is less than or equal to second rectangle's x0 or x1...
// using vertical sweep line instead here becuase it needs to be efficient
//
fn calculate(rectangles: &[[i32; 4]]) -> i64 {
    //borrrow the data instead of taking ownership
    //a reference to a slice of arrays containing 4 i32s
    //
    //sweep-line algo works on x coordinate events instead of directly on rectangles
    let mut events: Vec<(i32, i32, i32, i32)> = Vec::new();

    for &[x0, y0, x1, y1] in rectangles {
        if x0 == x1 || y0 == y1 {
            continue;
        }

        events.push((x0, y0, y1, 1)); // rectangle starts
        events.push((x1, y0, y1, -1)); // rectangle ends
    }

    if events.is_empty() {
        return 0;
    }

    events.sort_by_key(|e| e.0); //for each item in events, call it e and use e.0 as the sorting
    //key (the x coordinate)

    let mut active: Vec<(i32, i32)> = Vec::new();
    let mut area: i64 = 0;
    let mut prev_x = events[0].0;
    let mut i = 0;

    while i < events.len() {
        let x = events[i].0;
        let width = (x - prev_x) as i64;

        if width > 0 {
            let covered_y = merged_y_length(&active);
            area += width * covered_y;
        }

        while i < events.len() && events[i].0 == x {
            let (_, y0, y1, kind) = events[i];

            if kind == 1 {
                insert_interval(&mut active, (y0, y1));
            } else {
                remove_interval(&mut active, (y0, y1));
            }

            i += 1;
        }

        prev_x = x;
    }

    area
}

fn merged_y_length(intervals: &[(i32, i32)]) -> i64 {
    if intervals.is_empty() {
        return 0;
    }

    let mut total = 0_i64;
    let mut start = intervals[0].0;
    let mut end = intervals[0].1;

    for &(y0, y1) in &intervals[1..] {
        if y0 <= end {
            end = end.max(y1);
        } else {
            total += (end - start) as i64;
            start = y0;
            end = y1;
        }
    }

    total + (end - start) as i64
}

fn insert_interval(active: &mut Vec<(i32, i32)>, interval: (i32, i32)) {
    let pos = active
        .binary_search_by_key(&interval, |&(y0, y1)| (y0, y1))
        .unwrap_or_else(|pos| pos);

    active.insert(pos, interval);
}

fn remove_interval(active: &mut Vec<(i32, i32)>, interval: (i32, i32)) {
    if let Some(pos) = active.iter().position(|&x| x == interval) {
        active.remove(pos);
    }
}

fn main() {
    let rectangles = [[3, 3, 8, 5], [6, 3, 8, 9], [11, 6, 14, 12]];

    println!("{}", calculate(&rectangles)); // 36, passes reference 
}
