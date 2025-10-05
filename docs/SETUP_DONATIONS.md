# 設定贊助收款指南

## 快速設定檢查清單

### ✅ Ko-fi 設定（推薦先做）

1. **註冊 Ko-fi**
   - 網址：https://ko-fi.com/
   - 時間：5 分鐘

2. **連接 PayPal**
   - Settings → Payments → PayPal
   - 授權後即可收款

3. **設定贊助金額**
   ```
   建議金額（台幣）：
   - 小額：NT$100 (~$3 USD)
   - 中額：NT$300 (~$10 USD)
   - 大額：NT$500 (~$17 USD)
   ```

4. **取得您的 Ko-fi 連結**
   ```
   格式：https://ko-fi.com/YOUR_USERNAME
   ```

5. **更新專案文件**
   需要更新的文件：
   - `.github/FUNDING.yml`
   - `README.md`
   - `docs/README.zh-TW.md`
   - `docs/README.zh-CN.md`
   - `docs/README.de.md`

### ✅ GitHub Sponsors 設定（選擇性）

1. **申請資格檢查**
   - GitHub 帳號 > 90 天
   - 有公開專案
   - 遵守服務條款

2. **申請步驟**
   - 前往：https://github.com/sponsors
   - 填寫申請表
   - 等待審核（通常 1-3 天）

3. **設定 Stripe（台灣可用）**
   需要準備：
   - 身分證
   - 銀行帳戶資訊
   - 地址證明

4. **設定贊助層級**
   ```
   建議層級（美金）：
   - $1/月：基本支持
   - $5/月：標準支持
   - $10/月：進階支持
   - $25/月：企業支持
   ```

## 📊 手續費比較

| 平台 | 平台費用 | 支付手續費 | 總手續費 | 台灣友善度 |
|------|----------|------------|----------|------------|
| Ko-fi | 0% | PayPal 3.4%+NT$10 | ~4.4% | ⭐⭐⭐⭐⭐ |
| GitHub Sponsors | 0% | Stripe 3.4%+NT$10 | ~4.4% | ⭐⭐⭐ |
| Buy Me a Coffee | 5% | PayPal 3.4%+NT$10 | ~9.4% | ⭐⭐⭐⭐ |
| Patreon | 5-12% | Stripe 2.9%+NT$10 | ~8-15% | ⭐⭐⭐ |

**結論：Ko-fi 最適合台灣用戶**

## 🔧 更新文件範例

### 1. 更新 .github/FUNDING.yml

```yaml
# 更新為您的實際帳號
ko_fi: ivanh0906
github: YOUR_GITHUB_USERNAME
custom: ['https://ko-fi.com/ivanh0906']
```

### 2. 更新 README.md

在 "Support This Project" 區塊更新：

```markdown
## 💝 Support This Project

- 💰 [Sponsor on Ko-fi](https://ko-fi.com/ivanh0906)
- 💰 [GitHub Sponsors](https://github.com/sponsors/YOUR_GITHUB_USERNAME)
- ⭐ Star this repository
```

### 3. 更新多語言文檔

繁體中文 (README.zh-TW.md)：
```markdown
- 💰 [Ko-fi 贊助](https://ko-fi.com/ivanh0906)
- 💰 [GitHub Sponsors](https://github.com/sponsors/YOUR_GITHUB_USERNAME)
```

簡體中文 (README.zh-CN.md)：
```markdown
- 💰 [Ko-fi 赞助](https://ko-fi.com/ivanh0906)
- 💰 [GitHub Sponsors](https://github.com/sponsors/YOUR_GITHUB_USERNAME)
```

德語 (README.de.md)：
```markdown
- 💰 [Auf Ko-fi sponsern](https://ko-fi.com/ivanh0906)
- 💰 [GitHub Sponsors](https://github.com/sponsors/YOUR_GITHUB_USERNAME)
```

## 💡 最佳實踐

### 在 Ko-fi 頁面上：

1. **清楚說明資金用途**
   ```
   支持 BOM 計算引擎開發：
   ✓ 持續功能開發
   ✓ Bug 修復與維護
   ✓ 文檔改進
   ✓ 社群支援
   ```

2. **設定目標（選填）**
   ```
   範例月度目標：
   - NT$3,000/月：基本維護
   - NT$10,000/月：新功能開發
   - NT$30,000/月：全職開發
   ```

3. **提供回饋**
   ```
   贊助者福利（可選）：
   - 優先 Bug 修復
   - 功能建議優先考慮
   - 專案致謝名單
   - 早期功能預覽
   ```

## 🎯 行動計畫

### 今天立即執行：
- [ ] 註冊 Ko-fi
- [ ] 連接 PayPal
- [ ] 設定贊助金額
- [ ] 更新 .github/FUNDING.yml
- [ ] 更新所有 README 文件

### 本週內完成：
- [ ] 申請 GitHub Sponsors
- [ ] 準備 Stripe 所需文件
- [ ] 優化 Ko-fi 頁面內容
- [ ] 測試贊助流程

### 長期維護：
- [ ] 定期感謝贊助者
- [ ] 公開資金使用情況
- [ ] 更新開發進度
- [ ] 回應贊助者反饋

## ❓ 常見問題

**Q: 收到的贊助需要繳稅嗎？**
A: 在台灣，個人收入超過一定金額需申報所得稅。建議諮詢會計師。

**Q: PayPal 提款手續費多少？**
A: 提款到台灣銀行帳戶通常免手續費，但可能有匯率損失。

**Q: GitHub Sponsors 何時入帳？**
A: 通常每月初結算上月收入，約 7-14 天後入帳。

**Q: 可以同時使用多個平台嗎？**
A: 可以！建議 Ko-fi + GitHub Sponsors 雙管齊下。

**Q: 如何追蹤贊助收入？**
A: 各平台都有儀表板顯示收入報表，記得定期下載備份。

## 📞 需要幫助？

如有問題請聯絡：
- Email: xiaoivan1@proton.me
- GitHub Issues: 您的專案 Issues 頁面

---

祝您收款順利！🎉
