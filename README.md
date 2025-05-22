# Sekigae3
座席最適化システムすね  
ユーザーに座りたい席のアンケートをとり、座席配置を最適化します

## memo
- [ ] セッションidはtimestamp入れる

## api
- POST `/api/create`  
  席替えセッションを作成します  
  
  リクエスト内容
  - 座席形状`[[bool..]..]`
  - ユーザーセット`[(number, ?name)..]`
  
  レスポンス内容
  - id`<id>`

- GET `/<id>` 
  フォーラム画面です  
  管理者はアンケートの締め切りと席替えの実行
  ユーザーはアンケートを入力することができます

- DELETE `/api/<id>/del`
  席替えを削除します

- GET `/api/<id>/sekigae`
  席替え結果を取得します

- GET `/api/<id>/info`
  席替えの情報を取得します
  設定など
  - 重みが調整可能か

- GET `/api/<id>/user/list`
  ユーザーの番号リストを取得します

- GET `/api/<id>/user/<number>/get`
  ユーザーの情報(希望席など)を取得します

- POST `/api/<id>/user/<number>/set`
  ユーザーの情報を指定します
  
  リクエスト内容
  - 名前
  - 座席指定