# Bank-Management
Title
Soroban SafeVault – Hệ thống Ngân hàng Phi tập trung trên Stellar

Description
Soroban SafeVault là một giải pháp quản lý tài sản số (Stellar Assets) đơn giản nhưng bảo mật cao. Dự án này được xây dựng để cho phép người dùng nạp, lưu trữ và rút các token (như XLM hoặc Stablecoins) thông qua Hợp đồng thông minh (Smart Contract).

Mục đích: Tạo ra một môi trường lưu trữ tài sản không cần sự can thiệp của con người (Trustless), nơi các quy tắc nạp/rút được thực thi hoàn toàn bằng code.

Tại sao làm Idea này? Với sự phát triển của DeFi, việc hiểu cách quản lý luồng tiền giữa ví cá nhân và Smart Contract là nền tảng cốt lõi. Dự án này giúp giải quyết bài toán quản lý số dư nội bộ và tương tác trực tiếp với Standard Token Interface của Soroban.

Tính năng
Dự án bao gồm 3 chức năng chính được bảo mật bằng cơ chế xác thực của Stellar:

Deposit (Nạp tiền): Cho phép người dùng chuyển Token từ ví cá nhân vào Smart Contract. Hệ thống sẽ tự động cập nhật số dư ghi nhận cho địa chỉ ví đó.

Withdraw (Rút tiền): Người dùng có thể rút lại số tiền đã gửi bất cứ lúc nào. Hàm này đi kèm kiểm tra số dư và yêu cầu chữ ký xác thực (require_auth) để đảm bảo an toàn.

Balance Inquiry (Kiểm tra số dư): Truy vấn nhanh số dư hiện tại đang được khóa trong hợp đồng thông minh.

Multi-Token Support: Hợp đồng được thiết kế linh hoạt để có thể làm việc với bất kỳ mã thông báo nào (XLM, USDC, v.v.) chỉ bằng cách truyền token_id.

Contract
Link tới contract: https://stellar.expert/explorer/testnet/contract/CBMU6IONTBSK6NAXHGLPUHKXTTL4NKHDH3K6FL3MK72445X3KFM24HBB

Future scopes
Trong tương lai, Soroban SafeVault sẽ được phát triển thêm các tính năng:

Time-Lock: Cho phép người dùng khóa tài sản trong một khoảng thời gian nhất định (phù hợp cho tiết kiệm dài hạn).

Yield Farming: Tích hợp với các giao thức cho vay để tạo ra lãi suất từ số dư đang lưu trữ.

<img width="1709" height="614" alt="image" src="https://github.com/user-attachments/assets/e9ffaa97-d10d-4565-84fd-840c5e798f20" />


Web Dashboard: Xây dựng giao diện Web3 hoàn chỉnh bằng React/Next.js giúp người dùng thao tác nạp/rút mà không cần dùng dòng lệnh (CLI).

