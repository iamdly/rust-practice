struct Solution {}

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut point_log = Vec::<i32>::new();

        for op in ops {
            let len = point_log.len();
            let p_index = if len > 0 { len - 1 } else { 0 };
            let pp_index = if len > 1 { len - 2 } else { 0 };

            if op == "+" {
                point_log.push(point_log[pp_index] + point_log[p_index]);
            } else if op == "D" {
                point_log.push(point_log[p_index] * 2)
            } else if op == "C" {
                point_log.pop();
            } else {
                if let Ok(n) = op.parse::<i32>() {
                    point_log.push(n);
                } else {
                    point_log.push(0);
                }
            }
        }

        return if let Some(sum) = point_log.into_iter().reduce(|p, pp| p + pp) {
            sum
        } else {
            0
        };
    }
}

pub fn run() {
    println!("[p8]LeetCode 682. 棒球比赛");

    assert_eq!(
        30,
        Solution::cal_points(vec![
            "5".to_string(),
            "2".to_string(),
            "C".to_string(),
            "D".to_string(),
            "+".to_string(),
        ])
    );

    assert_eq!(
        27,
        Solution::cal_points(vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string()
        ])
    )
}
