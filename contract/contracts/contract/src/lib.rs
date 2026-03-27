#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct BankContract;

#[contractimpl]
impl BankContract {
    /// Nạp tiền: Chuyển token từ ví người dùng vào Contract
    pub fn deposit(env: Env, token_id: Address, user: Address, amount: i128) {
        // 1. Xác thực người dùng
        user.require_auth();

        if amount <= 0 {
            panic!("So tien nap phai lon hon 0");
        }

        // 2. Khởi tạo client cho Token (ví dụ: XLM hoặc USDC)
        let client = token::Client::new(&env, &token_id);

        // 3. Thực hiện chuyển tiền từ ví người dùng (user) vào chính Contract này (env.current_contract_address())
        client.transfer(&user, &env.current_contract_address(), &amount);

        // 4. Lưu lại số dư nội bộ để ghi nhận 'user' đã nạp bao nhiêu
        let mut balance: i128 = env.storage().persistent().get(&user).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&user, &balance);
    }

    /// Rút tiền: Chuyển token từ Contract trả lại ví người dùng
    pub fn withdraw(env: Env, token_id: Address, user: Address, amount: i128) {
        user.require_auth();

        // 1. Kiểm tra số dư nội bộ trong Contract
        let mut balance: i128 = env.storage().persistent().get(&user).unwrap_or(0);
        if balance < amount {
            panic!("So du trong hop dong khong du");
        }

        // 2. Cập nhật số dư nội bộ trước (tránh lỗi Reentrancy)
        balance -= amount;
        env.storage().persistent().set(&user, &balance);

        // 3. Thực hiện chuyển Token từ Contract trả lại cho người dùng
        let client = token::Client::new(&env, &token_id);
        client.transfer(&env.current_contract_address(), &user, &amount);
    }

    /// Xem số dư mà người dùng đã gửi vào Contract
    pub fn get_bal(env: Env, user: Address) -> i128 {
        env.storage().persistent().get(&user).unwrap_or(0)
    }
}