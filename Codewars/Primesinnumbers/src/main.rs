
fn prime_factors(n: i64) -> String {
    let upperLimit: i64 = ((n as f64).sqrt() + 1.0) as i64;
    let mut cnt: u32 = 0;
    let mut tmpn = n;
    let mut ans: String = "".to_string();
    for i in 2..upperLimit {
        if tmpn == 1 {
            break;
        }
        while tmpn % i == 0 {
            cnt = cnt + 1;
            tmpn = tmpn / i;
        }
        if cnt == 1 {
            ans.push_str(&format!("({})", i));
        } else if cnt != 0 {
            ans.push_str(&format!("({}**{})", i, cnt));
        }
        cnt = 0;
    }
    if tmpn != 1 {
        ans.push_str(&format!("({})", tmpn));
    }
    ans
}
fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17*17*93*677, "(3)(17**2)(31)(677)");
    
}