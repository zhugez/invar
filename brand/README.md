# Invar brand kit

**Change code. Preserve behavior.**

Bộ nhận diện đề xuất cho Invar. Tên sản phẩm và tagline giữ nguyên từ dự án. Màu sắc, logo và bố cục là hướng thiết kế mới cho yêu cầu brand kit, chưa phải bộ nhận diện đã được người dùng phê duyệt.

## Ý tưởng: Invariant spine

Một chữ I có trục dọc cố định và hai thanh ngang lệch nhẹ. Hai đầu gợi sự thay đổi. Trục giữa là điểm tham chiếu được giữ lại.

Đây là ẩn dụ thị giác. Logo không có nghĩa Invar chứng minh tính đúng đắn, bảo đảm mọi hành vi, xác thực người duyệt, hay cung cấp sandbox. Không dùng khiên, dấu chứng nhận, ổ khóa hoặc dấu tick như một cam kết an toàn.

## Mở preview

Có thể mở trực tiếp `brand/index.html` trong trình duyệt. Font, hình và stylesheet đều nằm trong thư mục này, không cần CDN hay tài khoản.

Để xem qua HTTP, chạy từ thư mục gốc repository:

```sh
python3 -m http.server 8765 --bind 127.0.0.1
```

Mở `http://127.0.0.1:8765/brand/`. Dừng server bằng Ctrl+C.

## Asset

| File trong `assets/` | Công dụng |
| --- | --- |
| `logo-primary.svg` | Logo chính: biểu tượng xanh, chữ Graphite, nền trong suốt. |
| `logo-mono.svg` | Logo một màu Graphite cho nền sáng và in đơn sắc. |
| `logo-reversed.svg` | Logo màu Chalk cho nền Graphite hoặc Signal Blue. |
| `wordmark.svg` | Chữ invar riêng, không kèm biểu tượng. |
| `mark.svg` | Biểu tượng xanh, nền trong suốt. |
| `mark-reversed.svg` | Biểu tượng Chalk, nền trong suốt. |
| `icon.svg`, `icon.png` | Icon vuông 512 × 512, nền Signal Blue. |
| `favicon.svg` | Icon nhỏ, nền xanh đục để không phụ thuộc màu tab. |
| `social-card.svg`, `social-card.png` | Ảnh chia sẻ 1200 × 630 với tagline hiện có. |

Logo và wordmark dùng path SVG được vẽ riêng, không chứa text hay tham chiếu font. `social-card.svg` nhúng Barlow Bold và toàn bộ giấy phép font trong metadata để giữ typography và thông tin giấy phép khi mở độc lập. PNG là bản render từ SVG, không phải ảnh được tạo bằng mô hình sinh ảnh.

`manifest.json` ghi nguồn và SHA-256 của font. `make_assets.py` là nguồn hình học dùng chung cho các biến thể SVG.

### Quy tắc logo

- Giữ nguyên tỉ lệ và độ lệch của hai thanh ngang.
- Chừa khoảng trống ít nhất bằng độ rộng trục giữa ở mọi phía. Ưu tiên gấp đôi khoảng đó.
- Chiều rộng khuyến nghị tối thiểu: lockup 144 px, biểu tượng 16 px. Kiểm tra lại ở kích thước in thực tế.
- Dùng bản primary hoặc mono trên nền sáng. Dùng bản reversed trên Graphite hoặc Signal Blue.
- Không kéo giãn, xoay, bo góc thêm, tạo gradient, thêm glow hoặc thay chữ bằng font gần giống.
- Không đặt trên ảnh hoặc nền thiếu tương phản. Không dùng màu trạng thái sản phẩm để thay màu thương hiệu.
- Slider trong preview là nghiên cứu chuyển động. Nó không chỉnh sửa file asset.

## Màu sắc

| Tên | Hex | Vai trò |
| --- | --- | --- |
| Signal Blue | `#2347F5` | Màu thương hiệu, điểm nhấn, liên kết trên nền sáng. |
| Graphite | `#17262C` | Chữ chính, nền tối. |
| Chalk | `#F3F6F7` | Nền sáng, chữ đảo trên nền Blue hoặc Graphite. |
| Steel | `#586A74` | Chữ phụ trên nền sáng. |
| Line | `#CFD8DC` | Đường phân cách trang trí, không làm ranh giới duy nhất cho control. |

Tương phản được tính từ các mã sRGB này: Graphite/Chalk **14.32:1**, Blue/Chalk **5.88:1**, Steel/Chalk **5.19:1**, Graphite/Line **10.75:1**. Đây là kiểm tra các cặp màu, không phải chứng nhận toàn bộ giao diện đáp ứng một tiêu chuẩn accessibility.

Màu trạng thái tách khỏi màu thương hiệu: pass `#166443`, change `#A23D27`, review `#805907`. Luôn kèm nhãn hoặc nội dung mô tả. Không dùng màu đơn độc để báo trạng thái. Các màu này dùng được làm chữ trên Chalk theo phép tính tương phản trong script kiểm tra.

CSS cho giao diện web có trong `tokens.css`. Các giá trị máy đọc được có trong `tokens.json`. Preview hỗ trợ giao diện sáng/tối nhưng giữ nguyên màu thực của các mẫu nhận diện.

## Typography và bố cục

**Barlow Regular 400** cho nội dung. **Barlow Bold 700** cho tiêu đề và nhấn mạnh. Hai file TTF gốc nằm trong `fonts/`, kèm nguyên văn giấy phép SIL Open Font License 1.1 và thông tin tác giả trong `fonts/OFL.txt`.

Dùng system monospace cho lệnh, đường dẫn, hash và giá trị kỹ thuật. Không ép người dùng đổi font terminal để dùng CLI.

- Tiêu đề lớn: 48–96 px trên desktop. Thu nhỏ theo chiều rộng màn hình.
- Tiêu đề mục: 32–48 px. Nội dung chính: 17–18 px.
- Body line-height khoảng 1.55. Đoạn đọc dài không rộng quá 68 ký tự tương đối.
- Khoảng cách nền: 4, 8, 12, 16, 24, 32, 48, 64, 96 px.
- Dùng hình chữ nhật rõ cạnh, đường mảnh và khoảng trống. Không biến mọi nội dung thành card.
- Trong ví dụ kỹ thuật, giữ lệnh và output chính xác. Không thêm trang trí vào JSON.

## Giọng điệu

**Chính xác. Trực tiếp. Có trách nhiệm.**

Mô tả điều quan sát được, nêu giới hạn và chỉ ra hành động tiếp theo. Không nhân cách hóa lỗi của AI hoặc hứa bảo vệ tuyệt đối.

| Nên viết | Tránh viết |
| --- | --- |
| “All configured examples match accepted behavior.” | “Your code cannot break.” |
| “stdout changed. Review the first differing byte.” | “The AI made your app unsafe.” |
| “Review the candidate before locking it.” | “Automatically trust the new baseline.” |

**Mô tả ngắn:** A local behavior check for code changes.

**Mô tả sản phẩm:** Invar is a local CLI that captures command results, requires explicit approval, and compares later runs with accepted examples. It complements existing tests.

**Tagline chuẩn:** Change code. Preserve behavior.

Dùng “Invar” trong câu văn, `invar` khi viết lệnh. Giữ nguyên tagline tiếng Anh trong logo và ảnh chia sẻ. Không tạo testimonial, số liệu sử dụng hoặc tuyên bố khác biệt chưa được chứng minh.

## Tái tạo và kiểm tra

Không cần thư viện Python ngoài standard library:

```sh
python3 brand/make_assets.py
python3 brand/check.py
```

Lệnh đầu tạo lại SVG, token và manifest. Lệnh sau kiểm tra SVG, file tham chiếu, font, màu và PNG. Hai lệnh không sửa CLI, candidate hoặc accepted baseline.

Để xuất lại cả PNG, cần CLI `agent-browser` và Chromium đã cài sẵn:

```sh
python3 brand/make_assets.py --png
python3 brand/check.py
```

Script mở SVG cục bộ, đợi font tải xong và chụp bằng Chromium ở scale 1: icon 512 × 512, social card 1200 × 630. Script cũng thêm metadata nguồn vào PNG và đóng phiên trình duyệt xuất ảnh. Chỉ việc tái xuất PNG mới cần công cụ này. Mở preview hoặc dùng các asset có sẵn không cần agent-browser.

## Kiểm tra đã thực hiện

- Preview được xem trên Chromium ở các bề rộng 320, 390, 768, 1024 và 1440 px. Không phát hiện tràn ngang ở các kích thước này.
- Font Regular/Bold tải thành công. Các hình đã được kiểm tra sau khi tải, bao gồm ảnh dùng lazy loading. Preview cũng mở thành công bằng `file:` khi trình duyệt ở chế độ offline.
- Kiểm tra 18 đích liên kết nội bộ qua HTTP, tất cả trả về 200. Một logo tải qua trình duyệt khớp từng byte với file nguồn.
- Kiểm tra tương phản từ màu chữ và nền thực tế của 110 phần tử văn bản ở chế độ sáng và tối trên desktop. Không phát hiện cặp nào dưới ngưỡng dùng trong phép kiểm tra: 4.5:1 cho chữ thường, 3:1 cho chữ lớn. Đây là kiểm tra tự động giới hạn, không thay thế đánh giá accessibility đầy đủ.
- Slider phản hồi phím mũi tên, cập nhật giá trị hiển thị và giữ nguyên vị trí trục giữa. Skip link đưa focus vào nội dung chính. Khi bật reduced motion, hành vi cuộn chuyển về `auto`.
- SVG parse thành công. PNG đúng kích thước, có metadata nguồn và kiểm tra CRC hợp lệ. Hash font khớp manifest.
- Công cụ detector của Impeccable chạy bằng regex vì thiếu module parser. Không coi kết quả rỗng của detector là bằng chứng kiểm tra đầy đủ. Các phép kiểm tra trình duyệt và `check.py` bổ sung bằng chứng cụ thể ở trên.

## Phạm vi giấy phép và tên gọi

Asset gốc và mã nguồn mới thuộc phạm vi giấy phép dự án [Apache-2.0](../LICENSE). Font giữ giấy phép riêng [OFL 1.1](fonts/OFL.txt). Giữ file giấy phép font khi phân phối bộ kit.

Bộ nhận diện không bao gồm việc tra cứu hoặc xác nhận quyền nhãn hiệu. Các tên Invar trùng lặp đã được ghi nhận trong [nghiên cứu của dự án](../docs/research/competitive-analysis.md). Việc tạo logo không giải quyết các xung đột tên đó.
