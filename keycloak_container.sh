#!/bin/zsh

# 設定
CONTAINER_NAME="aitalker-keycloak"
KEYCLOAK_USER="admin"
KEYCLOAK_PASSWORD="admin"
KEYCLOAK_PORT=8080
VOLUME_PATH="`pwd`/container_volumes/keycloak_data"  # 保存先（自由に変更OK）

# 引数チェック
if [[ $# -ne 1 ]]; then
  echo "Usage: $0 {start|status|stop}"
  exit 1
fi

# 起動前にディレクトリの存在確認
if [[ "$1" == "start" && ! -d "$VOLUME_PATH" ]]; then
  echo "Creating volume directory at $VOLUME_PATH..."
  mkdir -p "$VOLUME_PATH"
fi

case $1 in
  start)
    if podman container exists $CONTAINER_NAME; then
      echo "Starting existing container '$CONTAINER_NAME'..."
      podman start $CONTAINER_NAME
    else
      echo "Creating and starting new Keycloak container '$CONTAINER_NAME'..."

      # ボリュームディレクトリ作成
      mkdir -p $VOLUME_PATH

      podman run -d \
        --name $CONTAINER_NAME \
        -e KEYCLOAK_ADMIN=$KEYCLOAK_USER \
        -e KEYCLOAK_ADMIN_PASSWORD=$KEYCLOAK_PASSWORD \
        -p $KEYCLOAK_PORT:8080 \
        -v $VOLUME_PATH:/opt/keycloak/data \
        quay.io/keycloak/keycloak:latest \
        start-dev
    fi
    ;;

  status)
    if podman container exists $CONTAINER_NAME; then
      podman inspect -f 'Container "{{.Name}}" is {{.State.Status}}' $CONTAINER_NAME
    else
      echo "Container '$CONTAINER_NAME' does not exist."
    fi
    ;;

  stop)
    if podman container exists $CONTAINER_NAME; then
      echo "Stopping container '$CONTAINER_NAME'..."
      podman stop $CONTAINER_NAME
    else
      echo "Container '$CONTAINER_NAME' does not exist."
    fi
    ;;

  *)
    echo "Invalid argument: $1"
    echo "Usage: $0 {start|status|stop}"
    exit 1
    ;;
esac