fn main() {
    let v = std::io::stdin().lines().take(2).collect::<Vec<_>>();
    let (a, b) = (v[0].as_ref().unwrap(), v[1].as_ref().unwrap()).clone();
    let mut dp = vec![vec![0; a.len() + 1]; b.len() + 1];
    for j in 0..=a.len() {
        dp[0][j] = j;
    }
    for i in 0..=b.len() {
        dp[i][0] = i;
    }
    for (i, b) in b.chars().enumerate().map(|(i, c)| (i + 1, c)) {
        for (j, a) in a.chars().enumerate().map(|(i, c)| (i + 1, c)) {
            dp[i][j] = if a == b {
                dp[i - 1][j - 1]
            } else {
                dp[i - 1][j - 1] + 1
            }
            .min(dp[i - 1][j] + 1)
            .min(dp[i][j - 1] + 1);
        }
    }
    println!("{}", dp[b.len()][a.len()]);
}
