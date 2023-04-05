pub mod tools {
    pub struct Math;

    impl Math {
    
        pub fn abs(num: i32) -> i32 {
            let mut _abs_: i32 = 0;
            if num > 0 {
                _abs_ = num.clone();
            } else {
                _abs_ = -num.clone();
            }
            _abs_
        }
    
        pub fn min(num_a: i32, num_b: i32) -> i32 {
            let mut _min_: i32 = 0;
            if num_a < num_b {
                _min_ = num_a;
            } else {
                _min_ = num_b;
            }
            _min_
        }
    
        pub fn max(num_a: i32, num_b: i32) -> i32 {
            let mut _max_: i32 = 0;
            if num_a > num_b {
                _max_ = num_a;
            } else {
                _max_ = num_b;
            }
            _max_
        }
    
        pub fn sqrt(num_: i32) -> i32 {
            let sqrt_: i32 = num_ * num_;
            sqrt_
        }
    
        pub fn radians(deg: f32) -> f32 {
            let radians = deg * (3.14/180.0);
            radians
        }
    
        pub fn degrees(rad: f32) -> f32 {
            let degree = rad * (180.0/3.14);
            degree
        }
    
        pub fn constrain(x: i32, low: i32, high: i32) -> i32 {
            let mut _constr_:i32 = 0;
            if x < low {
                _constr_ = low;
            } else if x > high {
                _constr_ = high;
            } else {
                _constr_ = x;
            }
            _constr_
        }
    }    
}