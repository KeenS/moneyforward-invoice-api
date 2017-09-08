use chrono::*;
use reqwest;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// APIエラー
pub struct ApiError {
    /// e.g. "400"
    pub code: String,
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 詳細エラー
pub struct Error {
    /// e.g. "必要なパラメーターが存在しない、もしくは空です。"
    pub message: String,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 事業所
pub struct Office {
    /// 事業所名 e.g. "サンプル事業所"
    pub name: String,
    /// 郵便番号 e.g. "123-4567"
    pub zip: String,
    /// 県 e.g. "東京都"
    pub prefecture: String,
    /// 住所（丁目まで） e.g. "港区サンプル1-2-3"
    pub address1: String,
    /// 住所（建物以降） e.g. "サンプルビル"
    pub address2: String,
    /// 電話番号 e.g. "03-1234-5678"
    pub tel: String,
    /// FAX番号 e.g. "03-5678-1234"
    pub fax: String,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 事業所の更新リクエスト用データ
pub struct UpdateOffice {
    /// 事業所名 e.g. "サンプル事業所"
    pub name: Option<String>,
    /// 郵便番号 e.g. "123-4567"
    pub zip: Option<String>,
    /// 県 e.g. "東京都"
    pub prefecture: Option<String>,
    /// 住所（丁目まで） e.g. "港区サンプル1-2-3"
    pub address1: Option<String>,
    /// 住所（建物以降） e.g. "サンプルビル"
    pub address2: Option<String>,
    /// 電話番号 e.g. "03-1234-5678"
    pub tel: Option<String>,
    /// FAX番号 e.g. "03-5678-1234"
    pub fax: Option<String>,
}


#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 取引先一覧
pub struct Partners {
    /// 結果のメタデータ
    pub meta: Meta,
    /// 取引先一覧
    pub partners: Vec<Partner>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 検索結果のメタデータ
pub struct Meta {
    /// 総項目数
    pub total_count: u32,
    /// 総ページ数
    pub total_pages: u32,
    /// 現ページ
    pub current_page: String,
    /// 1ページあたりの項目数
    pub per_page: String,
}



#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
/// 取引先
pub struct Partner {
    /// 取引先ID
    pub id: String,
    /// 顧客コード
    pub code: Option<String>,
    /// 名前
    pub name: String,
    /// 名前（カナ）
    pub name_kana: Option<String>,
    /// 敬称
    pub name_suffix: String,
    /// メモ
    pub memo: Option<String>,
    /// 部門
    pub departments: Vec<Department>,
    /// 作成日時
    pub created_at: DateTime<FixedOffset>,
    /// 更新日時
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 部門
pub struct Department {
    /// 部門ID
    pub id: String,
    /// 郵便番号
    pub zip: Option<String>,
    /// 電話番号
    pub tel: Option<String>,
    /// 都道府県
    pub prefecture: String,
    /// 住所1
    pub address1: Option<String>,
    /// 住所2
    pub address2: Option<String>,
    /// 担当者氏名
    pub person_name: Option<String>,
    /// 担当者役職
    pub person_title: Option<String>,
    /// 部門名
    pub name: Option<String>,
    /// メールアドレス
    pub email: Option<String>,
    /// ccメールアドレス
    pub cc_emails: Option<String>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 取引先作成用リクエストデータ
pub struct NewPartner {
    /// 顧客コード
    pub code: Option<String>,
    /// 名前
    pub name: String,
    /// 名前（カナ）
    pub name_kana: Option<String>,
    /// 敬称
    pub name_suffix: Option<String>,
    /// メモ
    pub memo: Option<String>,
    /// 郵便番号
    pub zip: Option<String>,
    /// 電話番号
    pub tel: Option<String>,
    /// 都道府県
    pub prefecture: Option<String>,
    /// 住所1
    pub address1: Option<String>,
    /// 住所2
    pub address2: Option<String>,
    /// 担当者氏名
    pub person_name: Option<String>,
    /// 担当者役職
    pub person_title: Option<String>,
    /// 部門名
    pub department_name: Option<String>,
    /// メールアドレス
    pub email: Option<String>,
    /// ccメールアドレス
    pub cc_emails: Option<String>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 取引先更新用リクエストデータ
pub struct UpdatePartner {
    /// 顧客コード
    pub code: Option<String>,
    /// 名前
    pub name: Option<String>,
    /// 名前 (カナ)
    pub name_kana: Option<String>,
    /// 敬称
    pub name_suffix: Option<String>,
    /// メモ
    pub memo: Option<String>,
    /// 部門
    pub departments: Vec<UpdateDepartmentInfo>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 取引先更新用リクエストデータに付随する部門更新用リクエストデータ
pub struct UpdateDepartmentInfo {
    /// 部門ID。既存の部門を更新する際には必須です。
    pub id: Option<String>,
    /// 郵便番号
    pub zip: Option<String>,
    /// 電話番号
    pub tel: Option<String>,
    /// 都道府県
    pub prefecture: Option<String>,
    /// 住所1
    pub address1: Option<String>,
    /// 住所2
    pub address2: Option<String>,
    /// 担当者氏名
    pub person_name: Option<String>,
    /// 担当者役職
    pub person_title: Option<String>,
    /// 部門名
    pub name: Option<String>,
    /// メールアドレス
    pub email: Option<String>,
    /// ccメールアドレス
    pub cc_emails: Option<String>,
}


#[derive(Debug, Clone, PartialOrd, PartialEq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 請求書一覧
pub struct Billings {
    /// 結果のメタデータ
    pub meta: Meta,
    /// 請求書一覧
    pub billings: Vec<Billing>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Billing {
    /// 請求書ID e.g. "ABCDEFGHIJKLMNOPQRST123"
    pub id: String,
    pub user_id: String,
    /// 取引先ID e.g. "ABCDEFGHIJKLMNOPQRST789"
    pub partner_id: String,
    /// 部門ID e.g. "ABCDEFGHIJKLMNOPQRST012",
    pub department_id: String,
    /// 取引先名 e.g. "サンプル取引先"
    pub partner_name: String,
    /// 取引先敬称 e.g. "様"
    pub partner_name_suffix: String,
    /// 取引先詳細 e.g. "hogehoge"
    pub partner_detail: String,
    /// 担当者ID e.g. "ABCDEFGHIJKLMNOPQRST345"
    pub member_id: String,
    /// 担当者名 e.g. "member_name"
    pub member_name: String,
    /// 事業所名 e.g. "サンプル事業所"
    pub office_name: String,
    /// 事業所詳細 e.g. ""
    pub office_detail: String,
    /// 件名 e.g. "件名サンプル"
    pub title: String,
    /// 消費税 e.g. 80
    pub excise_price: u32,
    /// 割引額 e.g. 0
    pub deduct_price: u32,
    /// 小計額 e.g. 1000
    pub subtotal: u32,
    /// メモ e.g. ""
    pub memo: String,
    /// 支払条件 e.g. ""
    pub payment_condition: String,
    /// 合計額 e.g. 1080
    pub total_price: u32,
    // /// 請求日 e.g. "2015/10/31"
    // pub billing_date: Date<FixedOffset>,
    // /// 支払い期日 e.g. "2015/11/30"
    // pub due_date: Date<FixedOffset>,
    // /// 売上日 e.g. "2015/10/31"
    // pub sales_date: Date<FixedOffset>,
    /// 作成日時 e.g. "2015/10/31T00:00:00.000+09:00"
    pub created_at: DateTime<FixedOffset>,
    /// 更新日時 e.g. "2015/10/31T00:00:00.000+09:00"
    pub updated_at: DateTime<FixedOffset>,
    /// 請求番号 e.g. "1"
    pub billing_number: String,
    /// 備考 e.g. ""
    pub note: String,
    /// 文書名 e.g. ""
    pub document_name: String,
    /// タグ
    pub tags: Vec<String>,
    /// 状態
    pub status: Vec<Status>,
    /// 品目
    pub items: Vec<BillingItem>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 請求書各種状況
pub struct Status {
    /// 郵送状況 e.g. "未郵送"
    pub posting: String,
    /// メール状況 e.g. "未送信"
    pub email: String,
    /// ダウンロード状況 e.g. ""
    pub download: String,
    /// 支払い 状況 e.g. "未設定"
    pub payment: String,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
/// 品目
pub struct BillingItem {
    /// 品目ID e.g. "ABCDEFGHIJKLMNOPQRST012"
    pub id: String,
    /// コード e.g. "ITEM-001"
    pub code: String,
    /// 品名 e.g. "商品A"
    pub name: String,
    /// 詳細 e.g. ""
    pub detail: String,
    /// 数量 e.g. 1
    pub quantity: u32,
    /// 単価 e.g. 1000
    pub unit_price: u32,
    /// 単位 e.g. "個"
    pub unit: String,
    /// 金額 e.g. 1000
    pub price: u32,
    /// 表示順 e.g. 0
    pub display_order: u32,
    /// 課税対象 e.g. true
    pub excise: bool,
    /// 作成日時 "2015/10/31T00:00:00.000+09:00"
    pub created_at: DateTime<FixedOffset>,
    /// 更新日時 "2015/10/31T00:00:00.000+09:00"
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug)]
pub struct BillingPdf(pub(crate) reqwest::Response);
impl ::std::io::Read for BillingPdf {
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        self.0.read(buf)
    }
}


#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
/// 請求書検索の結果
pub struct BillingQueryResponse {
    /// メタデータ
    pub meta: BillingQueryMeta,
    /// 検索結果請求書
    pub billings: Vec<Billing>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
/// 請求書検索結果のメタデータ
pub struct BillingQueryMeta {
    /// 総項目数
    pub total_count: u32,
    /// 総ページ数
    pub total_pages: u32,
    /// 現ページ
    pub current_page: u32,
    /// 1ページあたりの項目数
    pub per_page: u32,
    /// 条件
    pub condition: Condition,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
/// 検索条件
pub struct Condition {
    /// 検索文字列 e.g. "hoge"
    pub query: String,
    /// 期間絞込対象 e.g. "created_at"
    pub range_key: String,
    /// 期間開始日 e.g. "2015-10-01"
    pub from: NaiveDate,
    /// 期間終了日 "2015-10-31"
    pub to: NaiveDate,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 請求書作成用リクエストデータ
pub struct NewBilling {
    /// 部門ID
    pub department_id: String,
    /// 件名
    pub title: Option<String>,
    /// 請求書番号
    pub billing_number: Option<String>,
    /// 振込先
    pub payment_condition: Option<String>,
    /// 備考
    pub note: Option<String>,
    /// 請求日
    pub billing_date: Option<NaiveDate>,
    /// お支払期限
    pub due_date: Option<NaiveDate>,
    /// 売上計上日
    pub sales_date: Option<NaiveDate>,
    /// メモ
    pub memo: Option<String>,
    /// 帳票名
    pub document_name: Option<String>,
    /// タグ。カンマ区切り文字列で記載
    pub tags: Option<String>,
    /// 品目
    pub items: Vec<NewBillingItem>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 請求書作成用リクエストデータに付随する品目データ
pub struct NewBillingItem {
    /// 品目ID。IDがあれば既存の品目の更新を、IDがなければ追加を意味する
    pub id: Option<String>,
    /// 名前
    pub name: Option<String>,
    /// コード
    pub code: Option<String>,
    /// 詳細
    pub detail: Option<String>,
    /// 数量
    pub quantity: Option<String>,
    /// 単価
    pub unit_price: Option<String>,
    /// 単位
    pub unit: Option<String>,
    /// 税対象
    pub excise: bool,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 請求書更新用リクエストデータ
pub struct UpdateBilling {
    /// 部門ID
    pub department_id: String,
    /// 件名
    pub title: Option<String>,
    /// 請求書番号
    pub billing_number: Option<String>,
    /// 振込先
    pub payment_condition: Option<String>,
    /// 備考
    pub note: Option<String>,
    /// 請求日
    pub billing_date: Option<NaiveDate>,
    /// お支払期限
    pub due_date: Option<NaiveDate>,
    /// 売上計上日
    pub sales_date: Option<NaiveDate>,
    /// メモ
    pub memo: Option<String>,
    /// 帳票名
    pub document_name: Option<String>,
    /// タグ。カンマ区切り文字列で記載
    pub tags: Option<String>,
    /// 品目
    pub items: Vec<UpdateBillingItem>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 請求書更新用リクエストデータに付随する品目更新用データ
pub struct UpdateBillingItem {
    /// 品目Id
    pub id: Option<String>,
    /// 名前
    pub name: Option<String>,
    /// コード
    pub code: Option<String>,
    /// 詳細
    pub detail: Option<String>,
    /// 数量
    pub quantity: Option<String>,
    /// 単価
    pub unit_price: Option<String>,
    /// 単位
    pub unit: Option<u32>,
    /// 税対象
    pub excise: bool,
    /// 削除するならtrue
    pub _destroy: bool,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 品目一覧
pub struct Items {
    /// メタデータ
    pub meta: Meta,
    /// 品目一覧
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Item {
    /// 品目ID e.g. "ABCDEFGHIJKLMNOPQRST012"
    pub id: String,
    /// コード e.g. "ITEM-001"
    pub code: String,
    /// 名前 e.g. "商品A"
    pub name: String,
    /// 詳細 e.g. ""
    pub detail: String,
    /// 数量 e.g. 1
    pub quantity: u32,
    /// 単価 e.g. 1000
    pub unit_price: u32,
    /// 単位 e.g. "個"
    pub unit: String,
    /// 金額 e.g. 1000
    pub price: u32,
    /// 課税対象 e.g. true
    pub excise: bool,
    /// 作成日時 e.g. "2015/10/31T00:00:00.000+09:00"
    pub created_at: DateTime<FixedOffset>,
    /// 更新日時 e.g. "2015/10/31T00:00:00.000+09:00"
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 品目作成用のリクエストデータ
pub struct NewItem {
    /// 名前
    pub name: Option<String>,
    /// 品目コード
    pub code: Option<String>,
    /// 詳細
    pub detail: Option<String>,
    /// 単価
    pub unit_price: Option<u32>,
    /// 単位
    pub unit: Option<String>,
    /// 数量
    pub quantity: Option<u32>,
    /// 消費税を計算するか
    pub excise: Option<bool>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 品目更新用のリクエストデータ
pub struct UpdateItem {
    /// 名前
    pub name: Option<String>,
    /// 品目コード
    pub code: Option<String>,
    /// 詳細
    pub detail: Option<String>,
    /// 単価
    pub unit_price: Option<u32>,
    /// 単位
    pub unit: Option<String>,
    /// 数量
    pub quantity: Option<u32>,
    /// 消費税を計算するか
    pub excise: Option<bool>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 送付履歴一覧
pub struct SentHistories {
    /// メタデータ
    pub meta: Meta,
    /// 送付履歴
    pub sent_history_list: Vec<SentHistory>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
#[derive(Serialize, Deserialize)]
/// 送付データ
pub struct SentHistory {
    /// e.g. "ABCDEFGHIJKLMNOP"
    pub operator_id: String,
    /// e.g. "メール"
    #[serde(rename = "type")]
    pub type_: String,
    /// e.g. "請求書"
    pub document_type: String,
    /// e.g. "ABCDEFGHIJKLMNOP"
    pub document_id: String,
    /// e.g. ""
    pub from: String,
    /// e.g. "sample@moneyforward.co.jp"
    pub to: String,
    /// e.g. ""
    pub cc: String,
    /// e.g. "2015-05-15T11:40:44.000+09:00"
    pub sent_at: String,
}
