struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank { balance }
    }

    // 用户转账
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        // 检查账户是否存在
        if !self.check_account(account1) {
            return false;
        }

        if !self.check_account(account2) {
            return false;
        }

        // 检查账户1是否有足够的钱
        if !self.check_account_money(account1, money) {
            return false;
        }

        // 转账
        self.balance[(account1 as usize) - 1] -= money;
        self.balance[(account2 as usize) - 1] += money;

        return true;
    }

    // 用户存款
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        // 检查账户是否存在
        if !self.check_account(account) {
            return false;
        }

        // 存款
        self.balance[(account as usize) - 1] += money;

        return true;
    }

    // 用户取款
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        // 检查账户是否存在
        if !self.check_account(account) {
            return false;
        }

        // 检查账户余额是否足够
        if !self.check_account_money(account, money) {
            return false;
        }

        self.balance[(account as usize) - 1] -= money;

        return true;
    }

    // 检查账户是否存在
    fn check_account(&self, account: i32) -> bool {
        account >= 0 && account <= self.balance.len() as i32
    }

    // 检查账户余额是否充足
    fn check_account_money(&self, account: i32, money: i64) -> bool {
        self.get_account_money(account) >= money
    }

    // 获取用户余额
    fn get_account_money(&self, account: i32) -> i64 {
        self.balance[(account as usize) - 1]
    }
}

pub fn run(arg: &str) {
    println!("[p5]LeetCode2043.简易银行系统: {}", arg);

    let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);

    // 返回 true ，账户 3 的余额是 $20 ，所以可以取款 $10 。
    // 账户 3 余额为 $20 - $10 = $10 。
    assert_eq!(bank.withdraw(3, 10), true);

    // 返回 true ，账户 5 的余额是 $30 ，所以可以转账 $20 。
    // 账户 5 的余额为 $30 - $20 = $10 ，账户 1 的余额为 $10 + $20 = $30 。
    assert_eq!(bank.transfer(5, 1, 20), true);

    // 返回 true ，可以向账户 5 存款 $20 。
    // 账户 5 的余额为 $10 + $20 = $30 。
    assert_eq!(bank.deposit(5, 20), true);

    // 返回 false ，账户 3 的当前余额是 $10 。
    // 所以无法转账 $15 。
    assert_eq!(bank.transfer(3, 4, 15), false);

    // 返回 false ，交易无效，因为账户 10 并不存在。
    assert_eq!(bank.withdraw(10, 50), false);

    println!("SUCCESS");
}
