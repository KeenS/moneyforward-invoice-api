# トークンの取得
[公式ドキュメント](https://github.com/moneyforward/invoice-api-doc#%E8%AA%8D%E8%A8%BC%E3%81%AB%E3%81%A4%E3%81%84%E3%81%A6)に従ってOAuthのアクセストークンを取得します。

MoneyforwardがhttpsのコールバックURLしか認めていないため、HTTPSサーバを立てましょう

## HTTPSサーバの準備
### 証明書の準備
もし証明書を持っていないなら、自分で仮の認証局を立てて証明書を発行しましょう。
最終的に PKCS#12 秘密鍵があれば十分です。

* [プライベート認証局の証明書、サーバー証明書、クライアント証明書の作成方法について | レンタルサーバー・自宅サーバー設定・構築のヒント](https://server-setting.info/centos/private-ca-cert.html)

などを参考に準備して下さい（説明が丁寧なので記事が長いですが、「思ったよりは」簡単に作成できます）

### コールバックサーバの準備
もし用意できているならばそれを使いましょう。
ひとまず開発用にトークンだけ取得できればいいのであれば、get_token.rsに簡易サーバを用意してあるのでそれを使いましょう。

## moneyforwardへのクライアントの登録
先程の公式ドキュメントに従ってクライアントを登録します。
コールバックURLはget_token.rsを使うのであれば`https://localhost:3000/cb`を登録しましょう。

## トークンの取得
通常の取得方法は公式ドキュメントに従って下さい。get_token.rsを使う場合は以下のフローに従います。

``` console
$ export MF_INVOICE_CLIENT_ID=your_id
$ export MF_INVOICE_CLIENT_SECRET=your_secret
$ cargo run --example get_token your_identity_file your_password
Open this URL in your browser:
// 以下に認可URLが表示されるのでそこをブラウザで開く。あとはブラウザで認可する
https://invoice.moneyforward.com/oauth/authorize?client_id=your_id&scope=write&response_type=code&redirect_uri=https%3A%2F%2Flocalhost%3A3000%2Fcb&state=1234

// 認可が終わると以下に情報が表示される
MF returned the following code:
some_code

MF returned the following state:
1234

// 下記のaccess_tokenがアクセストークン
MF returned the following token:
Ok(Token { token_type: "bearer", access_token: "your_token", scopes: [], expires_in: Some(2592000), refresh_token: Some("your refresh token") })


```

これでアクセストークンが発行されました。

# Exampleの実行

トークンを取得できたら下記のようにExampleを実行できます

``` console
$ export MF_INVOICE_ACCESS_TOKEN=your_token
// 取引先操作のサンプルを実行したい場合
$ cargo run --example partner
```

