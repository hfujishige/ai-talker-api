#!/bin/zsh

# ==============================================================================
# PostgreSQL メジャーバージョンアップ マイグレーションスクリプト
# PG 17 (既存データ) → PG 18.2-alpine3.22
#
# 方式: pg_dumpall による論理バックアップ → 新バージョンへリストア
# ==============================================================================

set -euo pipefail

# 設定 (pgsql_container.sh と合わせる)
CONTAINER_NAME="aitalker-postgres"
POSTGRES_USER="api_user_rw"
POSTGRES_PASSWORD="HE4ycm8uCER3"
POSTGRES_PORT=25432
VOLUME_PATH="$(pwd)/container_volumes/postgres_data"
DUMP_FILE="$(pwd)/container_volumes/pg17_dumpall.sql"

OLD_IMAGE="docker.io/library/postgres:17"
NEW_IMAGE="docker.io/library/postgres:18.2-alpine3.22"
TEMP_CONTAINER="aitalker-postgres-migrate"

echo "========================================"
echo " PostgreSQL Migration: 17 → 18.2"
echo "========================================"

# -----------------------------------------------
# 事前チェック
# -----------------------------------------------
if [[ ! -d "$VOLUME_PATH" ]]; then
  echo "ERROR: データディレクトリが見つかりません: $VOLUME_PATH"
  exit 1
fi

PG_VERSION_FILE="$VOLUME_PATH/PG_VERSION"
if [[ ! -f "$PG_VERSION_FILE" ]]; then
  echo "ERROR: PG_VERSION ファイルが見つかりません"
  exit 1
fi

CURRENT_VERSION=$(cat "$PG_VERSION_FILE")
if [[ "$CURRENT_VERSION" != "17" ]]; then
  echo "ERROR: 既存データが PG $CURRENT_VERSION です (PG 17 を期待)"
  exit 1
fi

echo "✓ 既存データ: PostgreSQL $CURRENT_VERSION"

# 既存コンテナが動作中なら停止
if podman container exists $CONTAINER_NAME 2>/dev/null; then
  echo "既存コンテナ '$CONTAINER_NAME' を停止・削除します..."
  podman stop $CONTAINER_NAME 2>/dev/null || true
  podman rm $CONTAINER_NAME 2>/dev/null || true
fi

# -----------------------------------------------
# Step 1: PG 17 コンテナで既存データからダンプ取得
# -----------------------------------------------
echo ""
echo "[Step 1/5] PG 17 コンテナを一時起動してダンプを取得..."

podman run -d \
  --name $TEMP_CONTAINER \
  -e POSTGRES_USER=$POSTGRES_USER \
  -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
  -v $VOLUME_PATH:/var/lib/postgresql/data \
  $OLD_IMAGE

# PostgreSQL が起動するまで待機
echo "  PostgreSQL 起動待ち..."
for i in {1..30}; do
  if podman exec $TEMP_CONTAINER pg_isready -U $POSTGRES_USER >/dev/null 2>&1; then
    echo "  ✓ PostgreSQL 17 起動完了"
    break
  fi
  if [[ $i -eq 30 ]]; then
    echo "ERROR: PostgreSQL 17 が起動しませんでした"
    podman rm -f $TEMP_CONTAINER
    exit 1
  fi
  sleep 1
done

echo "  pg_dumpall 実行中..."
podman exec $TEMP_CONTAINER pg_dumpall -U $POSTGRES_USER > "$DUMP_FILE"
DUMP_SIZE=$(wc -c < "$DUMP_FILE" | tr -d ' ')
echo "  ✓ ダンプ完了: $DUMP_FILE ($DUMP_SIZE bytes)"

# 一時コンテナ停止・削除
podman stop $TEMP_CONTAINER
podman rm $TEMP_CONTAINER
echo "  ✓ 一時コンテナを削除"

# -----------------------------------------------
# Step 2: 旧データディレクトリをバックアップ
# -----------------------------------------------
echo ""
echo "[Step 2/5] 旧データディレクトリをバックアップ..."

BACKUP_PATH="${VOLUME_PATH}_pg17_backup"
if [[ -d "$BACKUP_PATH" ]]; then
  echo "  既存バックアップを削除: $BACKUP_PATH"
  rm -rf "$BACKUP_PATH"
fi

mv "$VOLUME_PATH" "$BACKUP_PATH"
echo "  ✓ バックアップ: $BACKUP_PATH"

# -----------------------------------------------
# Step 3: PG 18.2 コンテナを新規起動
# -----------------------------------------------
echo ""
echo "[Step 3/5] PG 18.2 コンテナを新規起動..."
echo "  NOTE: PG 18+ ではマウントポイントが /var/lib/postgresql に変更されています"

mkdir -p "$VOLUME_PATH"

podman run -d \
  --name $CONTAINER_NAME \
  -e POSTGRES_USER=$POSTGRES_USER \
  -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
  -p $POSTGRES_PORT:5432 \
  -v $VOLUME_PATH:/var/lib/postgresql \
  $NEW_IMAGE

# PostgreSQL が起動するまで待機
echo "  PostgreSQL 起動待ち..."
for i in {1..30}; do
  if podman exec $CONTAINER_NAME pg_isready -U $POSTGRES_USER >/dev/null 2>&1; then
    echo "  ✓ PostgreSQL 18.2 起動完了"
    break
  fi
  if [[ $i -eq 30 ]]; then
    echo "ERROR: PostgreSQL 18.2 が起動しませんでした"
    exit 1
  fi
  sleep 1
done

# -----------------------------------------------
# Step 4: ダンプをリストア
# -----------------------------------------------
echo ""
echo "[Step 4/5] ダンプをリストアしています..."

podman exec -i $CONTAINER_NAME psql -U $POSTGRES_USER -d postgres < "$DUMP_FILE"
echo "  ✓ リストア完了"

# -----------------------------------------------
# Step 5: バージョン確認
# -----------------------------------------------
echo ""
echo "[Step 5/5] バージョン確認..."

NEW_VERSION=$(podman exec $CONTAINER_NAME psql -U $POSTGRES_USER -d postgres -tAc "SELECT version();")
echo "  $NEW_VERSION"

PG_VERSION_CHECK=$(cat "$VOLUME_PATH/PG_VERSION")
echo "  PG_VERSION ファイル: $PG_VERSION_CHECK"

# -----------------------------------------------
# 完了
# -----------------------------------------------
echo ""
echo "========================================"
echo " マイグレーション完了!"
echo "========================================"
echo ""
echo "バックアップディレクトリ: $BACKUP_PATH"
echo "ダンプファイル: $DUMP_FILE"
echo ""
echo "動作確認後、不要になったら以下を削除してください:"
echo "  rm -rf $BACKUP_PATH"
echo "  rm $DUMP_FILE"
